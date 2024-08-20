use core::fmt;
use std::rc::Rc;

use taffy::Point;

use crate::{host_config::generated::HostConfig, DebugMode};

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
