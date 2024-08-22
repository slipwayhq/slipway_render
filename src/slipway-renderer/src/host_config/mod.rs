use csscolorparser::ParseColorError;
use generated::SpacingsConfig;
use image::Rgba;

use crate::{element::LayoutableElement, errors::RenderError, Spacing};

pub mod default_host_config;

#[allow(
    clippy::to_string_trait_impl,
    clippy::derivable_impls,
    clippy::wrong_self_convention
)]
pub(super) mod generated;

pub(super) trait ValidSpacing {
    fn from(&self, element: &dyn LayoutableElement) -> u32;

    #[allow(clippy::wrong_self_convention)]
    fn from_spacing(&self, spacing: Spacing) -> u32;

    fn padding(&self) -> u32;
}

impl ValidSpacing for Option<SpacingsConfig> {
    fn from(&self, element: &dyn LayoutableElement) -> u32 {
        self.from_spacing(element.get_spacing())
    }
    fn from_spacing(&self, spacing: Spacing) -> u32 {
        match spacing {
            Spacing::None => 0,
            Spacing::Small => valid_spacing(self.as_ref().map_or(0, |s| s.small)),
            Spacing::Medium => valid_spacing(self.as_ref().map_or(0, |s| s.medium)),
            Spacing::Large => valid_spacing(self.as_ref().map_or(0, |s| s.large)),
            Spacing::ExtraLarge => valid_spacing(self.as_ref().map_or(0, |s| s.extra_large)),
            Spacing::Padding => valid_spacing(self.as_ref().map_or(0, |s| s.padding)),
            Spacing::Default => valid_spacing(self.as_ref().map_or(0, |s| s.default)),
        }
    }

    fn padding(&self) -> u32 {
        valid_spacing(self.as_ref().map_or(0, |s| s.default))
    }
}

fn valid_spacing(spacing: i64) -> u32 {
    if spacing < 0 {
        0
    } else {
        spacing as u32
    }
}

impl generated::HostConfig {
    pub fn get_separator(&self) -> generated::SeparatorConfig {
        self.separator.clone().unwrap_or_else(|| {
            generated::builder::SeparatorConfig::default()
                .try_into()
                .expect("Default separator config should be valid")
        })
    }
    pub fn get_separator_line_thickness(&self) -> Result<u32, RenderError> {
        let maybe_line_thickness = self.get_separator().line_thickness;
        maybe_line_thickness
            .try_into()
            .map_err(|_| RenderError::HostConfig {
                path: "separator.lineThickness".to_string(),
                message: format!("Failed to parse {} as u32", maybe_line_thickness),
            })
    }

    pub fn get_separator_line_color(&self) -> Result<Rgba<u8>, RenderError> {
        let line_color = self
            .get_separator()
            .line_color
            .expect("separator.lineColor should be set");
        parse_color(&line_color).map_err(|_| RenderError::HostConfig {
            path: "separator.lineColor".to_string(),
            message: format!("Failed to parse color {}", line_color),
        })
    }
}

fn parse_color(input: &str) -> Result<image::Rgba<u8>, ParseColorError> {
    let c = csscolorparser::parse(input)?;
    Ok(image::Rgba(c.to_rgba8()))
}
