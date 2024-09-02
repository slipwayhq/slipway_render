use std::{cell::RefCell, rc::Rc};

use imageproc::rect::Rect;
use taffy::{Layout, NodeId, Style, TaffyTree};

use crate::element_layout_data::{ElementLayoutData, ElementTaffyData};
use crate::errors::TaffyErrorToRenderError;
use crate::layout_context::LayoutContext;
use crate::layout_impl::NodeContext;
use crate::masked_image::MaskedImage;
use crate::{errors::RenderError, masked_image::SlipwayCanvas};

pub(super) trait Layoutable: HasLayoutData {
    fn layout(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<NodeId, RenderError> {
        let layout_data = self.layout_data();

        let taffy_data = self.layout_override(context, baseline_style, tree)?;
        let node_id = taffy_data.node_id;

        let mut data_mut = layout_data.borrow_mut();
        data_mut.taffy_data = Some(taffy_data);

        Ok(node_id)
    }

    fn layout_override(
        &self,
        context: &LayoutContext,
        _baseline_style: Style,
        _tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        unimplemented!("layout not implemented for {}", context.path.clone());
    }

    fn draw(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        context.print_local_context();

        let refcell_layout_data = self.layout_data();

        let absolute_rect = {
            let layout_data = refcell_layout_data.borrow();
            let taffy_data =
                layout_data
                    .taffy_data
                    .as_ref()
                    .ok_or(RenderError::TaffyDataNotFound {
                        path: context.path.clone(),
                    })?;

            let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;

            let (width, height) = image.dimensions();
            let image_rect = Rect::at(0, 0).of_size(width, height);
            let absolute_rect = node_layout.absolute_rect(context);

            if image_rect.intersect(absolute_rect).is_some() {
                self.draw_override(context, tree, taffy_data, image)?;
            }
            absolute_rect
        };

        refcell_layout_data.borrow_mut().rect = Some(absolute_rect.into());

        Ok(())
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        _tree: &TaffyTree<NodeContext>,
        _taffy_data: &ElementTaffyData,
        _image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        unimplemented!("draw_override not implemented for {}", context.path.clone());
    }
}

pub(super) trait TaffyLayoutUtils {
    fn absolute_rect(&self, context: &LayoutContext) -> Rect;
}

impl TaffyLayoutUtils for Layout {
    fn absolute_rect(&self, context: &LayoutContext) -> Rect {
        Rect::at(
            context.current_origin.x as i32,
            context.current_origin.y as i32,
        )
        .of_size(
            self.size.width.max(1.) as u32,
            self.size.height.max(1.) as u32,
        )
    }
}

// Implement Layoutable for Box<T> where T: Layoutable
impl<T: Layoutable> Layoutable for Box<T> {
    fn layout(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<NodeId, RenderError> {
        self.as_ref().layout(context, baseline_style, tree)
    }

    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        self.as_ref().layout_override(context, baseline_style, tree)
    }

    fn draw(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        self.as_ref().draw(context, tree, image)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        self.as_ref()
            .draw_override(context, tree, taffy_data, image)
    }
}

impl<T: HasLayoutData> HasLayoutData for Box<T> {
    fn layout_data(&self) -> &RefCell<ElementLayoutData> {
        self.as_ref().layout_data()
    }
}

pub(super) trait HasLayoutData {
    fn layout_data(&self) -> &RefCell<ElementLayoutData>;
    fn node_id(&self) -> NodeId {
        self.layout_data()
            .borrow()
            .taffy_data
            .as_ref()
            .expect("Element should have taffy data")
            .node_id
    }
    fn child_node_ids(&self) -> Vec<NodeId> {
        self.layout_data()
            .borrow()
            .taffy_data
            .as_ref()
            .expect("Element should have taffy data")
            .child_element_node_ids
            .clone()
    }
}
