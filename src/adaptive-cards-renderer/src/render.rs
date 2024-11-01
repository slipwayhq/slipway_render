use std::{cell::RefCell, rc::Rc};

use adaptive_cards_host_config::HostConfig;
use image::RgbaImage;
use taffy::{AvailableSpace, TaffyTree};

use crate::{
    adaptive_cards::AdaptiveCard,
    errors::{RenderError, TaffyErrorToRenderError},
    fonts::FontCache,
    layout_context::LayoutContext,
    layoutable::Layoutable,
    masked_image::{Ejectable, MaskedImage},
    measure::{measure_function, NodeContext},
    DebugMode,
};

pub fn render_from_str(
    target: &str,
    host_config: &HostConfig,
    width: u32,
    height: u32,
    debug_mode: DebugMode,
) -> Result<(RgbaImage, AdaptiveCard), RenderError> {
    let target = serde_json::from_str::<AdaptiveCard>(target).unwrap();
    let image = render(&target, host_config, width, height, debug_mode)?;
    Ok((image, target))
}

/// Renders an AdaptiveCard to an image.
/// This occurs in three parts:
/// - Layout: The tree of AdaptiveCard elements populate their layout data.
/// - Calculate: Taffy calculates the final layout.
/// - Draw: The AdaptiveCard elements draw themselves onto an image using the layout data.
pub fn render(
    target: &AdaptiveCard,
    host_config: &HostConfig,
    width: u32,
    height: u32,
    debug_mode: DebugMode,
) -> Result<RgbaImage, RenderError> {
    // Create the context for the root element.
    let context = LayoutContext::new(host_config, debug_mode);

    // Create the Taffy tree, which will be populated in the layout pass.
    let mut tree: TaffyTree<NodeContext> = TaffyTree::new();

    // Layout the root element, which will recursively layout all descendants.
    let root = target.layout(&context, Default::default(), &mut tree)?;

    let font_cache = FontCache::new();

    // Calculate the final layout of the tree.
    tree.compute_layout_with_measure(
        root,
        taffy::Size {
            width: AvailableSpace::Definite(width as f32),
            height: AvailableSpace::Definite(height as f32),
        },
        // This closure is a FnMut closure and can be used to borrow external context for the duration of layout
        // For example, you may wish to borrow a global font registry and pass it into your text measuring function
        |known_dimensions, available_space, _node_id, node_context, _style| {
            measure_function(known_dimensions, available_space, node_context, &font_cache)
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
    target.draw(&context, &tree, masked_image.clone())?;

    // Eject the image from the masked image, returning an owned image.
    let image = masked_image.eject()?;

    Ok(image)
}
