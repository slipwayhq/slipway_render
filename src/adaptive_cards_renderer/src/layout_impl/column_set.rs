use std::{cell::RefCell, rc::Rc};

use adaptive_cards::Column;
use taffy::{Dimension, TaffyTree};

use crate::{
    element_layout_data::{ElementLayoutData, ElementTaffyData},
    errors::RenderError,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::AsLayoutable,
    masked_image::MaskedImage,
};

use super::container_shared::{container_draw_override, vertical_container_layout_override};

impl AsLayoutable for Column<ElementLayoutData> {
    fn as_layoutable(&self) -> &dyn crate::layoutable::Layoutable {
        self
    }
}

impl crate::layoutable::Layoutable for adaptive_cards::ColumnSet<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        mut baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        baseline_style.min_size.width = Dimension::Percent(1.);
        vertical_container_layout_override(self, context, baseline_style, tree)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        // Delegate to the shared container draw function.
        container_draw_override(self, context, tree, taffy_data, image, scratch)
    }
}
