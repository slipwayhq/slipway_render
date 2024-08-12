use std::{cell::RefCell, rc::Rc};

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
