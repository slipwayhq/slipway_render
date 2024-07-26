use image::{ImageBuffer, Rgba};

use crate::{
    errors::RenderError,
    layoutable::{LayoutPath, Layoutable},
    AdaptiveCard, Element, Rect, Size,
};

impl Layoutable for AdaptiveCard {
    fn measure_override(
        &self,
        path: &LayoutPath,
        available_size: Size,
    ) -> Result<Size, RenderError> {
        let mut size = Size::new(0.0, 0.0);
        if let Some(body) = &self.body {
            for element in body.iter() {
                let desired_size = element.as_layoutable().measure(path, available_size)?;
                size.width = size.width.max(desired_size.width);
                size.height += desired_size.height;
            }
        }
        Ok(size)
    }

    fn arrange_override(&self, path: &LayoutPath, final_rect: Rect) -> Result<Rect, RenderError> {
        let mut y = final_rect.y;
        if let Some(body) = &self.body {
            for element in body.iter() {
                let desired_size = element
                    .as_layoutable()
                    .layout_data()
                    .borrow()
                    .measure_result
                    .as_ref()
                    .expect("Element should have been measured before arranging")
                    .desired_size;

                let element_rect =
                    Rect::new(final_rect.x, y, final_rect.width, desired_size.height);
                let actual_rect = element.as_layoutable().arrange(path, element_rect)?;
                y += desired_size.height;
            }
        }
        Ok(final_rect)
    }
}
