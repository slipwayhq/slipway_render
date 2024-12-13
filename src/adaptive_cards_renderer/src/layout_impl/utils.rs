use std::{cell::RefCell, num::ParseFloatError, rc::Rc};

use adaptive_cards::{ContainerStyle, VerticalContentAlignment};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut};
use taffy::{
    prelude::length, Dimension, JustifyContent, LengthPercentageAuto, Rect, Style, TaffyTree,
};

use crate::{
    element_layout_data::{ElementTaffyData, Placement},
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::{ContainerStyleToConfig, StringToColor, ValidSpacing},
    layout_context::LayoutContext,
    masked_image::MaskedImage,
    utils::TaffyLayoutUtils,
};

use adaptive_cards_host_config::HostConfig;

use super::measure::NodeContext;

/// Parse a string as a dimension.
pub(super) fn parse_dimension(
    input: &str,
    context: &LayoutContext,
) -> Result<Dimension, RenderError> {
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

/// Get the margins for elements that have specified they should bleed, based on their
/// placement relative to their siblings.
pub(super) fn get_margins_for_bleed(
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

pub(super) fn apply_horizontal_alignment(
    horizontal_alignment: Option<adaptive_cards::HorizontalAlignment>,
    style: &mut Style,
    context: &LayoutContext,
) {
    let horizontal_alignment =
        horizontal_alignment.unwrap_or(context.inherited.horizontal_alignment);

    style.justify_content = Some(match horizontal_alignment {
        adaptive_cards::HorizontalAlignment::Center => taffy::style::JustifyContent::Center,
        adaptive_cards::HorizontalAlignment::Right => taffy::style::JustifyContent::FlexEnd,
        adaptive_cards::HorizontalAlignment::Left => taffy::style::JustifyContent::FlexStart,
    });
}

// Converts the VerticalContentAlignment as set on the container element to a JustifyContent property
// as required by the layout flexbox.
pub(super) fn vertical_content_alignment_to_justify_content(
    vertical_content_alignment: VerticalContentAlignment,
) -> JustifyContent {
    match vertical_content_alignment {
        VerticalContentAlignment::Top => JustifyContent::FlexStart,
        VerticalContentAlignment::Center => JustifyContent::Center,
        VerticalContentAlignment::Bottom => JustifyContent::FlexEnd,
    }
}

pub(super) fn draw_background(
    style: ContainerStyle,
    context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    taffy_data: &ElementTaffyData,
    image: &Rc<RefCell<MaskedImage>>,
) -> Result<(), RenderError> {
    // Get the config for the style specified on the container.
    let style_config = context.host_config.container_styles.from(style);

    // Get our absolute rectangle.
    let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
    let rect = node_layout.absolute_rect(context);

    let mut image_mut = image.borrow_mut();

    // If the style has a background color, use it to draw a rectangle over our absolute rect.
    if let Some(background_color_str) = style_config.background_color.as_ref() {
        let background_color = background_color_str.to_color()?;
        draw_filled_rect_mut(&mut *image_mut, rect, background_color);
    }

    // Same for the boarder.
    // Technically, we shouldn't draw a border here:
    // https://github.com/microsoft/AdaptiveCards/blob/15418ce93b452dd0858415db40ddba05cd154c73/specs/features/Tables.md?plain=1#L65-L91
    // The border color property is technically, and unintuitively, supposed to be used
    // with the "gridStyle" property on a table to color the table grid lines.
    // We're going to deviate from the official Adaptive Cards behavior here to do the
    // intuitive thing and use the border color to draw borders.
    if let Some(border_color_str) = style_config.border_color.as_ref() {
        let border_color = border_color_str.to_color()?;
        draw_hollow_rect_mut(&mut *image_mut, rect, border_color);
    }

    Ok(())
}
