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
    fn is_visible(&self) -> bool;
}

impl<T: Toggleable> Toggleable for Box<T> {
    fn is_visible(&self) -> bool {
        self.as_ref().is_visible()
    }
}

// Implemented by all elements, and Column.
pub trait StackableToggleable: Toggleable {
    fn separator(&self) -> bool;
    fn spacing(&self) -> Spacing;
}

impl<T: StackableToggleable> StackableToggleable for Box<T> {
    fn separator(&self) -> bool {
        self.as_ref().separator()
    }

    fn spacing(&self) -> Spacing {
        self.as_ref().spacing()
    }
}

// Implemented by all elements, and Column.
pub trait SizedStackableToggleable: StackableToggleable {
    fn width_or_height(&self) -> WidthOrHeight;
}

impl<T: SizedStackableToggleable> SizedStackableToggleable for Box<T> {
    fn width_or_height(&self) -> WidthOrHeight {
        self.as_ref().width_or_height()
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
    fn is_visible(&self) -> bool {
        true
    }
}

impl<TLayoutData> StackableToggleable for TableRow<TLayoutData>
where
    TLayoutData: Default,
{
    fn separator(&self) -> bool {
        true
    }

    fn spacing(&self) -> Spacing {
        Spacing::None
    }
}

impl<TLayoutData> SizedStackableToggleable for TableRow<TLayoutData>
where
    TLayoutData: Default,
{
    fn width_or_height(&self) -> WidthOrHeight {
        WidthOrHeight::Height(StringOrBlockElementHeight::BlockElementHeight(
            BlockElementHeight::Auto,
        ))
    }
}

impl<TLayoutData> Toggleable for TableCell<TLayoutData>
where
    TLayoutData: Default,
{
    fn is_visible(&self) -> bool {
        true
    }
}

impl<TLayoutData> StackableToggleable for TableCell<TLayoutData>
where
    TLayoutData: Default,
{
    fn separator(&self) -> bool {
        true
    }

    fn spacing(&self) -> Spacing {
        Spacing::None
    }
}

pub trait SizedLayoutData {
    fn width_or_height(&self) -> WidthOrHeight;
}

impl<TLayoutData> SizedStackableToggleable for TableCell<TLayoutData>
where
    TLayoutData: Default + SizedLayoutData,
{
    fn width_or_height(&self) -> WidthOrHeight {
        self.layout_data().borrow().width_or_height()
    }
}

impl<TLayoutData> Default for TextBlock<TLayoutData>
where
    TLayoutData: Default + SizedLayoutData,
{
    fn default() -> Self {
        TextBlock {
            color: None,
            fallback: None,
            font_type: None,
            height: None,
            horizontal_alignment: None,
            id: None,
            is_subtle: false,
            is_visible: true,
            max_lines: None,
            requires: None,
            separator: None,
            size: None,
            spacing: None,
            style: TextBlockStyle::Default,
            text: String::new(),
            weight: None,
            wrap: true,
            type_: None,
            layout_data: Default::default(),
        }
    }
}
impl<TLayoutData> Default for Table<TLayoutData>
where
    TLayoutData: Default + SizedLayoutData,
{
    fn default() -> Self {
        Table {
            columns: None,
            fallback: None,
            first_row_as_header: true,
            grid_style: None,
            height: None,
            horizontal_cell_content_alignment: None,
            id: None,
            is_visible: true,
            requires: None,
            rows: None,
            separator: None,
            show_grid_lines: true,
            spacing: None,
            vertical_cell_content_alignment: None,
            type_: None,
            layout_data: Default::default(),
        }
    }
}

impl<TLayoutData> Default for TableRow<TLayoutData>
where
    TLayoutData: Default + SizedLayoutData,
{
    fn default() -> Self {
        TableRow {
            cells: None,
            horizontal_cell_content_alignment: None,
            style: None,
            vertical_cell_content_alignment: None,
            type_: None,
            layout_data: Default::default(),
        }
    }
}
impl<TLayoutData> Default for TableCell<TLayoutData>
where
    TLayoutData: Default + SizedLayoutData,
{
    fn default() -> Self {
        TableCell {
            background_image: None,
            bleed: None,
            items: Vec::new(),
            min_height: None,
            rtl: None,
            select_action: None,
            style: None,
            vertical_content_alignment: None,
            type_: None,
            layout_data: Default::default(),
        }
    }
}
