use imageproc::rect::Rect;
use std::{cell::RefCell, rc::Rc};
use taffy::Layout;

use crate::layout_context::LayoutContext;

/// Extracts the inner `T` value from an `Rc<RefCell<T>>` if the reference count is 1,
/// returning `None` otherwise.
pub(super) fn extract_from_rc_refcell<T>(shared: Rc<RefCell<T>>) -> Option<T> {
    // Attempt to unwrap the Rc, which should succeed if the reference count is 1
    match Rc::try_unwrap(shared) {
        Ok(ref_cell) => {
            // Now, take out the RgbaImage from the RefCell
            Some(ref_cell.into_inner())
        }
        Err(_) => {
            // If this fails, it means the reference count was not 1
            None
        }
    }
}

pub(super) trait ClampToU32 {
    fn clamp_to_u32(self) -> u32;
}

impl<T> ClampToU32 for T
where
    T: PartialOrd + From<u32> + TryInto<u32>,
{
    fn clamp_to_u32(self) -> u32 {
        let lower_bound: T = 0.into();
        let upper_bound: T = u32::MAX.into();

        if self < lower_bound {
            0
        } else if self > upper_bound {
            u32::MAX
        } else {
            self.try_into().unwrap_or(u32::MAX)
        }
    }
}

pub(super) trait ClampToI32 {
    fn clamp_to_i32(self) -> i32;
}

// impl<T> ClampToI32 for T
// where
//     T: PartialOrd + From<i32> + TryInto<i32>,
// {
//     fn clamp_to_i32(self) -> i32 {
//         let lower_bound: T = i32::MIN.into();
//         let upper_bound: T = i32::MAX.into();

//         if self < lower_bound {
//             i32::MIN
//         } else if self > upper_bound {
//             i32::MAX
//         } else {
//             self.try_into().unwrap_or(i32::MAX)
//         }
//     }
// }

impl ClampToI32 for u32 {
    fn clamp_to_i32(self) -> i32 {
        self.min(i32::MAX as u32) as i32
    }
}

/// A trait for easily getting the absolute rect of an element from the Taffy
/// Layout data using the element's context data.
pub(super) trait TaffyLayoutUtils {
    /// Gets the absolute rect of the element using the context data.
    fn absolute_rect(&self, context: &LayoutContext) -> Rect;
}

impl TaffyLayoutUtils for Layout {
    fn absolute_rect(&self, context: &LayoutContext) -> Rect {
        // The context already has the element's absolute origin set,
        // so we just need to use that and the element's size to get the absolute rect.
        Rect::at(
            context.current_origin.x as i32,
            context.current_origin.y as i32,
        )
        .of_size(
            self.size.width.max(1.) as u32,
            self.size.height.max(1.) as u32,
        )
    }
}
