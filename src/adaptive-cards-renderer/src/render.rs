use std::{cell::RefCell, rc::Rc};

use adaptive_cards::AdaptiveCard;
use adaptive_cards_host_config::HostConfig;
use image::RgbaImage;
use taffy::{AvailableSpace, TaffyTree};

use crate::{
    element_layout_data::ElementLayoutData,
    errors::{RenderError, TaffyErrorToRenderError},
    layout_context::LayoutContext,
    layout_impl::measure::{measure, NodeContext},
    layout_scratch::LayoutScratch,
    layoutable::Layoutable,
    masked_image::{Ejectable, MaskedImage},
    DebugMode,
};

const ROBOTO_TTF: &[u8] = include_bytes!("../../../fonts/Roboto.ttf");
const ROBOTO_MONO_TTF: &[u8] = include_bytes!("../../../fonts/RobotoMono.ttf");

pub fn render_from_str(
    target: &str,
    host_config: &HostConfig,
    width: u32,
    height: u32,
    debug_mode: DebugMode,
) -> Result<(RgbaImage, AdaptiveCard<ElementLayoutData>), RenderError> {
    let target = serde_json::from_str::<AdaptiveCard<ElementLayoutData>>(target).unwrap();
    let image = render(&target, host_config, width, height, debug_mode)?;
    Ok((image, target))
}

/// Renders an AdaptiveCard to an image.
/// This occurs in three parts:
/// - Layout: The tree of AdaptiveCard elements populate their layout data.
/// - Calculate: Taffy calculates the final layout.
/// - Draw: The AdaptiveCard elements draw themselves onto an image using the layout data.
pub fn render(
    target: &AdaptiveCard<ElementLayoutData>,
    host_config: &HostConfig,
    width: u32,
    height: u32,
    debug_mode: DebugMode,
) -> Result<RgbaImage, RenderError> {
    // Check for any host config issues that won't be picked up by deserialization.
    validate_host_config(host_config)?;

    // Create the context for the root element.
    let context = LayoutContext::new(host_config, debug_mode);

    // Create the Taffy tree, which will be populated in the layout pass.
    let mut tree: TaffyTree<NodeContext> = TaffyTree::new();

    // Layout the root element, which will recursively layout all descendants.
    let root = target.layout(&context, Default::default(), &mut tree)?;

    let swash_scale_context = swash::scale::ScaleContext::new();
    let parley_layout_context = parley::LayoutContext::new();
    let mut parley_font_context = parley::FontContext::new();
    parley_font_context
        .collection
        .register_fonts(ROBOTO_TTF.into());
    parley_font_context
        .collection
        .register_fonts(ROBOTO_MONO_TTF.into());

    let mut scratch = LayoutScratch::new(
        parley_layout_context,
        parley_font_context,
        swash_scale_context,
    );

    // Calculate the final layout of the tree.
    tree.compute_layout_with_measure(
        root,
        taffy::Size {
            width: AvailableSpace::Definite(width as f32),
            height: AvailableSpace::Definite(height as f32),
        },
        |known_dimensions, available_space, _node_id, node_context, _style| {
            measure(
                known_dimensions,
                available_space,
                node_context,
                &context,
                &mut scratch,
            )
        },
    )
    .err_context(&context)?;

    // Print the tree (will only be displayed if the user enables the appropriate logging level).
    super::print_tree::print_tree(&tree, root);

    // Create the image to draw onto.
    let image = Rc::new(RefCell::new(RgbaImage::new(width, height)));

    // Create the masked image for the root element, which will have a mask the size of the image.
    let masked_image = MaskedImage::from_image(image, debug_mode);

    // Draw the root element onto the image, which will recursively draw all descendants.
    target.draw(&context, &tree, masked_image.clone(), &mut scratch)?;

    // Eject the image from the masked image, returning an owned image.
    let image = masked_image.eject()?;

    Ok(image)
}

fn validate_host_config(host_config: &HostConfig) -> Result<(), RenderError> {
    // https://github.com/microsoft/AdaptiveCards/issues/1078
    if host_config.font_family.is_some() {
        println!("Warning: hostConfig.fontFamily is deprecated and not used by this renderer.");
    }
    if host_config.font_sizes.is_some() {
        println!("Warning: hostConfig.fontSizes is deprecated and not used by this renderer.");
    }
    if host_config.font_weights.is_some() {
        println!("Warning: hostConfig.fontWeights is deprecated and not used by this renderer.");
    }

    Ok(())
}
