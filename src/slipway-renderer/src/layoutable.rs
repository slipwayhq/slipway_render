use std::{cell::RefCell, rc::Rc};

use std::fmt;

use imageproc::rect::Rect;
use taffy::{Layout, NodeId, Point, Style, TaffyTree};

use crate::errors::TaffyErrorToRenderError;
use crate::layout_impl::NodeContext;
use crate::masked_image::MaskedImage;
use crate::{errors::RenderError, host_config::generated::HostConfig, masked_image::SlipwayCanvas};

pub(super) trait Layoutable: HasLayoutData {
    fn layout(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<NodeId, RenderError> {
        let layout_data = self.layout_data();

        let node_id = self.layout_override(context, baseline_style, tree)?;

        let mut data_mut = layout_data.borrow_mut();
        data_mut.node_id = Some(node_id);

        Ok(node_id)
    }

    fn layout_override(
        &self,
        context: &LayoutContext,
        _baseline_style: Style,
        _tree: &mut TaffyTree<NodeContext>,
    ) -> Result<NodeId, RenderError> {
        unimplemented!("layout not implemented for {}", context.path.clone());
    }

    fn draw(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        context.print_local_context();

        let layout_data = self.layout_data();
        let data = layout_data.borrow();

        let node_id = data.node_id.ok_or(RenderError::NodeIdNotFound {
            path: context.path.clone(),
        })?;

        let node_layout = tree.layout(node_id).err_context(context)?;

        let (width, height) = image.dimensions();
        let image_rect = Rect::at(0, 0).of_size(width, height);
        let actual_rect = node_layout.actual_rect(context);

        if image_rect.intersect(actual_rect).is_some() {
            self.draw_override(context, tree, node_id, image)?;
        }

        Ok(())
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        _tree: &TaffyTree<NodeContext>,
        _node_id: NodeId,
        _image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        unimplemented!("draw_override not implemented for {}", context.path.clone());
    }
}

pub(super) trait TaffyLayoutUtils {
    fn actual_rect(&self, context: &LayoutContext) -> Rect;
}

impl TaffyLayoutUtils for Layout {
    fn actual_rect(&self, context: &LayoutContext) -> Rect {
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
    ) -> Result<NodeId, RenderError> {
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

#[derive(Default, Copy, Clone, Debug)]
pub(super) struct DebugMode {
    pub transparent_masks: bool,
    pub outlines: bool,
}

impl DebugMode {
    pub fn none() -> Self {
        DebugMode {
            transparent_masks: false,
            outlines: false,
        }
    }

    pub fn with_transparent_masks() -> Self {
        DebugMode {
            transparent_masks: true,
            outlines: false,
        }
    }

    pub fn with_outlines() -> Self {
        DebugMode {
            transparent_masks: false,
            outlines: true,
        }
    }

    pub fn full() -> Self {
        DebugMode {
            transparent_masks: true,
            outlines: true,
        }
    }
}

pub(super) struct LayoutContext<'hc> {
    pub host_config: &'hc HostConfig,
    pub debug_mode: DebugMode,
    pub path: Rc<LayoutPath>,
    pub current_origin: Point<f32>,
}

impl<'hc> LayoutContext<'hc> {
    pub fn new(host_config: &'hc HostConfig, debug_mode: DebugMode) -> Self {
        LayoutContext {
            host_config,
            debug_mode,
            path: Rc::new(LayoutPath {
                current: "root".to_string(),
                previous: None,
            }),
            current_origin: Point { x: 0., y: 0. },
        }
    }

    pub fn for_child_str(&self, child_name: &str) -> Self {
        self.for_child(child_name.to_string())
    }

    pub fn for_child(&self, child_name: String) -> Self {
        self.for_child_origin(child_name, Point { x: 0., y: 0. })
    }

    pub fn for_child_str_origin(&self, child_name: &str, relative_location: Point<f32>) -> Self {
        self.for_child_origin(child_name.to_string(), relative_location)
    }

    pub fn for_child_origin(&self, child_name: String, relative_location: Point<f32>) -> Self {
        LayoutContext {
            host_config: self.host_config,
            debug_mode: self.debug_mode,
            path: Rc::new(LayoutPath {
                current: child_name,
                previous: Some(self.path.clone()),
            }),
            current_origin: self.current_origin + relative_location,
        }
    }

    pub fn print_local_context(&self) {
        println!("{}: {:?}", self.path, self.current_origin);
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
