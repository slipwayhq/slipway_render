use std::cell::RefCell;

use image::{ImageBuffer, Rgba};
use std::fmt;

use crate::{errors::RenderError, Rect, Size};

fn render(
    target: &dyn Layoutable,
    path: &LayoutPath,
    available_size: Size,
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<(), RenderError> {
    target.measure(path, available_size)?;
    target.arrange(
        path,
        Rect::new(0., 0., available_size.width, available_size.height),
    )?;
    target.draw(path, image)?;
    Ok(())
}

pub(super) trait Layoutable {
    fn measure(&self, path: &LayoutPath, available_size: Size) -> Result<(), RenderError> {
        if available_size.is_invalid() {
            return Err(RenderError::MeasureError {
                path: path.clone(),
                message: format!("available size {} is not valid", available_size),
            });
        }

        let layout_data = self.layout_data();

        {
            let data = layout_data.borrow();
            if data.previous_measure.eq(&available_size) {
                return Ok(());
            }
        }

        let desired_size = self.measure_override(path, available_size);

        if desired_size.is_invalid() {
            return Err(RenderError::MeasureError {
                path: path.clone(),
                message: format!("desired size {} is not valid", desired_size),
            });
        }

        let mut data_mut = layout_data.borrow_mut();
        data_mut.desired_size = Some(desired_size);
        data_mut.previous_measure = available_size;

        Ok(())
    }

    fn arrange(&self, path: &LayoutPath, final_rect: Rect) -> Result<(), RenderError> {
        if final_rect.is_invalid() {
            return Err(RenderError::ArrangeError {
                path: path.clone(),
                message: format!("final rect {} is not valid.", final_rect),
            });
        }

        let layout_data = self.layout_data();

        {
            let data = layout_data.borrow();
            if data.desired_size.is_none() {
                return Err(RenderError::ArrangeError {
                    path: path.clone(),
                    message: "desired size not set.".to_string(),
                });
            }

            if data.previous_arrange.eq(&final_rect) {
                return Ok(());
            }
        }

        let actual_rect = self.arrange_override(path, final_rect);

        if actual_rect.is_invalid() {
            return Err(RenderError::MeasureError {
                path: path.clone(),
                message: format!("actual rect {} is not valid", actual_rect),
            });
        }

        let mut data_mut = layout_data.borrow_mut();
        data_mut.actual_rect = Some(actual_rect);
        data_mut.previous_arrange = final_rect;

        Ok(())
    }

    fn draw(
        &self,
        path: &LayoutPath,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), RenderError> {
        let layout_data = self.layout_data();
        let data = layout_data.borrow();

        let Some(actual_rect) = data.actual_rect else {
            return Err(RenderError::ArrangeError {
                path: path.clone(),
                message: "actual rect not set.".to_string(),
            });
        };

        if actual_rect.is_invalid() {
            return Err(RenderError::DrawError {
                path: path.clone(),
                message: format!("actual rect {} is not valid.", actual_rect),
            });
        }

        let temp_width = actual_rect.width.ceil() as u32;
        let temp_height = actual_rect.height.ceil() as u32;
        let mut temp_image = ImageBuffer::new(temp_width, temp_height);

        self.draw_override(
            path,
            Rect::new(0., 0., actual_rect.width, actual_rect.height),
            &mut temp_image,
        );

        // copy temp_image to image in position actual_rect
        for x in 0..temp_width {
            for y in 0..temp_height {
                let pixel = temp_image.get_pixel(x, y);
                image.put_pixel(
                    (actual_rect.x + x as f32) as u32,
                    (actual_rect.y + y as f32) as u32,
                    *pixel,
                );
            }
        }

        Ok(())
    }

    /// Returns the desired size of the element.
    fn measure_override(&self, path: &LayoutPath, _available_size: Size) -> Size {
        unimplemented!("measure_override not implemented for {}", path.clone());
    }

    /// Returns the actual rect used.
    fn arrange_override(&self, path: &LayoutPath, _final_rect: Rect) -> Rect {
        unimplemented!("arrange_override not implemented for {}", path.clone());
    }

    /// Returns the image bytes.
    fn draw_override(
        &self,
        path: &LayoutPath,
        _rect: Rect,
        _image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) {
        unimplemented!("draw_override not implemented for {}", path.clone());
    }

    fn layout_data(&self) -> &RefCell<LayoutData>;
}

// Implement Layoutable for Box<T> where T: Layoutable
impl<T: Layoutable> Layoutable for Box<T> {
    fn measure(&self, path: &LayoutPath, available_size: Size) -> Result<(), RenderError> {
        self.as_ref().measure(path, available_size)
    }

    fn arrange(&self, path: &LayoutPath, final_rect: Rect) -> Result<(), RenderError> {
        self.as_ref().arrange(path, final_rect)
    }

    fn draw(
        &self,
        path: &LayoutPath,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), RenderError> {
        self.as_ref().draw(path, image)
    }

    fn measure_override(&self, path: &LayoutPath, available_size: Size) -> Size {
        self.as_ref().measure_override(path, available_size)
    }

    fn arrange_override(&self, path: &LayoutPath, final_rect: Rect) -> Rect {
        self.as_ref().arrange_override(path, final_rect)
    }

    fn draw_override(
        &self,
        path: &LayoutPath,
        rect: Rect,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) {
        self.as_ref().draw_override(path, rect, image)
    }

    fn layout_data(&self) -> &RefCell<LayoutData> {
        self.as_ref().layout_data()
    }
}

#[derive(Default, Debug)]
pub(super) struct LayoutData {
    pub desired_size: Option<Size>,
    pub actual_rect: Option<Rect>,
    pub previous_measure: Size,
    pub previous_arrange: Rect,
}

#[derive(Clone, Debug)]
pub struct LayoutPath {
    path: Vec<String>,
}

impl fmt::Display for LayoutPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.join(" > "))
    }
}
