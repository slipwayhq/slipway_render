use std::{cell::RefCell, rc::Rc};

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
