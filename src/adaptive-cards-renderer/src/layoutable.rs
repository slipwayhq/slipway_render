use std::{cell::RefCell, rc::Rc};

use imageproc::rect::Rect;
use taffy::{Layout, NodeId, Style, TaffyTree};

use crate::element_layout_data::{ElementLayoutData, ElementTaffyData};
use crate::errors::TaffyErrorToRenderError;
use crate::layout_context::LayoutContext;
use crate::layout_impl::NodeContext;
use crate::masked_image::MaskedImage;
use crate::{errors::RenderError, masked_image::SlipwayCanvas};

/// A trait for Adaptive Card elements which can be laid out and drawn.
/// This trait is implemented automatically by the Adaptive Cards types generator for
/// elements in the list `UNIMPLEMENTED_LAYOUTABLE_TYPES` using the default implementations
/// of `layout_override` and `draw_override` which will panic if used.
/// Other elements will have this trait implemented manually and `layout_override` and
/// `draw_override` should be implemented for them.
pub(super) trait Layoutable: HasLayoutData {
    /// Lays out the element and its descendants by populating the Taffy tree and returning the
    /// node_id of this element in the tree.
    /// This default implementation should be sufficient for all elements and does not
    /// need to be overridden. It handles calling `layout_override` and storing the resulting
    /// Taffy data in the element struct.
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

    /// Lays out the element and its descendants by populating the Taffy tree and returning the
    /// ElementTaffyData for this element.
    /// This method should be overridden for each element type.
    fn layout_override(
        &self,
        context: &LayoutContext,
        _baseline_style: Style,
        _tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        unimplemented!("layout not implemented for {}", context.path.clone());
    }

    /// Draws the element and its descendants onto the image.
    /// This default implementation should be sufficient for all elements and does not
    /// need to be overridden. It handles fetching the layout data and checking that
    /// the element's drawing rect intersects with the image before calling `draw_override`,
    /// and storing the final absolute rect used for drawing the element in the layout data.
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

    /// Draws the element and its descendants onto the image.
    /// This method should be overridden for each element type.
    /// Note that the context will have had it's `current_origin` set for the element's
    /// top-left corner, and the image will have been masked to the element's drawing rect.
    /// Other layout information can be fetched from the Taffy tree using the node id and
    /// child node ids stored in the `taffy_data` parameter.
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

/// A trait for easily getting the absolute rect of an element from the Taffy
/// Layout data using the element's context data.
pub(super) trait TaffyLayoutUtils {
    /// Gets the absolute rect of the element using the context data.
    fn absolute_rect(&self, context: &LayoutContext) -> Rect;
}

impl TaffyLayoutUtils for Layout {
    fn absolute_rect(&self, context: &LayoutContext) -> Rect {
        // The context already has the element's absolute origin set,
        // so we just need to use that and the element's size to get the absolute rect.
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

/// Implement Layoutable for a boxed Layoutable type.
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

/// Implement HasLayoutData for a boxed HasLayoutData type.
impl<T: HasLayoutData> HasLayoutData for Box<T> {
    fn layout_data(&self) -> &RefCell<ElementLayoutData> {
        self.as_ref().layout_data()
    }
}

/// A trait for Adaptive Card elements which have layout data. This trait is implemented
/// automatically by the Adaptive Cards types generator.
pub(super) trait HasLayoutData {
    /// Gets the layout data for the element.
    fn layout_data(&self) -> &RefCell<ElementLayoutData>;

    /// Gets the Taffy node id for the element, panicing if the Taffy data doesn't exist.
    fn node_id(&self) -> NodeId {
        self.layout_data()
            .borrow()
            .taffy_data
            .as_ref()
            .expect("Element should have taffy data")
            .node_id
    }

    /// Gets the Taffy node ids for the child elements of the element, panicing
    /// if the Taffy data doesn't exist.
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
