use std::{cell::RefCell, rc::Rc};

use adaptive_cards::Container;
use taffy::{Dimension, Size, Style, TaffyTree};

use crate::{
    element_layout_data::{ElementLayoutData, ElementTaffyData},
    errors::RenderError,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::Layoutable,
    masked_image::MaskedImage,
};

use super::{
    container_shared::{container_draw_override, container_layout_override},
    utils::{draw_background, get_margins_for_bleed, parse_dimension},
};

impl Layoutable for Container<ElementLayoutData> {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233
    // Reference: https://github.com/microsoft/AdaptiveCards/blob/728044c67510871445d23533fb9830ac57fbbf99/source/nodejs/adaptivecards/src/card-elements.ts#L7820-L7888

    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        // Parse the min height, if specified.
        let min_height = match &self.min_height {
            Some(min_height) => parse_dimension(min_height, context)?,
            None => Dimension::Auto,
        };

        // Create a mutable baseline style with the min height.
        let mut baseline_style = Style {
            min_size: Size {
                width: Dimension::Percent(1.),
                height: min_height,
            },
            ..baseline_style
        };

        // If the container is set to bleed, we need to add appropriate margins to the baseline style.
        if self.bleed() {
            let placement = self.layout_data.borrow().placement();
            baseline_style.margin = get_margins_for_bleed(&placement, context.host_config);
        }

        // Create the child context.
        let child_elements_context = context
            .for_child_str("items")
            .with_vertical_content_alignment(&self.vertical_content_alignment)
            .with_style(&self.style);

        // Get the child elements.
        let child_elements = &self.items;

        // Delegate to the shared container layout function.
        container_layout_override(
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
        // Draw the container background, if necessary.
        if let Some(style) = self.style {
            draw_background(style, context, tree, taffy_data, &image)?;
        }

        // Create the child context.
        let child_elements_context = context.for_child_str("items");

        // Get the child elements.
        let child_elements = &self.items;

        // Delegate to the shared container draw function.
        container_draw_override(
            context,
            tree,
            taffy_data,
            image,
            scratch,
            child_elements_context,
            child_elements,
        )
    }
}
