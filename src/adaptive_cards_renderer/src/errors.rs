use std::rc::Rc;

use image::ImageError;
use thiserror::Error;

use crate::layout_context::{LayoutContext, LayoutPath};

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Measure failed for {path}:\n{message}")]
    Other {
        path: Rc<LayoutPath>,
        message: String,
    },

    #[error("Measure result not found for {path}")]
    TaffyDataNotFound { path: Rc<LayoutPath> },

    #[error("Layout error at {path}:\n{taffy_error:?}")]
    Taffy {
        path: Rc<LayoutPath>,
        taffy_error: taffy::TaffyError,
    },

    #[error("The image reference count was not 1 after all rendering completed")]
    ImageReferenceCountNotOne,

    #[error("Image operation failed for {path}:\n{inner}")]
    ImageError {
        path: Rc<LayoutPath>,
        inner: ImageError,
    },

    #[error("HostConfig error:\n{message}")]
    HostConfig { message: String },
}

pub(super) trait TaffyErrorToRenderError<T> {
    fn err_context(self, context: &LayoutContext) -> Result<T, RenderError>;
}

impl<T> TaffyErrorToRenderError<T> for taffy::TaffyResult<T> {
    fn err_context(self, context: &LayoutContext) -> Result<T, RenderError> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(RenderError::Taffy {
                path: context.path.clone(),
                taffy_error: e,
            }),
        }
    }
}
