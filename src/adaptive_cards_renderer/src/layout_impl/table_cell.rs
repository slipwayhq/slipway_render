use std::{cell::RefCell, rc::Rc};

use adaptive_cards::TableCell;
use taffy::TaffyTree;

use crate::{
    element_layout_data::ElementTaffyData, errors::RenderError, layout_context::LayoutContext,
    layout_scratch::LayoutScratch, masked_image::MaskedImage, ElementLayoutData,
};

use super::{
    container_shared::{container_draw_override, container_layout_override},
    measure::NodeContext,
    Layoutable,
};

impl Layoutable for TableCell<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
        scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        container_layout_override(self, context, baseline_style, tree, scratch)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        println!("Cell draw_override");
        container_draw_override(self, context, tree, taffy_data, image, scratch)
    }
}
