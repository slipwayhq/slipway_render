#[allow(
    clippy::to_string_trait_impl,
    clippy::derivable_impls,
    clippy::wrong_self_convention
)]
mod generated;

pub use generated::*;

pub fn default() -> HostConfig {
    crate::builder::HostConfig::default()
        .try_into()
        .expect("default host config should be valid")
}

impl HostConfig {
    pub fn get_all_font_family_stacks(&self) -> Vec<&str> {
        vec![
            &self.font_types.default.font_family,
            &self.font_types.monospace.font_family,
        ]
    }
}
