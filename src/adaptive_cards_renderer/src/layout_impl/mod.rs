use adaptive_cards::{
    AdaptiveCard, Column, ColumnSet, Container, ContainerStyle, Element, StackableItemMethods,
    VerticalContentAlignment,
};

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
impl crate::layoutable::Layoutable for adaptive_cards::Table<ElementLayoutData> {}

trait HasChildElements<TElement>
where
    TElement: StackableItemMethods + Layoutable,
{
    fn get_child_elements(&self) -> &[TElement];
}

impl HasChildElements<Element<ElementLayoutData>> for Column<ElementLayoutData> {
    fn get_child_elements(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_deref().unwrap_or_default()
    }
}
impl HasChildElements<Column<ElementLayoutData>> for ColumnSet<ElementLayoutData> {
    fn get_child_elements(&self) -> &[Column<ElementLayoutData>] {
        self.columns.as_deref().unwrap_or_default()
    }
}

impl HasChildElements<Element<ElementLayoutData>> for Container<ElementLayoutData> {
    fn get_child_elements(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_slice()
    }
}

impl HasChildElements<Element<ElementLayoutData>> for AdaptiveCard<ElementLayoutData> {
    fn get_child_elements(&self) -> &[Element<ElementLayoutData>] {
        self.body.as_deref().unwrap_or_default()
    }
}

trait VerticalContainer<TElement>: HasChildElements<TElement>
where
    TElement: StackableItemMethods + Layoutable,
{
    fn get_min_height(&self) -> Option<&str>;

    fn get_bleed(&self) -> bool;

    fn get_placement(&self) -> Placement;

    fn get_vertical_content_alignment(&self) -> Option<VerticalContentAlignment>;

    fn get_style(&self) -> Option<ContainerStyle>;
}

impl VerticalContainer<Element<ElementLayoutData>> for Container<ElementLayoutData> {
    fn get_min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn get_bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn get_placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn get_vertical_content_alignment(&self) -> Option<VerticalContentAlignment> {
        self.vertical_content_alignment
    }

    fn get_style(&self) -> Option<ContainerStyle> {
        self.style
    }
}

impl VerticalContainer<Element<ElementLayoutData>> for Column<ElementLayoutData> {
    fn get_min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn get_bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn get_placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn get_vertical_content_alignment(&self) -> Option<VerticalContentAlignment> {
        self.vertical_content_alignment
    }

    fn get_style(&self) -> Option<ContainerStyle> {
        self.style
    }
}
