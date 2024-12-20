use std::{cell::RefCell, rc::Rc};

use crate::{
    element_layout_data::{ElementTaffyData, Placement},
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::ValidSpacing,
    is_transparent,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::Layoutable,
    masked_image::MaskedImage,
    utils::{ClampToU32, TaffyLayoutUtils},
};
use adaptive_cards::{
    BlockElementHeight, BlockElementWidth, ContainerStyle, SizedStackableToggleable,
    StackableToggleable, StringOrBlockElementHeight, StringOrBlockElementWidthOrNumber,
    WidthOrHeight,
};
use adaptive_cards_host_config::HostConfig;
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_line_segment_mut};
use taffy::{prelude::length, Dimension, Display, FlexDirection, Rect, Size, Style, TaffyTree};

use super::{
    utils::{
        get_margins_for_bleed, parse_dimension, vertical_content_alignment_to_justify_content,
    },
    ItemsContainer, ItemsContainerOrientation,
};

pub(super) fn container_layout_override<
    TParent: ItemsContainer<TItem>,
    TItem: StackableToggleable + Layoutable + SizedContainerItem,
>(
    parent: &TParent,
    context: &LayoutContext,
    baseline_style: taffy::Style,
    tree: &mut TaffyTree<NodeContext>,
    scratch: &mut LayoutScratch,
) -> Result<ElementTaffyData, RenderError> {
    let mut style = baseline_style;

    // Parse the min height, if specified.
    if let Some(min_height) = parent.min_height() {
        style.min_size.height = parse_dimension(min_height, context)?
    };

    // If the container is set to bleed, we need to add appropriate margins to the baseline style.
    if parent.bleed() {
        let placement = parent.placement(); //parent.layout_data.borrow().placement();
        style.margin = get_margins_for_bleed(&placement, context.host_config);
    }

    // Create the child context.
    let child_items_context = parent.apply_child_items_layout_context(
        context
            .for_child_str(parent.children_collection_name())
            .with_vertical_content_alignment(parent.vertical_content_alignment())
            .with_horizontal_content_alignment(parent.horizontal_content_alignment())
            .with_style(parent.style()),
    );

    // Get the children.
    let child_items = parent
        .children()
        .iter()
        .filter(|c| c.is_visible())
        .collect::<Vec<_>>();

    // This will contain one node id for each child, in the same order as the children array.
    let mut child_item_node_ids = Vec::new();

    // This will contain the complete set of child node ids, including decorative items like separators.
    let mut child_node_ids = Vec::new();

    // If any of the child items have a separator, we need to know the line thickness we should draw.
    let separator_line_thickness = parent.separator_thickness(context.host_config);

    // Used to determine if we're drawing the first or last child item.
    let item_count = child_items.len();

    // Get the sum of all the weighted sizes of the child items.
    let sum_of_weighted = child_items
        .iter()
        .fold(0., |acc, item| acc + item.weighted_size());

    // For each child item...
    for (index, &child) in child_items.iter().enumerate() {
        // Determine the placement of the item within the container relative to any siblings.
        let item_position = match parent.orientation() {
            ItemsContainerOrientation::Vertical => match index {
                0 if item_count == 1 => Placement::SoleVertical,
                0 => Placement::Top,
                i if i == item_count - 1 => Placement::Bottom,
                _ => Placement::WithinVertical,
            },
            ItemsContainerOrientation::Horizontal => match index {
                0 if item_count == 1 => Placement::SoleHorizontal,
                0 => Placement::Left,
                i if i == item_count - 1 => Placement::Right,
                _ => Placement::WithinHorizontal,
            },
        };

        // Save the placement to the item's layout data so we can use it when drawing the item.
        child.layout_data().borrow_mut().placement = Some(item_position);

        // If the item has a separator, we need to add some spacing as defined by
        // the host config, plus additional spacing for the separator line thickness.
        let has_separator = child.separator();
        let spacing = context.host_config.spacing.from(child)
            + match has_separator {
                true => separator_line_thickness,
                false => 0,
            };

        // If the item has any spacing, add a node to the Taffy tree to represent it.
        if spacing > 0 {
            match item_position {
                Placement::Bottom | Placement::WithinVertical => {
                    let spacer_style = Style {
                        size: Size {
                            height: length(spacing as f32),
                            width: Dimension::Auto,
                        },
                        ..Style::default()
                    };
                    let spacer_node_id = tree.new_leaf(spacer_style).err_context(context)?;
                    child_node_ids.push(spacer_node_id);
                }
                Placement::Right | Placement::WithinHorizontal => {
                    let spacer_style = Style {
                        size: Size {
                            height: Dimension::Auto,
                            width: length(spacing as f32),
                        },
                        ..Style::default()
                    };
                    let spacer_node_id = tree.new_leaf(spacer_style).err_context(context)?;
                    child_node_ids.push(spacer_node_id);
                }
                _ => {}
            }
        }

        // Create a context for the child item.
        let item_context = child_items_context.for_child(index.to_string());

        // Create a baseline style for the child item, which we will build upon.
        let mut item_baseline_style = Style::default();

        // Apply the height of the item to the style.
        child.apply_size_to_style(&mut item_baseline_style, &item_context, sum_of_weighted)?;

        // Call `layout` on the child item, which returns its node id in the Taffy tree.
        let item_node_id = child.layout(&item_context, item_baseline_style, tree, scratch)?;

        // Add the node id to the child_item_node_ids array so it can be used in the
        // draw pass to fetch the child item's final position.
        child_item_node_ids.push(item_node_id);

        child_node_ids.push(item_node_id);
    }

    // Next we build up the container style based on the host config and item properties.
    apply_container_style_padding(parent, context.host_config, &mut style);

    // Use the vertical content alignment (which was populated by the caller of this function)
    // to determine the flexbox justify content property.
    let justify_content = vertical_content_alignment_to_justify_content(
        child_items_context.inherited.vertical_content_alignment,
    );

    style.display = Display::Flex;

    style.flex_direction = match parent.orientation() {
        ItemsContainerOrientation::Horizontal => FlexDirection::Row,
        ItemsContainerOrientation::Vertical => FlexDirection::Column,
    };

    style.justify_content = Some(justify_content);

    // Finally add ourself to the taffy tree and return the node id other metadata.
    tree.new_with_children(style, &child_node_ids)
        .err_context(context)
        .map(|node_id| ElementTaffyData {
            node_id,
            child_item_node_ids,
        })
}

pub(super) enum PaddingBehavior {
    None,
    Narrow,
    Default,
    Style,
}

/// Containers only have padding if they are not the default style (and so have a background color).
fn apply_container_style_padding<
    TParent: ItemsContainer<TItem>,
    TItem: StackableToggleable + Layoutable + SizedContainerItem,
>(
    parent: &TParent,
    host_config: &HostConfig,
    baseline_style: &mut Style,
) {
    let additional_padding = match parent.padding_behavior() {
        PaddingBehavior::None => 0,
        PaddingBehavior::Narrow => host_config.spacing.small.clamp_to_u32(),
        PaddingBehavior::Default => host_config.spacing.padding(),
        PaddingBehavior::Style => match parent.style() {
            None => 0,
            Some(style) => {
                if style == ContainerStyle::Default {
                    0
                } else {
                    host_config.spacing.padding()
                }
            }
        },
    };

    let border_thickness = parent.border_thickness(host_config);

    let total_padding = (border_thickness + additional_padding) as f32;

    baseline_style.padding = Rect {
        top: length(total_padding),
        left: length(total_padding),
        right: length(total_padding),
        bottom: length(total_padding),
    };
}

pub(super) trait SizedContainerItem {
    fn weighted_size(&self) -> f64;

    fn apply_size_to_style(
        &self,
        style: &mut Style,
        context: &LayoutContext,
        sum_of_weighted: f64,
    ) -> Result<(), RenderError>;
}

impl<T: SizedStackableToggleable> SizedContainerItem for T {
    fn weighted_size(&self) -> f64 {
        match self.width_or_height() {
            WidthOrHeight::Width(width) => match width {
                StringOrBlockElementWidthOrNumber::Number(width) => width,
                _ => 0.,
            },
            WidthOrHeight::Height(_) => 0.,
        }
    }

    fn apply_size_to_style(
        &self,
        style: &mut Style,
        context: &LayoutContext,
        sum_of_weighted: f64,
    ) -> Result<(), RenderError> {
        match self.width_or_height() {
            WidthOrHeight::Width(width) => match width {
                StringOrBlockElementWidthOrNumber::BlockElementWidth(width) => match width {
                    BlockElementWidth::Auto => {
                        // Matches AC's web auto behavior.
                        style.flex_basis = Dimension::Auto;
                        style.flex_grow = 0.;
                        style.flex_shrink = 1.;
                    }
                    BlockElementWidth::Stretch => {
                        // Matches AC's web stretch behavior. Ensures they distribute evenly.
                        style.flex_basis = Dimension::Length(50.);
                        style.flex_grow = 1.;
                        style.flex_shrink = 1.;
                    }
                },
                StringOrBlockElementWidthOrNumber::String(width) => {
                    style.size.width = parse_dimension(&width, context)?;
                    style.min_size.width = style.size.width; // Set as otherwise width can be smaller than set width.
                }
                StringOrBlockElementWidthOrNumber::Number(width) => {
                    // Matches AC's web weighted behavior.
                    style.flex_basis = Dimension::Percent((width / sum_of_weighted) as f32);
                    style.flex_grow = 1.;
                    style.flex_shrink = 1.;
                }
            },

            WidthOrHeight::Height(height) => match height {
                StringOrBlockElementHeight::String(height) => {
                    style.size.height = parse_dimension(&height, context)?;
                    style.min_size.height = style.size.height;
                }
                StringOrBlockElementHeight::BlockElementHeight(height) => match height {
                    BlockElementHeight::Auto => {
                        // Matches AC's web auto behavior.
                        style.flex_basis = Dimension::Auto;
                        style.flex_grow = 0.;
                        style.flex_shrink = 0.;
                    }
                    BlockElementHeight::Stretch => {
                        // Matches AC's web stretch behavior.
                        style.flex_basis = Dimension::Auto;
                        style.flex_grow = 1.;
                        style.flex_shrink = 1.;
                    }
                },
            },
        };

        Ok(())
    }
}

// The shared container draw logic for AdaptiveCard and Container.
pub(super) fn container_draw_override<
    TParent: ItemsContainer<TItem>,
    TItem: StackableToggleable + Layoutable,
>(
    parent: &TParent,
    context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    taffy_data: &ElementTaffyData,
    image: Rc<RefCell<MaskedImage>>,
    scratch: &mut LayoutScratch,
) -> Result<(), RenderError> {
    // Draw the container background, if necessary.
    draw_background(parent, context, tree, taffy_data, &image)?;

    // Create the child context.
    let child_items_context = context.for_child_str(parent.children_collection_name());

    // Get the child items.
    let child_items = parent
        .children()
        .iter()
        .filter(|c| c.is_visible())
        .collect::<Vec<_>>();

    // Fetch our calculated layout data from the Taffy tree, and find our absolute rectangle
    // where we need to draw ourselves.
    let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
    let absolute_rect = node_layout.absolute_rect(context);

    // Fetch the separator properties from the host config, in case we need to draw any.
    let separator_line_thickness = parent.separator_thickness(context.host_config);
    let separator_color = parent.separator_color(context.host_config)?;

    // Fetch the child item node ids from the layout data, so we can match them
    // to the child items array.
    let child_item_node_ids = &taffy_data.child_item_node_ids;

    // For each visible child item...
    for (i, &child) in child_items.iter().enumerate() {
        // Get the item's calculated layout data from the Taffy tree.
        let item_layout = tree
            .layout(child_item_node_ids[i])
            .err_context(&child_items_context)?;

        // Create a context for the child item.
        let item_context =
            child_items_context.for_child_origin(i.to_string(), item_layout.location);

        // And get the item's absolute rectangle.
        let item_rect = item_layout.absolute_rect(&item_context);

        let item_placement = child
            .layout_data()
            .borrow()
            .placement
            .expect("Element placement not set");

        // Draw the separator, if necessary.
        match item_placement {
            Placement::Top
            | Placement::Bottom
            | Placement::SoleVertical
            | Placement::WithinVertical => {
                draw_horizontal_separator(
                    context,
                    i,
                    item_rect,
                    child,
                    separator_line_thickness,
                    separator_color,
                    &image,
                );
            }
            Placement::Left
            | Placement::Right
            | Placement::SoleHorizontal
            | Placement::WithinHorizontal => {
                draw_vertical_separator(
                    context,
                    i,
                    item_rect,
                    child,
                    separator_line_thickness,
                    separator_color,
                    &image,
                );
            }
        }

        // Calculate the intersection of the item's rectangle with the container's rectangle.
        let maybe_intersection = absolute_rect.intersect(item_rect);

        if maybe_intersection.is_none() && !context.debug_mode.transparent_masks {
            // If there is no overlap, we can skip drawing the item.
            // Unless we're in the debug mode which specifies transparent masks.
            continue;
        };

        // Call `draw` on the child item.
        child.draw(item_context, tree, Rc::clone(&image), scratch)?;
    }
    Ok(())
}

/// Draws a horizontal separator line between child items, if the child item has
/// its separator property set to true.
fn draw_horizontal_separator<TItem: StackableToggleable + Layoutable>(
    context: &LayoutContext,
    item_index: usize,
    item_rect: imageproc::rect::Rect,
    item: &TItem,
    separator_line_thickness: u32,
    separator_color: image::Rgba<u8>,
    image: &Rc<RefCell<MaskedImage>>,
) {
    let has_separator = item.separator();
    if has_separator && item_index > 0 {
        let spacing = context.host_config.spacing.from(item);
        let half_spacing = (spacing / 2) as f32;

        // The bottom of the horizontal line will be half the separator spacing above the top of the item,
        // minus an additional pixel.
        let y_bottom_float = item_rect.top() as f32 - half_spacing - 1.;

        // The top of the horizontal line is going to be half the spacing above the top of the item,
        // minus the line thickness.
        let y_top_float = y_bottom_float - separator_line_thickness as f32 + 1.;

        let x_start = item_rect.left() as f32;
        let x_end = item_rect.right() as f32;

        let mut image_mut = image.borrow_mut();

        // Draw the top and bottom lines, which may be between pixels.
        draw_line_segment_mut(
            &mut *image_mut,
            (x_start, y_top_float),
            (x_end, y_top_float),
            separator_color,
        );

        if separator_line_thickness > 1 {
            draw_line_segment_mut(
                &mut *image_mut,
                (x_start, y_bottom_float),
                (x_end, y_bottom_float),
                separator_color,
            );

            // Now draw lines for all the pixels in between the top and bottom.
            let y_bottom = y_bottom_float.ceil() as u32;
            let y_top = y_top_float.floor() as u32;
            for y in y_top..=y_bottom {
                draw_line_segment_mut(
                    &mut *image_mut,
                    (x_start, y as f32),
                    (x_end, y as f32),
                    separator_color,
                );
            }
        }
    }
}

/// Draws a vertical separator line between child items, if the child item has
/// its separator property set to true.
fn draw_vertical_separator<TItem: StackableToggleable + Layoutable>(
    context: &LayoutContext,
    item_index: usize,
    item_rect: imageproc::rect::Rect,
    item: &TItem,
    separator_line_thickness: u32,
    separator_color: image::Rgba<u8>,
    image: &Rc<RefCell<MaskedImage>>,
) {
    let has_separator = item.separator();
    if has_separator && item_index > 0 {
        let spacing = context.host_config.spacing.from(item);
        let half_spacing = (spacing / 2) as f32;

        // The right of the horizontal line will be half the separator spacing to the left of the item,
        // minus an additional pixel.
        let x_right_float = item_rect.left() as f32 - half_spacing - 1.;

        // The left of the horizontal line is going to be half the spacing to the left of the item,
        // minus the line thickness.
        let x_left_float = x_right_float - separator_line_thickness as f32 + 1.;

        let y_start = item_rect.top() as f32;
        let y_end = item_rect.bottom() as f32;

        let mut image_mut = image.borrow_mut();

        // Draw the left and right lines, which max be between pixels.
        draw_line_segment_mut(
            &mut *image_mut,
            (x_left_float, y_start),
            (x_left_float, y_end),
            separator_color,
        );

        if separator_line_thickness > 1 {
            draw_line_segment_mut(
                &mut *image_mut,
                (x_right_float, y_start),
                (x_right_float, y_end),
                separator_color,
            );

            // Now draw lines for all the pixels in between the left and right.
            let x_right = x_right_float.ceil() as u32;
            let x_left = x_left_float.floor() as u32;
            for x in x_left..=x_right {
                draw_line_segment_mut(
                    &mut *image_mut,
                    (x as f32, y_start),
                    (x as f32, y_end),
                    separator_color,
                );
            }
        }
    }
}

pub fn draw_background<TParent: ItemsContainer<TItem>, TItem: StackableToggleable + Layoutable>(
    parent: &TParent,
    context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    taffy_data: &ElementTaffyData,
    image: &Rc<RefCell<MaskedImage>>,
) -> Result<(), RenderError> {
    // Get our absolute rectangle.
    let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
    let rect = node_layout.absolute_rect(context);

    let mut image_mut = image.borrow_mut();

    // If the style has a background color, use it to draw a rectangle over our absolute rect.
    let background_color = parent.background_color(context.host_config)?;
    if !is_transparent(background_color) {
        draw_filled_rect_mut(&mut *image_mut, rect, background_color);
    }

    // Technically, we shouldn't draw a border here:
    // https://github.com/microsoft/AdaptiveCards/blob/15418ce93b452dd0858415db40ddba05cd154c73/specs/features/Tables.md?plain=1#L65-L91
    // The border color property is technically, and unintuitively, supposed to be used
    // with the "gridStyle" property on a table to color the table grid lines.
    // We're going to deviate from the official Adaptive Cards behavior here to do the
    // intuitive thing and use the border color to draw borders.
    let border_thickness = parent.border_thickness(context.host_config);
    if border_thickness > 0 {
        let border_color = parent.border_color(context.host_config)?;
        if !is_transparent(border_color) {
            let mut current_rect = rect;
            for _ in 0..border_thickness {
                draw_hollow_rect_mut(&mut *image_mut, current_rect, border_color);
                current_rect =
                    imageproc::rect::Rect::at(current_rect.left() + 1, current_rect.top() + 1)
                        .of_size(current_rect.width() - 2, current_rect.height() - 2);
            }
        }
    }

    Ok(())
}
