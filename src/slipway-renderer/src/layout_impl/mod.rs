use std::num::ParseFloatError;

use taffy::{prelude::length, Dimension, LengthPercentageAuto, Rect, Size};

use crate::{
    element_layout_data::Placement,
    errors::RenderError,
    host_config::{generated::HostConfig, ValidSpacing},
    layout_context::LayoutContext,
};

mod adaptive_card;
mod container;
mod container_shared;
mod text_block;

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

fn get_margins_for_bleed(
    placement: &Placement,
    host_config: &HostConfig,
) -> Rect<LengthPercentageAuto> {
    let negative_padding = -1. * host_config.spacing.padding() as f32;

    // Note, this currently does not render as desired, I think because of an issue in Taffy:
    // https://github.com/DioxusLabs/taffy/issues/706
    match placement {
        Placement::Top => Rect {
            top: length(negative_padding),
            left: length(negative_padding),
            right: length(negative_padding),
            bottom: length(0.),
        },
        Placement::Bottom => Rect {
            top: length(0.),
            left: length(negative_padding),
            right: length(negative_padding),
            bottom: length(negative_padding),
        },
        Placement::SoleVertical => Rect {
            top: length(negative_padding),
            left: length(negative_padding),
            right: length(negative_padding),
            bottom: length(negative_padding),
        },
        Placement::WithinVertical => Rect {
            top: length(0.),
            left: length(negative_padding),
            right: length(negative_padding),
            bottom: length(0.),
        },
        Placement::Left => Rect {
            top: length(negative_padding),
            left: length(negative_padding),
            right: length(0.),
            bottom: length(negative_padding),
        },
        Placement::Right => Rect {
            top: length(negative_padding),
            left: length(0.),
            right: length(negative_padding),
            bottom: length(negative_padding),
        },
        Placement::SoleHorizontal => Rect {
            top: length(negative_padding),
            left: length(negative_padding),
            right: length(negative_padding),
            bottom: length(negative_padding),
        },
        Placement::WithinHorizontal => Rect {
            top: length(negative_padding),
            left: length(0.),
            right: length(0.),
            bottom: length(negative_padding),
        },
    }
}
