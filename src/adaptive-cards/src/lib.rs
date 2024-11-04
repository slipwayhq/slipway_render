mod element;
#[allow(
    clippy::to_string_trait_impl,
    clippy::derivable_impls,
    clippy::wrong_self_convention,
    clippy::bool_comparison
)]
mod generated;

use std::cell::RefCell;

pub use generated::*;

impl<TLayoutData> generated::Container<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }
}

pub trait HasLayoutData<T> {
    /// Gets the layout data for the element.
    fn layout_data(&self) -> &RefCell<T>;
}
