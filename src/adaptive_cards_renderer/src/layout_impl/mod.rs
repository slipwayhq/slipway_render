use adaptive_cards::{
    AdaptiveCard, Column, ColumnSet, Container, ContainerStyle, Element, HorizontalAlignment,
    StackableToggleable, VerticalAlignment,
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
    fn get_children(&self) -> &[TItem];

    fn get_min_height(&self) -> Option<&str>;

    fn get_bleed(&self) -> bool;

    fn get_placement(&self) -> Placement;

    fn get_vertical_content_alignment(&self) -> Option<VerticalAlignment>;

    fn get_horizontal_alignment(&self) -> Option<HorizontalAlignment>;

    fn get_style(&self) -> Option<ContainerStyle>;

    fn get_padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::ForStyle(self.get_style())
    }

    fn get_children_collection_name(&self) -> &'static str {
        "items"
    }

    fn get_orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Vertical
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for AdaptiveCard<ElementLayoutData> {
    fn get_children(&self) -> &[Element<ElementLayoutData>] {
        self.body.as_deref().unwrap_or_default()
    }

    fn get_min_height(&self) -> Option<&str> {
        None
    }

    fn get_bleed(&self) -> bool {
        false
    }

    fn get_placement(&self) -> Placement {
        Placement::SoleVertical
    }

    fn get_vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn get_horizontal_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn get_style(&self) -> Option<ContainerStyle> {
        None
    }

    fn get_padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::Always
    }

    fn get_children_collection_name(&self) -> &'static str {
        "body"
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for Container<ElementLayoutData> {
    fn get_children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_slice()
    }

    fn get_min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn get_bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn get_placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn get_vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn get_horizontal_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn get_style(&self) -> Option<ContainerStyle> {
        self.style
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for Column<ElementLayoutData> {
    fn get_children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_deref().unwrap_or_default()
    }

    fn get_min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn get_bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn get_placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn get_vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn get_horizontal_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn get_style(&self) -> Option<ContainerStyle> {
        self.style
    }
}

impl ItemsContainer<Column<ElementLayoutData>> for ColumnSet<ElementLayoutData> {
    fn get_children(&self) -> &[Column<ElementLayoutData>] {
        self.columns.as_deref().unwrap_or_default()
    }

    fn get_min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn get_bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn get_placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn get_vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        None
    }

    fn get_horizontal_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn get_style(&self) -> Option<ContainerStyle> {
        self.style
    }

    fn get_children_collection_name(&self) -> &'static str {
        "columns"
    }

    fn get_orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Horizontal
    }
}
