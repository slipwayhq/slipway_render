use std::num::ParseFloatError;

use taffy::{Dimension, Size};

use crate::{
    element::LayoutableElement, errors::RenderError, host_config::generated::SpacingsConfig,
    layout_context::LayoutContext, Spacing,
};

mod adaptive_card;
mod container;
mod text_block;

trait ValidSpacing {
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

fn parse_dimension(input: &str, context: &LayoutContext) -> Result<Dimension, RenderError> {
    fn inner(input: &str) -> Result<Dimension, ParseFloatError> {
        if let Some(input) = input.strip_suffix("px") {
            Ok(Dimension::Length(input.parse::<f32>()?))
        } else if let Some(input) = input.strip_suffix('%') {
            Ok(Dimension::Percent(input.parse::<f32>()?))
        } else {
            Ok(Dimension::Length(input.parse::<f32>()?))
        }
    }

    let result = inner(input);

    result.map_err(|_| RenderError::Other {
        path: context.path.clone(),
        message: format!("Failed to parse as dimension: {}", input),
    })
}

pub(super) enum NodeContext {
    Text,
}

pub(super) fn measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    _available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    node_context: Option<&mut NodeContext>,
) -> Size<f32> {
    if let Size {
        width: Some(width),
        height: Some(height),
    } = known_dimensions
    {
        return Size { width, height };
    }

    match node_context {
        None => Size::ZERO,
        Some(NodeContext::Text) => Size {
            width: 100.,
            height: 10.,
        },
    }
}
