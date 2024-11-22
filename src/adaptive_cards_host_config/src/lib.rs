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
