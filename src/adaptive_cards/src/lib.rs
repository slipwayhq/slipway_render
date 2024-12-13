mod deserializers;
#[allow(
    dead_code,
    clippy::to_string_trait_impl,
    clippy::derivable_impls,
    clippy::wrong_self_convention,
    clippy::bool_comparison
)]
mod generated;

#[cfg(test)]
mod tests;

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

impl<TLayoutData> generated::ColumnSet<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }
}

pub trait HasLayoutData<TLayoutData> {
    /// Gets the layout data for the element.
    fn layout_data(&self) -> &RefCell<TLayoutData>;
}

/// Implement HasLayoutData for a boxed HasLayoutData type.
impl<T: HasLayoutData<TLayoutData>, TLayoutData> HasLayoutData<TLayoutData> for Box<T> {
    fn layout_data(&self) -> &RefCell<TLayoutData> {
        self.as_ref().layout_data()
    }
}

// Implemented by everything deriving from ToggleableItem.
pub trait ToggleableItemMethods {
    fn get_is_visible(&self) -> bool;
}

impl<T: ToggleableItemMethods> ToggleableItemMethods for Box<T> {
    fn get_is_visible(&self) -> bool {
        self.as_ref().get_is_visible()
    }
}

// Implemented by all elements, and Column.
pub trait StackableItemMethods: ToggleableItemMethods {
    fn get_separator(&self) -> bool;
    fn get_spacing(&self) -> Spacing;
}

impl<T: StackableItemMethods> StackableItemMethods for Box<T> {
    fn get_separator(&self) -> bool {
        self.as_ref().get_separator()
    }

    fn get_spacing(&self) -> Spacing {
        self.as_ref().get_spacing()
    }
}

/// A trait for getting the properties common to all layoutable Adaptive Cards elements.
/// These are the properties which appear in the `Element` and `ToggleableItem` (which
/// `Element` extends) schema items from the Adaptive Cards typed schema.
/// This trait is implemented for all Adaptive Cards elements automatically by the
/// Adaptive Cards types generator.
pub trait ElementMethods: StackableItemMethods {
    fn get_height(&self) -> StringOrBlockElementHeight;
}

impl<T: ElementMethods> ElementMethods for Box<T> {
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.as_ref().get_height()
    }
}

impl<T> Column<T>
where
    T: Default,
{
    pub fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }

    pub fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
}
