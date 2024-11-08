// Allow unused code
#![allow(dead_code)]

mod debug_mode;
mod element_layout_data;
mod errors;
pub mod host_config_utils;
mod layout_context;
mod layout_impl;
mod layout_scratch;
mod layoutable;
mod masked_image;
mod print_tree;
mod rect;
pub mod render;
mod utils;

pub use debug_mode::DebugMode;
pub use element_layout_data::ElementLayoutData;
pub use host_config_utils::default_host_config::default_host_config;

// Bring in the `impl_as_trait!` macro.
extern crate adaptive_cards;
