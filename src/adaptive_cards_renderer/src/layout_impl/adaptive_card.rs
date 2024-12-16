use std::{cell::RefCell, rc::Rc};

use adaptive_cards::AdaptiveCard;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use taffy::{Dimension, Size, Style, TaffyTree};

use crate::{
    element_layout_data::{ElementLayoutData, ElementTaffyData},
    errors::RenderError,
    host_config_utils::StringToColor,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::Layoutable,
    masked_image::{MaskedImage, SlipwayCanvas},
};

use super::container_shared::{container_draw_override, container_layout_override_inner};

impl Layoutable for AdaptiveCard<ElementLayoutData> {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233
    // Reference: https://github.com/microsoft/AdaptiveCards/blob/728044c67510871445d23533fb9830ac57fbbf99/source/nodejs/adaptivecards/src/card-elements.ts#L7820-L7888

    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        // The root AdaptiveCard element always has a size of 100% width and height.
        let baseline_style = Style {
            size: Size {
                width: Dimension::Percent(1.),
                height: Dimension::Percent(1.),
            },
            ..baseline_style
        };

        // An AdaptiveCard element behaves like a vertical container, with the child
        // elements stored in the `body` field.
        // Create the child context.
        let child_elements_context = context
            .for_child_str("body")
            .with_vertical_content_alignment(self.vertical_content_alignment);

        // Get the child elements.
        let child_elements = self.body.as_deref().unwrap_or(&[]);

        // Delegate to the shared container layout function.
        container_layout_override_inner(
            context,
            baseline_style,
            tree,
            child_elements_context,
            child_elements,
        )
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        // Draw the background of the AdaptiveCard.
        draw_background(context, &image)?;

        // Delegate to the shared container draw function.
        container_draw_override(
            self, context, tree, taffy_data, image, scratch, None, "body",
        )
    }
}

/// Draws the background of the AdaptiveCard.
fn draw_background(
    context: &LayoutContext,
    image: &Rc<RefCell<MaskedImage>>,
) -> Result<(), RenderError> {
    // The AdaptiveCard element just uses the default background color from
    // the host config, so fetch it.
    let background_color = context
        .host_config
        .container_styles
        .default
        .background_color
        .to_color()?;

    let (width, height) = image.dimensions();
    let mut image_mut = image.borrow_mut();

    // Draw the background color over the entire image.
    draw_filled_rect_mut(
        &mut *image_mut,
        Rect::at(0, 0).of_size(width, height),
        background_color,
    );

    Ok(())
}
