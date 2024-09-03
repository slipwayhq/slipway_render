#[allow(
    clippy::enum_variant_names,
    clippy::bool_comparison,
    clippy::derivable_impls
)]
mod generated;
mod utils;

#[cfg(test)]
mod tests;

pub use generated::*;

impl generated::Container {
    pub fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }
}
