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

impl<TLayoutData> generated::Column<TLayoutData>
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
pub trait Toggleable {
    fn get_is_visible(&self) -> bool;
}

impl<T: Toggleable> Toggleable for Box<T> {
    fn get_is_visible(&self) -> bool {
        self.as_ref().get_is_visible()
    }
}

// Implemented by all elements, and Column.
pub trait StackableToggleable: Toggleable {
    fn get_separator(&self) -> bool;
    fn get_spacing(&self) -> Spacing;
}

impl<T: StackableToggleable> StackableToggleable for Box<T> {
    fn get_separator(&self) -> bool {
        self.as_ref().get_separator()
    }

    fn get_spacing(&self) -> Spacing {
        self.as_ref().get_spacing()
    }
}

// Implemented by all elements, and Column.
pub trait SizedStackableToggleable: StackableToggleable {
    fn get_width_or_height(&self) -> WidthOrHeight;
}

impl<T: SizedStackableToggleable> SizedStackableToggleable for Box<T> {
    fn get_width_or_height(&self) -> WidthOrHeight {
        self.as_ref().get_width_or_height()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum WidthOrHeight {
    Width(StringOrBlockElementWidthOrNumber),
    Height(StringOrBlockElementHeight),
}

impl<T> Toggleable for TableRow<T>
where
    T: Default,
{
    fn get_is_visible(&self) -> bool {
        true
    }
}

impl<TLayoutData> StackableToggleable for TableRow<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_separator(&self) -> bool {
        true
    }

    fn get_spacing(&self) -> Spacing {
        Spacing::Default
    }
}

impl<TLayoutData> SizedStackableToggleable for TableRow<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_width_or_height(&self) -> WidthOrHeight {
        WidthOrHeight::Height(StringOrBlockElementHeight::BlockElementHeight(
            BlockElementHeight::Auto,
        ))
    }
}

impl<TLayoutData> Toggleable for TableCell<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_is_visible(&self) -> bool {
        true
    }
}

impl<TLayoutData> StackableToggleable for TableCell<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_separator(&self) -> bool {
        true
    }

    fn get_spacing(&self) -> Spacing {
        Spacing::Default
    }
}

pub trait SizedLayoutData {
    fn get_width_or_height(&self) -> WidthOrHeight;
}

impl<TLayoutData> SizedStackableToggleable for TableCell<TLayoutData>
where
    TLayoutData: Default + SizedLayoutData,
{
    fn get_width_or_height(&self) -> WidthOrHeight {
        self.layout_data().borrow().get_width_or_height()
    }
}
