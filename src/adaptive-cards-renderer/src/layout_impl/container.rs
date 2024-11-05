use std::{cell::RefCell, rc::Rc};

use adaptive_cards::Container;
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut};
use taffy::{Dimension, Size, Style, TaffyTree};

use crate::{
    element_layout_data::{ElementLayoutData, ElementTaffyData},
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::{ContainerStyleToConfig, StringToColor},
    layout_context::LayoutContext,
    layoutable::Layoutable,
    masked_image::MaskedImage,
    measure::NodeContext,
    utils::TaffyLayoutUtils,
};

use super::{
    container_shared::{container_draw_override, container_layout_override},
    utils::{get_margins_for_bleed, parse_dimension},
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
    ) -> Result<(), RenderError> {
        // Draw the container background, if necessary.
        draw_background(self, context, tree, taffy_data, &image)?;

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
            child_elements_context,
            child_elements,
        )
    }
}

fn draw_background(
    container: &Container<ElementLayoutData>,
    context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    taffy_data: &ElementTaffyData,
    image: &Rc<RefCell<MaskedImage>>,
) -> Result<(), RenderError> {
    let Some(style) = container.style else {
        // We don't need to check the inherited style because we can just
        // use the existing background color drawn by an ancestor.
        // We don't want to draw a border if the style isn't explicitly
        // specified on this container.
        return Ok(());
    };

    // Get the config for the style specified on the container.
    let style_config = context.host_config.container_styles.from(style);

    // Get our absolute rectangle.
    let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
    let rect = node_layout.absolute_rect(context);

    let mut image_mut = image.borrow_mut();

    // If the style has a background color, use it to draw a rectangle over our absolute rect.
    if let Some(background_color_str) = style_config.background_color.as_ref() {
        let background_color = background_color_str.to_color()?;
        draw_filled_rect_mut(&mut *image_mut, rect, background_color);
    }

    // Same for the boarder.
    // Technically, we shouldn't draw a border here:
    // https://github.com/microsoft/AdaptiveCards/blob/15418ce93b452dd0858415db40ddba05cd154c73/specs/features/Tables.md?plain=1#L65-L91
    // The border color property is technically, and unintuitively, supposed to be used
    // with the "gridStyle" property on a table to color the table grid lines.
    // We're going to deviate from the official Adaptive Cards behavior here to do the
    // intuitive thing and use the border color to draw borders.
    if let Some(border_color_str) = style_config.border_color.as_ref() {
        let border_color = border_color_str.to_color()?;
        draw_hollow_rect_mut(&mut *image_mut, rect, border_color);
    }

    Ok(())
}
