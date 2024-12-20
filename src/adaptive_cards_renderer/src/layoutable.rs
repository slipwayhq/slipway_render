use std::{cell::RefCell, rc::Rc};

use adaptive_cards::{Element, HasLayoutData};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use taffy::{NodeId, Style, TaffyTree};

use crate::debug_mode::next_color;
use crate::element_layout_data::{ElementLayoutData, ElementTaffyData};
use crate::errors::TaffyErrorToRenderError;
use crate::layout_context::LayoutContext;
use crate::layout_impl::measure::NodeContext;
use crate::layout_scratch::LayoutScratch;
use crate::masked_image::MaskedImage;
use crate::utils::TaffyLayoutUtils;
use crate::{errors::RenderError, masked_image::SlipwayCanvas};

adaptive_cards::impl_as_trait!(
    crate::layoutable::Layoutable,
    AsLayoutable,
    as_layoutable,
    ElementLayoutData
);

/// A trait for Adaptive Card elements which can be laid out and drawn.
/// The default implementations of `layout_override` and `draw_override` which will panic if used.
/// This default implementation is used by elements we haven't implemented yet.
pub(super) trait Layoutable: HasLayoutData<ElementLayoutData> {
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
        scratch: &mut LayoutScratch,
    ) -> Result<NodeId, RenderError> {
        let layout_data = self.layout_data();

        let taffy_data = self.layout_override(context, baseline_style, tree, scratch)?;
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
        _scratch: &mut LayoutScratch,
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
        context: LayoutContext,
        tree: &TaffyTree<NodeContext>,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        // context.print_local_context();
        let mut results = Vec::new();

        {
            let self_layout_data = self.layout_data().borrow();

            // If we have virtual elements, draw them instead of the main element.
            let virtual_elements = {
                match self_layout_data.virtual_elements.as_deref() {
                    None => vec![&*self_layout_data],
                    Some(virtual_elements) => virtual_elements.iter().collect(),
                }
            };

            for (i, layout_data) in virtual_elements.iter().enumerate() {
                let absolute_rect =
                    {
                        let mut context = context.for_child(i.to_string());

                        let taffy_data = layout_data.taffy_data.as_ref().ok_or(
                            RenderError::TaffyDataNotFound {
                                path: context.path.clone(),
                            },
                        )?;

                        let node_layout = tree.layout(taffy_data.node_id).err_context(&context)?;
                        context.ensure_origin(node_layout.location);

                        let (width, height) = image.dimensions();
                        let image_rect = Rect::at(0, 0).of_size(width, height);
                        let absolute_rect = node_layout.absolute_rect(&context);

                        if image_rect.intersect(absolute_rect).is_some() {
                            // If we should draw debug outlines, do so.
                            if context.debug_mode.outlines {
                                let color = next_color();
                                let mut image_mut = image.borrow_mut();

                                draw_hollow_rect_mut(&mut *image_mut, absolute_rect, color);
                            }

                            let masked_image = MaskedImage::from_mask(image.clone(), absolute_rect);

                            self.draw_override(&context, tree, taffy_data, masked_image, scratch)?;
                        }
                        absolute_rect
                    };

                results.push(absolute_rect);
            }
        }

        {
            let mut self_layout_data = self.layout_data().borrow_mut();

            // If we have virtual elements, draw them instead of the main element.
            let mut virtual_elements = {
                match self_layout_data.virtual_elements.as_deref_mut() {
                    None => vec![&mut *self_layout_data],
                    Some(virtual_elements) => virtual_elements.iter_mut().collect(),
                }
            };

            for (layout_data, result) in virtual_elements.iter_mut().zip(results) {
                if layout_data.rect.is_some() {
                    panic!(
                        "Layout data rect has already been set for: {}",
                        context.path.clone()
                    );
                }
                layout_data.rect = Some(result.into());
            }
        }

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
        _scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        unimplemented!("draw_override not implemented for {}", context.path.clone());
    }
}

/// Implement Layoutable for all boxed Layoutable types.
impl<T: Layoutable> Layoutable for Box<T> {
    fn layout(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
        scratch: &mut LayoutScratch,
    ) -> Result<NodeId, RenderError> {
        self.as_ref().layout(context, baseline_style, tree, scratch)
    }

    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
        scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        self.as_ref()
            .layout_override(context, baseline_style, tree, scratch)
    }

    fn draw(
        &self,
        context: LayoutContext,
        tree: &TaffyTree<NodeContext>,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        self.as_ref().draw(context, tree, image, scratch)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        self.as_ref()
            .draw_override(context, tree, taffy_data, image, scratch)
    }
}

impl Layoutable for Element<ElementLayoutData> {
    fn layout(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
        scratch: &mut LayoutScratch,
    ) -> Result<NodeId, RenderError> {
        self.as_layoutable()
            .layout(context, baseline_style, tree, scratch)
    }

    fn draw(
        &self,
        context: LayoutContext,
        tree: &TaffyTree<NodeContext>,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        self.as_layoutable().draw(context, tree, image, scratch)
    }
}
