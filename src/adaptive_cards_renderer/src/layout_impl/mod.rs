use adaptive_cards::{
    AdaptiveCard, Column, ColumnSet, Container, ContainerStyle, Element, HorizontalAlignment,
    StackableToggleable, Table, TableCell, TableRow, VerticalAlignment,
};
use container_shared::PaddingBehavior;

use crate::{
    element_layout_data::{ElementLayoutData, Placement},
    layoutable::Layoutable,
};

mod adaptive_card;
mod column;
mod column_set;
mod container;
mod container_shared;
mod image;
pub(crate) mod measure;
mod table;
mod table_cell;
mod table_row;
mod text_block;
mod utils;

// Unimplemented adaptive card items.
impl crate::layoutable::Layoutable for adaptive_cards::TextRun<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionExecute<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionOpenUrl<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionShowCard<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionSubmit<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionToggleVisibility<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::FactSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ImageSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputChoiceSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputDate<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputNumber<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputText<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputTime<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputToggle<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::Media<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::RichTextBlock<ElementLayoutData> {}

enum ItemsContainerOrientation {
    Vertical,
    Horizontal,
}

trait ItemsContainer<TItem>
where
    TItem: StackableToggleable + Layoutable,
{
    fn children(&self) -> &[TItem];

    fn min_height(&self) -> Option<&str>;

    fn bleed(&self) -> bool;

    fn placement(&self) -> Placement;

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment>;

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment>;

    fn style(&self) -> Option<ContainerStyle>;

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::ForStyle(self.style())
    }

    fn children_collection_name(&self) -> &'static str {
        "items"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Vertical
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for AdaptiveCard<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.body.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        None
    }

    fn bleed(&self) -> bool {
        false
    }

    fn placement(&self) -> Placement {
        Placement::SoleVertical
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        None
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::Always
    }

    fn children_collection_name(&self) -> &'static str {
        "body"
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for Container<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_slice()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for Column<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }
}

impl ItemsContainer<Column<ElementLayoutData>> for ColumnSet<ElementLayoutData> {
    fn children(&self) -> &[Column<ElementLayoutData>] {
        self.columns.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        None
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }

    fn children_collection_name(&self) -> &'static str {
        "columns"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Horizontal
    }
}

impl ItemsContainer<TableRow<ElementLayoutData>> for Table<ElementLayoutData> {
    fn children(&self) -> &[TableRow<ElementLayoutData>] {
        self.rows.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        None
    }

    fn bleed(&self) -> bool {
        false
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_cell_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_cell_content_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        None
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::None
    }

    fn children_collection_name(&self) -> &'static str {
        "rows"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Vertical
    }
}

impl ItemsContainer<TableCell<ElementLayoutData>> for TableRow<ElementLayoutData> {
    fn children(&self) -> &[TableCell<ElementLayoutData>] {
        self.cells.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        None
    }

    fn bleed(&self) -> bool {
        false
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_cell_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_cell_content_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::None
    }

    fn children_collection_name(&self) -> &'static str {
        "cells"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Horizontal
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for TableCell<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_slice()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment.or_else(|| {
            self.layout_data
                .borrow()
                .table_column_definition()
                .vertical_cell_content_alignment
        })
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.layout_data
            .borrow()
            .table_column_definition()
            .horizontal_cell_content_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::AlwaysNarrow
    }
}
