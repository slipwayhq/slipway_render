// Allow unused code
#![allow(dead_code)]

mod adaptive_cards_types;
mod element;
mod errors;
mod host_config;
mod layout_impl;
mod layoutable;
mod masked_image;
mod rect;
mod render;
use adaptive_cards_types::generated::*;
mod debug_mode;
mod element_layout_data;
mod layout_context;
mod utils;

pub use debug_mode::DebugMode;
pub use host_config::default_host_config::default_host_config;
pub use render::render;

#[cfg(test)]
mod tests {}
