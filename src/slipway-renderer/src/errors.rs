use std::rc::Rc;

use image::ImageError;
use thiserror::Error;

use crate::layoutable::LayoutPath;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Measure failed for {path}:\n{message}")]
    Other {
        path: Rc<LayoutPath>,
        message: String,
    },

    #[error("Measure result not found for {path}")]
    MeasureResultNotFound { path: Rc<LayoutPath> },

    #[error("Arrange result not found for {path}")]
    ArrangeResultNotFound { path: Rc<LayoutPath> },

    #[error("The image reference count was not 1 after all rendering completed")]
    ImageReferenceCountNotOne,

    #[error("Image operation failed for {path}:\n{inner}")]
    ImageError {
        path: Rc<LayoutPath>,
        inner: ImageError,
    },
}
