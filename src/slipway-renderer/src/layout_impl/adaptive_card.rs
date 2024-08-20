use std::{cell::RefCell, rc::Rc};

use image::Rgba;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use taffy::{Dimension, Size, Style, TaffyTree};

use crate::{
    element_layout_data::ElementTaffyData,
    errors::RenderError,
    layout_context::LayoutContext,
    layoutable::Layoutable,
    masked_image::{MaskedImage, SlipwayCanvas},
    AdaptiveCard,
};

use super::{
    container::{container_draw_override, container_layout_override},
    NodeContext,
};

impl Layoutable for AdaptiveCard {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233
    // Reference: https://github.com/microsoft/AdaptiveCards/blob/728044c67510871445d23533fb9830ac57fbbf99/source/nodejs/adaptivecards/src/card-elements.ts#L7820-L7888

    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        let baseline_style = Style {
            size: Size {
                width: Dimension::Percent(1.),
                height: Dimension::Percent(1.),
            },
            ..baseline_style
        };

        let child_elements_context = context.for_child_str("body");
        let child_elements = self.body.as_deref().unwrap_or(&[]);

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
        // Fill the background with white.
        {
            let (width, height) = image.dimensions();
            let mut image_mut = image.borrow_mut();
            draw_filled_rect_mut(
                &mut *image_mut,
                Rect::at(0, 0).of_size(width, height),
                Rgba([255, 255, 255, 255]),
            );
        }

        let child_elements_context = context.for_child_str("body");
        let child_elements = self.body.as_deref().unwrap_or(&[]);

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
