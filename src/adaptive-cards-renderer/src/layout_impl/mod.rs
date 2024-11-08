use crate::element_layout_data::ElementLayoutData;

mod adaptive_card;
mod container;
mod container_shared;
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
impl crate::layoutable::Layoutable for adaptive_cards::ColumnSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::Column<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::FactSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::Image<ElementLayoutData> {}
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
