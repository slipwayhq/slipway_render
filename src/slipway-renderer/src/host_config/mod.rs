use csscolorparser::ParseColorError;
use generated::SpacingsConfig;

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

impl ValidSpacing for SpacingsConfig {
    fn from(&self, element: &dyn LayoutableElement) -> u32 {
        self.from_spacing(element.get_spacing())
    }
    fn from_spacing(&self, spacing: Spacing) -> u32 {
        match spacing {
            Spacing::None => 0,
            Spacing::Small => valid_spacing(self.small),
            Spacing::Medium => valid_spacing(self.medium),
            Spacing::Large => valid_spacing(self.large),
            Spacing::ExtraLarge => valid_spacing(self.extra_large),
            Spacing::Padding => valid_spacing(self.padding),
            Spacing::Default => valid_spacing(self.default),
        }
    }

    fn padding(&self) -> u32 {
        valid_spacing(self.default)
    }
}

fn valid_spacing(spacing: i64) -> u32 {
    if spacing < 0 {
        0
    } else {
        spacing as u32
    }
}

// impl generated::HostConfig {
//     pub fn separator(&self) -> generated::SeparatorConfig {
//         self.separator.clone().unwrap_or_else(|| {
//             generated::builder::SeparatorConfig::default()
//                 .try_into()
//                 .expect("Default separator config should be valid")
//         })
//     }
//     pub fn separator_line_thickness(&self) -> Result<u32, RenderError> {
//         let maybe_line_thickness = self.separator().line_thickness;
//         maybe_line_thickness
//             .try_into()
//             .map_err(|_| RenderError::HostConfig {
//                 path: "separator.lineThickness".to_string(),
//                 message: format!("Failed to parse {} as u32", maybe_line_thickness),
//             })
//     }

//     pub fn separator_line_color(&self) -> Result<Rgba<u8>, RenderError> {
//         let line_color = self
//             .separator()
//             .line_color
//             .expect("separator.lineColor should be set");

//         parse_color(&line_color).map_err(|_| RenderError::HostConfig {
//             path: "separator.lineColor".to_string(),
//             message: format!("Failed to parse color {}", line_color),
//         })
//     }

//     // pub fn container_styles(&self) -> generated::ContainerStylesConfig {
//     //     self.container_styles.clone().unwrap_or_else(|| {
//     //         generated::builder::ContainerStylesConfig::default()
//     //             .try_into()
//     //             .expect("Default container styles config should be valid")
//     //     })
//     // }
// }

pub(super) trait StringToColor {
    fn to_color(&self) -> Result<image::Rgba<u8>, RenderError>;
}

impl StringToColor for String {
    fn to_color(&self) -> Result<image::Rgba<u8>, RenderError> {
        parse_color_map_error(self)
    }
}

impl StringToColor for &str {
    fn to_color(&self) -> Result<image::Rgba<u8>, RenderError> {
        parse_color_map_error(self)
    }
}

impl StringToColor for Option<String> {
    fn to_color(&self) -> Result<image::Rgba<u8>, RenderError> {
        match self {
            Some(color) => parse_color_map_error(color),
            None => Ok(image::Rgba([0, 0, 0, 0])),
        }
    }
}

fn parse_color(input: &str) -> Result<image::Rgba<u8>, ParseColorError> {
    let c = csscolorparser::parse(input)?;
    Ok(image::Rgba(c.to_rgba8()))
}

fn parse_color_map_error(input: &str) -> Result<image::Rgba<u8>, RenderError> {
    parse_color(input).map_err(|_| RenderError::HostConfig {
        message: format!("Failed to parse color: {}", input),
    })
}
