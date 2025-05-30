use core::fmt;
use std::{collections::HashMap, rc::Rc};

use adaptive_cards_host_config::HostConfig;
use image::RgbaImage;
use taffy::Point;

use crate::{errors::RenderError, host_context::HostContext, DebugMode};
use adaptive_cards::{ContainerStyle, HorizontalAlignment, VerticalAlignment};

/// The context for the layout and draw passes of a single element.
#[derive(Clone)]
pub(super) struct LayoutContext<'cfg, 'ctx, 'render> {
    /// The host configuration for the layout.
    pub host_config: &'cfg HostConfig,

    pub host_context: &'ctx dyn HostContext,

    /// The debug mode for the layout (whether we should draw additional debug information).
    pub debug_mode: DebugMode,

    /// The path of the current element in the element tree.
    pub path: Rc<LayoutPath>,

    /// The current origin of the element relative to the origin of the image being rendered.
    /// This should only be used during the draw pass, where it should be set by the
    /// parent element based on the calculated layout position of the child element.
    pub current_origin: Point<f32>,

    /// Stored so we know if the origin has been updated.
    pub relative_origin: Option<Point<f32>>,

    /// The inherited context for the current element, passed down from the parent element.
    pub inherited: InheritedContext,

    // A map from the font stack in the host config to the font family resolved by the host.
    font_stack_to_resolved_family_map: &'render HashMap<String, String>,

    // A cache of resolved images, so we don't resolve them multiple times.
    image_cache: &'render HashMap<String, RgbaImage>,
}

impl<'cfg, 'ctx, 'render> LayoutContext<'cfg, 'ctx, 'render> {
    #[must_use]
    pub fn new(
        host_config: &'cfg HostConfig,
        host_context: &'ctx dyn HostContext,
        debug_mode: DebugMode,
        font_stack_to_resolved_family_map: &'render HashMap<String, String>,
        image_cache: &'render HashMap<String, RgbaImage>,
    ) -> Self {
        LayoutContext {
            host_config,
            host_context,
            debug_mode,
            path: Rc::new(LayoutPath {
                current: "root".to_string(),
                previous: None,
            }),
            current_origin: Point { x: 0., y: 0. },
            relative_origin: Some(Point { x: 0., y: 0. }),
            inherited: InheritedContext::default(),
            font_stack_to_resolved_family_map,
            image_cache,
        }
    }

    /// Creates a new LayoutContext for a child element with the given name.
    /// The `current_origin` of the child element will remain the same.
    #[must_use]
    pub fn for_child_str(&self, child_name: &str) -> Self {
        self.for_child(child_name.to_string())
    }

    // Creates a new LayoutContext for a child element with the given name.
    /// The `current_origin` of the child element will remain the same.
    #[must_use]
    pub fn for_child(&self, child_name: String) -> Self {
        self.for_child_origin_inner(child_name, None)
    }

    /// Creates a new LayoutContext for a child element with the given name and relative location.
    /// The `current_origin` of the child element will be the sum of the current origin and the relative location.
    #[must_use]
    pub fn for_child_str_origin(&self, child_name: &str, relative_location: Point<f32>) -> Self {
        self.for_child_origin_inner(child_name.to_string(), Some(relative_location))
    }

    /// Creates a new LayoutContext for a child element with the given name and relative location.
    /// The `current_origin` of the child element will be the sum of the current origin and the relative location.
    #[must_use]
    pub fn for_child_origin(&self, child_name: String, relative_location: Point<f32>) -> Self {
        self.for_child_origin_inner(child_name, Some(relative_location))
    }

    /// Creates a new LayoutContext for a child element with the given name and relative location.
    /// The `current_origin` of the child element will be the sum of the current origin and the relative location.
    #[must_use]
    pub fn for_child_origin_inner(
        &self,
        child_name: String,
        relative_origin: Option<Point<f32>>,
    ) -> Self {
        let current_origin = match relative_origin {
            None => self.current_origin,
            Some(relative_location) => self.current_origin + relative_location,
        };

        LayoutContext {
            host_config: self.host_config,
            host_context: self.host_context,
            debug_mode: self.debug_mode,
            path: Rc::new(LayoutPath {
                current: child_name,
                previous: Some(self.path.clone()),
            }),
            current_origin,
            relative_origin,
            inherited: self.inherited,
            font_stack_to_resolved_family_map: self.font_stack_to_resolved_family_map,
            image_cache: self.image_cache,
        }
    }

    /// Sets the origin of the current element, ensuring it hasn't changed if it was previously set.
    pub fn ensure_origin(&mut self, relative_location: Point<f32>) {
        if let Some(existing_relative_origin) = self.relative_origin {
            if existing_relative_origin != relative_location {
                panic!("Attempted to set the origin of a LayoutContext more than once with different values.");
            }
        } else {
            self.current_origin = self.current_origin + relative_location;
            self.relative_origin = Some(relative_location);
        }
    }

    /// Creates a new LayoutContext based on the current context but with the given container style.
    #[must_use]
    pub fn with_style(mut self, value: Option<ContainerStyle>) -> Self {
        if let Some(value) = value {
            self.inherited.style = value;
        }

        self
    }

    /// Creates a new LayoutContext based on the current context but with the given vertical content alignment.
    #[must_use]
    pub fn with_vertical_content_alignment(mut self, value: Option<VerticalAlignment>) -> Self {
        if let Some(value) = value {
            self.inherited.vertical_content_alignment = value;
        }
        self
    }

    /// Creates a new LayoutContext based on the current context but with the given horizontal alignment.
    #[must_use]
    pub fn with_horizontal_content_alignment(mut self, value: Option<HorizontalAlignment>) -> Self {
        if let Some(value) = value {
            self.inherited.horizontal_alignment = value;
        }
        self
    }

    /// Creates a new LayoutContext based on the current context but with is_header set to true.
    #[must_use]
    pub fn within_header(mut self) -> Self {
        self.inherited.within_header = true;
        self
    }

    /// Prints the current context to the console.
    pub fn print_local_context(&self) {
        println!("debug: {}: {:?}", self.path, self.current_origin);
    }

    /// Gets the resolved font family for the given font stack.
    pub fn get_resolved_font_family(&self, font_stack: &str) -> Result<String, RenderError> {
        self.font_stack_to_resolved_family_map
            .get(font_stack)
            .cloned()
            .ok_or(RenderError::Other {
                path: self.path.clone(),
                message: format!("Font stack '{}' has not been resolved.", font_stack),
            })
    }
}

/// The path of an element in the element tree.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LayoutPath {
    /// The name of the current element.
    pub current: String,

    /// The path of the parent element, if any.
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

/// The inherited context for an element. This is context which affects
/// layout and drawing but is not directly controlled by the element itself.
#[derive(Copy, Clone, Debug)]
pub(super) struct InheritedContext {
    pub style: ContainerStyle,
    pub vertical_content_alignment: VerticalAlignment,
    pub horizontal_alignment: HorizontalAlignment,
    pub within_header: bool,
}

impl Default for InheritedContext {
    fn default() -> Self {
        InheritedContext {
            style: ContainerStyle::default(),
            vertical_content_alignment: VerticalAlignment::default(),
            horizontal_alignment: HorizontalAlignment::Left,
            within_header: false,
        }
    }
}
