use thiserror::Error;

use crate::layoutable::LayoutPath;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Measure failed for {path}:\n{message}")]
    MeasureError { path: LayoutPath, message: String },

    #[error("Arrange failed for {path}:\n{message}")]
    ArrangeError { path: LayoutPath, message: String },

    #[error("Draw failed for {path}:\n{message}")]
    DrawError { path: LayoutPath, message: String },
}
