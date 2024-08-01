use image::{GenericImage, ImageBuffer, Rgba, SubImage};

use crate::{
    errors::RenderError,
    layoutable::{LayoutContext, Layoutable},
    AdaptiveCard, Element, Rect, Size, SlipwayImage,
};

use super::ValidSpacing;

impl Layoutable for AdaptiveCard {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233

    fn measure_override(
        &self,
        context: &LayoutContext,
        available_size: Size,
    ) -> Result<Size, RenderError> {
        let body_context = context.for_child_str("body");

        let mut size = Size::new(0, 0);
        if let Some(body) = &self.body {
            for (i, element) in body.iter().enumerate() {
                let spacing = context.host_config.spacing.from(element.as_element());
                let element_context = body_context.for_child(i.to_string());
                let desired_size = element
                    .as_layoutable()
                    .measure(&element_context, available_size)?;
                size.width = size.width.max(desired_size.width);
                size.height += desired_size.height + spacing;
            }
        }

        Ok(size)
    }

    fn arrange_override(
        &self,
        context: &LayoutContext,
        final_rect: Rect,
    ) -> Result<Rect, RenderError> {
        let body_context = context.for_child_str("body");

        let mut child_rect = final_rect;
        let mut previous_child_height = 0;

        if let Some(body) = &self.body {
            for (i, element) in body.iter().enumerate() {
                let element_context = body_context.for_child(i.to_string());

                let spacing = context.host_config.spacing.from(element.as_element());

                let desired_size = element
                    .as_layoutable()
                    .layout_data()
                    .borrow()
                    .measure_result
                    .as_ref()
                    .expect("Element should have been measured before arranging")
                    .desired_size;

                child_rect.y += previous_child_height;
                child_rect.height = desired_size.height;
                child_rect.width = final_rect.width.max(desired_size.width);

                previous_child_height = desired_size.height + spacing;

                element
                    .as_layoutable()
                    .arrange(&element_context, child_rect)?;
            }
        }
        Ok(final_rect)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        image: &mut SubImage<&mut SlipwayImage>,
    ) -> Result<(), RenderError> {
        let body_context = context.for_child_str("body");

        if let Some(body) = &self.body {
            for (i, element) in body.iter().enumerate() {
                let element_context = body_context.for_child(i.to_string());

                let element_rect = element
                    .as_layoutable()
                    .layout_data()
                    .borrow()
                    .arrange_result
                    .as_ref()
                    .expect("Element should have been arranged before drawing")
                    .actual_rect;

                let mut sub_image = image.sub_image(
                    element_rect.x,
                    element_rect.y,
                    element_rect.width,
                    element_rect.height,
                );

                element
                    .as_layoutable()
                    .draw(&element_context, &mut sub_image)?;
            }
        }
        Ok(())
    }
}
