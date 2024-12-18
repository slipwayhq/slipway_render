use std::{cell::RefCell, rc::Rc};

use taffy::TaffyTree;

use crate::{
    element_layout_data::ElementTaffyData, errors::RenderError, layout_context::LayoutContext,
    layout_scratch::LayoutScratch, masked_image::MaskedImage, ElementLayoutData,
};

use super::measure::NodeContext;

impl crate::layoutable::Layoutable for adaptive_cards::Table<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        unimplemented!("Table layout not implemented");
        // let child_elements_context = context
        //     .for_child_str("rows")
        //     .with_vertical_content_alignment(parent.get_vertical_content_alignment())
        //     .with_horizontal_alignment(parent.get_horizontal_alignment())
        //     .with_style(parent.get_style());
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        unimplemented!("Table draw not implemented");
    }
}
