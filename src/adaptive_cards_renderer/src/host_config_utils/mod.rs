use adaptive_cards_host_config::{
    ContainerStyleConfig, ContainerStylesConfig, FontColorConfig, FontSizesConfig, FontTypeConfig,
    FontTypesConfig, FontWeightsConfig, ForegroundColorsConfig, SpacingsConfig, TextStyleConfig,
    TextStyleConfigColor, TextStyleConfigFontType, TextStyleConfigSize, TextStyleConfigWeight,
    TextStylesConfig,
};
use csscolorparser::ParseColorError;

use adaptive_cards::{
    Colors, ContainerStyle, FontSize, FontType, FontWeight, Spacing, StackableToggleable,
    TextBlockStyle,
};

use crate::{TRANSPARENT, errors::RenderError, premultiplied_alpha::pixel_to_premultiplied_alpha};

pub mod default_host_config;

pub(super) trait ValidSpacing {
    fn from(&self, element: &dyn StackableToggleable) -> u32;

    #[allow(clippy::wrong_self_convention)]
    fn from_spacing(&self, spacing: Spacing) -> u32;

    fn padding(&self) -> u32;
}

impl ValidSpacing for SpacingsConfig {
    fn from(&self, element: &dyn StackableToggleable) -> u32 {
        self.from_spacing(element.spacing())
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
        valid_spacing(self.padding)
    }
}

fn valid_spacing(spacing: i64) -> u32 {
    if spacing < 0 { 0 } else { spacing as u32 }
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
            None => Ok(TRANSPARENT),
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

    csscolorparser::parse(input).map(|c| pixel_to_premultiplied_alpha(image::Rgba(c.to_rgba8())))
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

pub(super) trait ForegroundColorToConfig {
    fn from(&self, color: Colors) -> &FontColorConfig;
}

impl ForegroundColorToConfig for ForegroundColorsConfig {
    fn from(&self, color: Colors) -> &FontColorConfig {
        match color {
            Colors::Default => &self.default,
            Colors::Dark => &self.dark,
            Colors::Light => &self.light,
            Colors::Accent => &self.accent,
            Colors::Good => &self.good,
            Colors::Warning => &self.warning,
            Colors::Attention => &self.attention,
        }
    }
}

pub(super) trait FontTypeToConfig {
    fn from(&self, font_type: FontType) -> &FontTypeConfig;
}

impl FontTypeToConfig for FontTypesConfig {
    fn from(&self, font_type: FontType) -> &FontTypeConfig {
        match font_type {
            FontType::Default => &self.default,
            FontType::Monospace => &self.monospace,
        }
    }
}

pub(super) trait FontSizeToNumber {
    fn from(&self, font_size: FontSize) -> i64;
}

impl FontSizeToNumber for FontSizesConfig {
    fn from(&self, font_size: FontSize) -> i64 {
        match font_size {
            FontSize::Small => self.small,
            FontSize::Default => self.default,
            FontSize::Medium => self.medium,
            FontSize::Large => self.large,
            FontSize::ExtraLarge => self.extra_large,
        }
    }
}

pub(super) trait FontWeightToNumber {
    fn from(&self, font_weight: FontWeight) -> i64;
}

impl FontWeightToNumber for FontWeightsConfig {
    fn from(&self, font_weight: FontWeight) -> i64 {
        match font_weight {
            FontWeight::Lighter => self.lighter,
            FontWeight::Default => self.default,
            FontWeight::Bolder => self.bolder,
        }
    }
}

pub(super) trait TextStyleToConfig {
    fn from(&self, text_style: TextBlockStyle) -> Option<&TextStyleConfig>;
}

impl TextStyleToConfig for TextStylesConfig {
    fn from(&self, text_style: TextBlockStyle) -> Option<&TextStyleConfig> {
        match text_style {
            TextBlockStyle::Default => None,
            TextBlockStyle::Heading => Some(&self.heading),
        }
    }
}

// Converting from the color in the host config "text style" to the one in an adaptive card text block.
pub(super) trait ToAdaptiveCardsColor {
    fn to_ac_color(&self) -> Colors;
}

impl ToAdaptiveCardsColor for Option<&TextStyleConfig> {
    fn to_ac_color(&self) -> Colors {
        match self {
            None => Colors::Default,
            Some(inner) => inner.to_ac_color(),
        }
    }
}

impl ToAdaptiveCardsColor for TextStyleConfig {
    fn to_ac_color(&self) -> Colors {
        self.color.to_ac_color()
    }
}

impl ToAdaptiveCardsColor for TextStyleConfigColor {
    fn to_ac_color(&self) -> Colors {
        match self {
            TextStyleConfigColor::Default => Colors::Default,
            TextStyleConfigColor::Dark => Colors::Dark,
            TextStyleConfigColor::Light => Colors::Light,
            TextStyleConfigColor::Accent => Colors::Accent,
            TextStyleConfigColor::Good => Colors::Good,
            TextStyleConfigColor::Warning => Colors::Warning,
            TextStyleConfigColor::Attention => Colors::Attention,
        }
    }
}

// Converting from the font size in the host config "text style" to the one in an adaptive card text block.
pub(super) trait ToAdaptiveCardsFontSize {
    fn to_ac_size(&self) -> FontSize;
}

impl ToAdaptiveCardsFontSize for Option<&TextStyleConfig> {
    fn to_ac_size(&self) -> FontSize {
        match self {
            None => FontSize::Default,
            Some(inner) => inner.to_ac_size(),
        }
    }
}

impl ToAdaptiveCardsFontSize for TextStyleConfig {
    fn to_ac_size(&self) -> FontSize {
        self.size.to_ac_size()
    }
}

impl ToAdaptiveCardsFontSize for TextStyleConfigSize {
    fn to_ac_size(&self) -> FontSize {
        match self {
            TextStyleConfigSize::Small => FontSize::Small,
            TextStyleConfigSize::Default => FontSize::Default,
            TextStyleConfigSize::Medium => FontSize::Medium,
            TextStyleConfigSize::Large => FontSize::Large,
            TextStyleConfigSize::ExtraLarge => FontSize::ExtraLarge,
        }
    }
}

// Converting from the font weight in the host config "text style" to the one in an adaptive card text block.
pub(super) trait ToAdaptiveCardsFontWeight {
    fn to_ac_weight(&self) -> FontWeight;
}

impl ToAdaptiveCardsFontWeight for Option<&TextStyleConfig> {
    fn to_ac_weight(&self) -> FontWeight {
        match self {
            None => FontWeight::Default,
            Some(inner) => inner.to_ac_weight(),
        }
    }
}

impl ToAdaptiveCardsFontWeight for TextStyleConfig {
    fn to_ac_weight(&self) -> FontWeight {
        self.weight.to_ac_weight()
    }
}

impl ToAdaptiveCardsFontWeight for TextStyleConfigWeight {
    fn to_ac_weight(&self) -> FontWeight {
        match self {
            TextStyleConfigWeight::Lighter => FontWeight::Lighter,
            TextStyleConfigWeight::Default => FontWeight::Default,
            TextStyleConfigWeight::Bolder => FontWeight::Bolder,
        }
    }
}

// Converting from the font type in the host config "text style" to the one in an adaptive card text block.
pub(super) trait ToAdaptiveCardsFontType {
    fn to_ac_type(&self) -> FontType;
}

impl ToAdaptiveCardsFontType for Option<&TextStyleConfig> {
    fn to_ac_type(&self) -> FontType {
        match self {
            None => FontType::Default,
            Some(inner) => inner.to_ac_type(),
        }
    }
}

impl ToAdaptiveCardsFontType for TextStyleConfig {
    fn to_ac_type(&self) -> FontType {
        self.font_type.to_ac_type()
    }
}

impl ToAdaptiveCardsFontType for TextStyleConfigFontType {
    fn to_ac_type(&self) -> FontType {
        match self {
            TextStyleConfigFontType::Default => FontType::Default,
            TextStyleConfigFontType::Monospace => FontType::Monospace,
        }
    }
}
