use std::{cell::RefCell, rc::Rc};

use std::fmt;

use imageproc::rect::Rect;
use taffy::{Layout, NodeId, TaffyTree};

use crate::errors::TaffyErrorToRenderError;
use crate::masked_image::MaskedImage;
use crate::{
    errors::RenderError, host_config::generated::HostConfig, masked_image::SlipwayCanvas,
    size::Size,
};

pub(super) trait Layoutable: HasLayoutData {
    fn layout(
        &self,
        context: &LayoutContext,
        tree: &mut TaffyTree<()>,
    ) -> Result<NodeId, RenderError> {
        let layout_data = self.layout_data();

        let node_id = self.layout_override(context, tree)?;

        let mut data_mut = layout_data.borrow_mut();
        data_mut.node_id = Some(node_id);

        Ok(node_id)
    }

    fn layout_override(
        &self,
        context: &LayoutContext,
        _tree: &mut TaffyTree<()>,
    ) -> Result<NodeId, RenderError> {
        unimplemented!("layout not implemented for {}", context.path.clone());
    }

    fn draw(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<()>,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        let layout_data = self.layout_data();
        let data = layout_data.borrow();

        let node_id = data.node_id.ok_or(RenderError::NodeIdNotFound {
            path: context.path.clone(),
        })?;

        let node_layout = tree.layout(node_id).err_context(context)?;

        let (width, height) = image.dimensions();
        let image_rect = Rect::at(0, 0).of_size(width, height);
        let actual_rect = node_layout.actual_rect();

        if image_rect.intersect(actual_rect).is_some() {
            self.draw_override(context, tree, node_id, image)?;
        }

        Ok(())
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        _tree: &TaffyTree<()>,
        _node_id: NodeId,
        _image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        unimplemented!("draw_override not implemented for {}", context.path.clone());
    }
}

pub(super) trait TaffyLayoutUtils {
    fn actual_rect(&self) -> Rect;
}

impl TaffyLayoutUtils for Layout {
    fn actual_rect(&self) -> Rect {
        Rect::at(self.location.x as i32, self.location.y as i32)
            .of_size(self.size.width as u32, self.size.height as u32)
    }
}

// Implement Layoutable for Box<T> where T: Layoutable
impl<T: Layoutable> Layoutable for Box<T> {
    fn layout(
        &self,
        context: &LayoutContext,
        tree: &mut TaffyTree<()>,
    ) -> Result<NodeId, RenderError> {
        self.as_ref().layout(context, tree)
    }

    fn layout_override(
        &self,
        context: &LayoutContext,
        tree: &mut TaffyTree<()>,
    ) -> Result<NodeId, RenderError> {
        self.as_ref().layout_override(context, tree)
    }

    fn draw(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<()>,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        self.as_ref().draw(context, tree, image)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<()>,
        node_id: NodeId,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        self.as_ref().draw_override(context, tree, node_id, image)
    }
}

impl<T: HasLayoutData> HasLayoutData for Box<T> {
    fn layout_data(&self) -> &RefCell<LayoutData> {
        self.as_ref().layout_data()
    }
}

pub(super) trait HasLayoutData {
    fn layout_data(&self) -> &RefCell<LayoutData>;
    fn node_id(&self) -> NodeId {
        self.layout_data()
            .borrow()
            .node_id
            .expect("Element should have a node_id")
    }
}

#[derive(Default, Debug, Clone)]
pub(super) struct LayoutData {
    pub node_id: Option<NodeId>,
}

#[derive(Clone, Debug)]
pub(super) struct MeasureResult {
    pub desired_size: Size,
    previous_measure: Size,
}

#[derive(Clone, Debug)]
pub(super) struct ArrangeResult {
    pub actual_rect: Rect,
    previous_arrange: Rect,
}

pub(super) struct LayoutContext<'hc> {
    pub host_config: &'hc HostConfig,
    pub path: Rc<LayoutPath>,
}

impl<'hc> LayoutContext<'hc> {
    pub fn new(host_config: &'hc HostConfig) -> Self {
        LayoutContext {
            host_config,
            path: Rc::new(LayoutPath {
                current: "root".to_string(),
                previous: None,
            }),
        }
    }

    pub fn for_child_str(&self, child_name: &str) -> Self {
        self.for_child(child_name.to_string())
    }

    pub fn for_child(&self, child_name: String) -> Self {
        LayoutContext {
            host_config: self.host_config,
            path: Rc::new(LayoutPath {
                current: child_name,
                previous: Some(self.path.clone()),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LayoutPath {
    pub current: String,
    pub previous: Option<Rc<LayoutPath>>,
}

impl fmt::Display for LayoutPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.current)?;
        if let Some(previous) = self.previous.as_ref() {
            write!(f, " <- {}", previous)?;
        }
        Ok(())
    }
}
