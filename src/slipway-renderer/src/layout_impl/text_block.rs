use std::{cell::RefCell, rc::Rc};

use image::Rgba;
use imageproc::drawing::draw_filled_rect_mut;
use taffy::{Dimension, NodeId, Size, Style, TaffyTree};

use crate::{
    errors::{RenderError, TaffyErrorToRenderError},
    layoutable::{LayoutContext, Layoutable, TaffyLayoutUtils},
    masked_image::MaskedImage,
    TextBlock,
};

use super::NodeContext;

impl Layoutable for TextBlock {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<NodeId, RenderError> {
        tree.new_leaf_with_context(
            Style {
                size: Size {
                    width: Dimension::Length(100.),
                    height: Dimension::Length(10.),
                },
                ..baseline_style
            },
            NodeContext::Text,
        )
        .err_context(context)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        node_id: NodeId,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        let mut image_mut = image.borrow_mut();
        let node_layout = tree.layout(node_id).err_context(context)?;

        draw_filled_rect_mut(
            &mut *image_mut,
            node_layout.actual_rect(context),
            Rgba([0, 0, 255, 255]),
        );

        Ok(())
    }
}
