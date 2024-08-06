use std::{cell::RefCell, rc::Rc};

use std::fmt;

use imageproc::rect::Rect;

use crate::masked_image::MaskedImage;
use crate::rect::MoveableFromOrigin;
use crate::{
    errors::RenderError, host_config::generated::HostConfig, masked_image::SlipwayCanvas,
    size::Size,
};

pub(super) trait Layoutable: HasLayoutData {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Base/Layout/Layoutable.cs#L356

    fn measure(&self, context: &LayoutContext, available_size: Size) -> Result<Size, RenderError> {
        let layout_data = self.layout_data();

        {
            let data = layout_data.borrow();
            if let Some(measure_result) = &data.measure_result {
                if measure_result.previous_measure.eq(&available_size) {
                    return Ok(measure_result.desired_size);
                }
            }
        }

        // We don't constrain the desired size to the available size here, because we want
        // to allow elements with minHeight to be able to express that they want more space.
        let desired_size = self.measure_override(context, available_size)?;

        let mut data_mut = layout_data.borrow_mut();
        data_mut.measure_result = Some(MeasureResult {
            desired_size,
            previous_measure: available_size,
        });

        Ok(desired_size)
    }

    fn arrange(&self, context: &LayoutContext, final_rect: Rect) -> Result<Rect, RenderError> {
        let layout_data = self.layout_data();

        {
            let data = layout_data.borrow();

            let Some(_measure_result) = &data.measure_result else {
                return Err(RenderError::MeasureResultNotFound {
                    path: context.path.clone(),
                });
            };

            if let Some(arrange_result) = &data.arrange_result {
                if arrange_result.previous_arrange.eq(&final_rect) {
                    return Ok(arrange_result.actual_rect);
                }
            };
        }

        // We pass in a Rect rather than a Size, unlike in the Avalonia reference, because
        // we only have horizontal / vertical alignment information in the implementing
        // struct, not in the Layoutable trait. Therefore the result of `arrange_override`
        // needs to include x and y coordinates not just width and height.
        let final_rect_at_origin = Rect::at(0, 0).of_size(final_rect.width(), final_rect.height());
        let actual_rect_at_origin = self.arrange_override(context, final_rect_at_origin)?;
        let actual_rect = actual_rect_at_origin.move_from_origin_into(final_rect);

        let mut data_mut = layout_data.borrow_mut();
        data_mut.arrange_result = Some(ArrangeResult {
            actual_rect,
            previous_arrange: final_rect,
        });

        Ok(actual_rect)
    }

    fn draw(
        &self,
        context: &LayoutContext,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        let layout_data = self.layout_data();
        let data = layout_data.borrow();

        let Some(arrange_result) = &data.arrange_result else {
            return Err(RenderError::ArrangeResultNotFound {
                path: context.path.clone(),
            });
        };

        let (width, height) = image.dimensions();
        let image_rect = Rect::at(0, 0).of_size(width, height);
        let actual_rect = arrange_result.actual_rect;

        if image_rect.intersect(actual_rect).is_some() {
            self.draw_override(context, image)?;
        }

        Ok(())
    }

    /// Returns the desired size of the element.
    fn measure_override(
        &self,
        context: &LayoutContext,
        _available_size: Size,
    ) -> Result<Size, RenderError> {
        unimplemented!(
            "measure_override not implemented for {}",
            context.path.clone()
        );
    }

    /// Returns the actual rect used.
    fn arrange_override(
        &self,
        context: &LayoutContext,
        _final_size: Rect,
    ) -> Result<Rect, RenderError> {
        unimplemented!(
            "arrange_override not implemented for {}",
            context.path.clone()
        );
    }

    /// Returns the image bytes.
    fn draw_override(
        &self,
        context: &LayoutContext,
        _image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        unimplemented!("draw_override not implemented for {}", context.path.clone());
    }
}

// Implement Layoutable for Box<T> where T: Layoutable
impl<T: Layoutable> Layoutable for Box<T> {
    fn measure(&self, context: &LayoutContext, available_size: Size) -> Result<Size, RenderError> {
        self.as_ref().measure(context, available_size)
    }

    fn arrange(&self, context: &LayoutContext, final_rect: Rect) -> Result<Rect, RenderError> {
        self.as_ref().arrange(context, final_rect)
    }

    fn draw(
        &self,
        context: &LayoutContext,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        self.as_ref().draw(context, image)
    }

    fn measure_override(
        &self,
        context: &LayoutContext,
        available_size: Size,
    ) -> Result<Size, RenderError> {
        self.as_ref().measure_override(context, available_size)
    }

    fn arrange_override(
        &self,
        context: &LayoutContext,
        final_rect: Rect,
    ) -> Result<Rect, RenderError> {
        self.as_ref().arrange_override(context, final_rect)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        self.as_ref().draw_override(context, image)
    }
}

impl<T: HasLayoutData> HasLayoutData for Box<T> {
    fn layout_data(&self) -> &RefCell<LayoutData> {
        self.as_ref().layout_data()
    }
}

pub(super) trait HasLayoutData {
    fn layout_data(&self) -> &RefCell<LayoutData>;
    fn desired_size(&self) -> Size {
        self.layout_data()
            .borrow()
            .measure_result
            .as_ref()
            .expect("Element should have been measured")
            .desired_size
    }
    fn actual_rect(&self) -> Rect {
        self.layout_data()
            .borrow()
            .arrange_result
            .as_ref()
            .expect("Element should have been arranged")
            .actual_rect
    }
}

#[derive(Default, Debug, Clone)]
pub(super) struct LayoutData {
    pub measure_result: Option<MeasureResult>,
    pub arrange_result: Option<ArrangeResult>,
}

#[derive(Clone, Debug)]
pub(super) struct MeasureResult {
    pub desired_size: Size,
    previous_measure: Size,
}

#[derive(Clone, Debug)]
pub(super) struct ArrangeResult {
    pub actual_rect: Rect,
    previous_arrange: Rect,
}

pub(super) struct LayoutContext<'hc> {
    pub host_config: &'hc HostConfig,
    pub path: Rc<LayoutPath>,
}

impl<'hc> LayoutContext<'hc> {
    pub fn for_child_str(&self, child_name: &str) -> Self {
        self.for_child(child_name.to_string())
    }

    pub fn for_child(&self, child_name: String) -> Self {
        LayoutContext {
            host_config: self.host_config,
            path: Rc::new(LayoutPath {
                current: child_name,
                previous: Some(self.path.clone()),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LayoutPath {
    pub current: String,
    pub previous: Option<Rc<LayoutPath>>,
}

impl fmt::Display for LayoutPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.current)?;
        if let Some(previous) = self.previous.as_ref() {
            write!(f, ".{}", previous)?;
        }
        Ok(())
    }
}
