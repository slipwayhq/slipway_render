use adaptive_cards_host_config::{ContainerStyleConfig, ContainerStylesConfig, SpacingsConfig};
use csscolorparser::ParseColorError;

use crate::{
    adaptive_cards::ContainerStyle, adaptive_cards::Spacing, element::LayoutableElement,
    errors::RenderError,
};

pub mod default_host_config;

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
    if let Some(s) = input.strip_prefix('#') {
        if s.len() == 4 {
            // Swap characters 0 and 3 so ARGB becomes RGBA.
            let mut chars: Vec<char> = s.chars().collect();
            chars.swap(0, 3);
            let rgba_string: String = chars.into_iter().collect();
            return csscolorparser::parse(&rgba_string).map(|c| image::Rgba(c.to_rgba8()));
        } else if s.len() == 8 {
            // Swap characters 0 and 1 with characters 6 and 7 so ARGB becomes RGBA.
            let mut chars: Vec<char> = s.chars().collect();
            chars.swap(0, 6);
            chars.swap(1, 7);
            let rgba_string: String = chars.into_iter().collect();
            return csscolorparser::parse(&rgba_string).map(|c| image::Rgba(c.to_rgba8()));
        }
    }

    csscolorparser::parse(input).map(|c| image::Rgba(c.to_rgba8()))
}

fn parse_color_map_error(input: &str) -> Result<image::Rgba<u8>, RenderError> {
    parse_color(input).map_err(|_| RenderError::HostConfig {
        message: format!("Failed to parse color: {}", input),
    })
}

pub(super) trait ContainerStyleToConfig {
    fn from(&self, style: ContainerStyle) -> &ContainerStyleConfig;
}

impl ContainerStyleToConfig for ContainerStylesConfig {
    fn from(&self, style: ContainerStyle) -> &ContainerStyleConfig {
        match style {
            ContainerStyle::Default => &self.default,
            ContainerStyle::Emphasis => &self.emphasis,
            ContainerStyle::Good => &self.good,
            ContainerStyle::Warning => &self.warning,
            ContainerStyle::Attention => &self.attention,
            ContainerStyle::Accent => &self.accent,
        }
    }
}
