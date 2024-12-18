use std::{cell::RefCell, rc::Rc};

use crate::{
    debug_mode::next_color,
    element_layout_data::{ElementTaffyData, Placement},
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::{StringToColor, ValidSpacing},
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
use imageproc::drawing::{draw_hollow_rect_mut, draw_line_segment_mut};
use taffy::{prelude::length, Dimension, Display, FlexDirection, Rect, Size, Style, TaffyTree};

use super::{
    utils::{
        draw_background, get_margins_for_bleed, parse_dimension,
        vertical_content_alignment_to_justify_content,
    },
    ItemsContainer, ItemsContainerOrientation,
};

pub(super) fn container_layout_override<
    TParent: ItemsContainer<TItem>,
    TItem: StackableToggleable + Layoutable + SizedContainerItem,
>(
    parent: &TParent,
    context: &LayoutContext,
    mut baseline_style: taffy::Style,
    tree: &mut TaffyTree<NodeContext>,
) -> Result<ElementTaffyData, RenderError> {
    // Parse the min height, if specified.
    if let Some(min_height) = parent.get_min_height() {
        baseline_style.min_size.height = parse_dimension(min_height, context)?
    };

    // If the container is set to bleed, we need to add appropriate margins to the baseline style.
    if parent.get_bleed() {
        let placement = parent.get_placement(); //parent.layout_data.borrow().placement();
        baseline_style.margin = get_margins_for_bleed(&placement, context.host_config);
    }

    // Create the child context.
    let child_items_context = context
        .for_child_str(parent.get_children_collection_name())
        .with_vertical_content_alignment(parent.get_vertical_content_alignment())
        .with_horizontal_content_alignment(parent.get_horizontal_content_alignment())
        .with_style(parent.get_style());

    // Get the children.
    let child_items = parent
        .get_children()
        .iter()
        .filter(|c| c.get_is_visible())
        .collect::<Vec<_>>();

    // This will contain one node id for each child, in the same order as the children array.
    let mut child_item_node_ids = Vec::new();

    // This will contain the complete set of child node ids, including decorative items like separators.
    let mut child_node_ids = Vec::new();

    // If any of the child items have a separator, we need to know the line thickness we should draw.
    let separator_line_thickness = context.host_config.separator.line_thickness.clamp_to_u32();

    // Used to determine if we're drawing the first or last child item.
    let item_count = child_items.len();

    // Get the sum of all the weighted sizes of the child items.
    let sum_of_weighted = child_items
        .iter()
        .fold(0., |acc, item| acc + item.get_weighted_size());

    // For each child item...
    for (index, &child) in child_items.iter().enumerate() {
        // Determine the placement of the item within the container relative to any siblings.
        let item_position = match parent.get_orientation() {
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
        let has_separator = child.get_separator();
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
        let item_node_id = child.layout(&item_context, item_baseline_style, tree)?;

        // Add the node id to the child_item_node_ids array so it can be used in the
        // draw pass to fetch the child item's final position.
        child_item_node_ids.push(item_node_id);

        child_node_ids.push(item_node_id);
    }

    // Next we build up the container style based on the host config and item properties.
    apply_container_style_padding(
        parent.get_padding_behavior(),
        context.host_config,
        &mut baseline_style,
    );

    // Use the vertical content alignment (which was populated by the caller of this function)
    // to determine the flexbox justify content property.
    let justify_content = vertical_content_alignment_to_justify_content(
        child_items_context.inherited.vertical_content_alignment,
    );

    baseline_style.display = Display::Flex;

    baseline_style.flex_direction = match parent.get_orientation() {
        ItemsContainerOrientation::Horizontal => FlexDirection::Row,
        ItemsContainerOrientation::Vertical => FlexDirection::Column,
    };

    baseline_style.justify_content = Some(justify_content);

    // Finally add ourself to the taffy tree and return the node id other metadata.
    tree.new_with_children(baseline_style, &child_node_ids)
        .err_context(context)
        .map(|node_id| ElementTaffyData {
            node_id,
            child_item_node_ids,
        })
}

pub(super) enum PaddingBehavior {
    Always,
    ForStyle(Option<ContainerStyle>),
}

/// Containers only have padding if they are not the default style (and so have a background color).
pub(super) fn apply_container_style_padding(
    padding_behavior: PaddingBehavior,
    host_config: &HostConfig,
    baseline_style: &mut Style,
) {
    let apply_padding = match padding_behavior {
        PaddingBehavior::Always => true,
        PaddingBehavior::ForStyle(style) => match style {
            None => false,
            Some(style) => style != ContainerStyle::Default,
        },
    };

    if apply_padding {
        let padding = host_config.spacing.padding() as f32;
        baseline_style.padding = Rect {
            top: length(padding),
            left: length(padding),
            right: length(padding),
            bottom: length(padding),
        };
    }
}

pub(super) trait SizedContainerItem {
    fn get_weighted_size(&self) -> f64;

    fn apply_size_to_style(
        &self,
        style: &mut Style,
        context: &LayoutContext,
        sum_of_weighted: f64,
    ) -> Result<(), RenderError>;
}

impl<T: SizedStackableToggleable> SizedContainerItem for T {
    fn get_weighted_size(&self) -> f64 {
        match self.get_width_or_height() {
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
        match self.get_width_or_height() {
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
    if let Some(style) = parent.get_style() {
        draw_background(style, context, tree, taffy_data, &image)?;
    }

    // Create the child context.
    let child_items_context = context.for_child_str(parent.get_children_collection_name());

    // Get the child items.
    let child_items = parent
        .get_children()
        .iter()
        .filter(|c| c.get_is_visible())
        .collect::<Vec<_>>();

    // Fetch our calculated layout data from the Taffy tree, and find our absolute rectangle
    // where we need to draw ourselves.
    let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
    let absolute_rect = node_layout.absolute_rect(context);

    // If we should draw debug outlines, do so.
    if context.debug_mode.outlines {
        let color = next_color();
        let mut image_mut = image.borrow_mut();

        draw_hollow_rect_mut(&mut *image_mut, absolute_rect, color);
    }

    // Fetch the separator properties from the host config, in case we need to draw any.
    let separator_line_thickness = context.host_config.separator.line_thickness.clamp_to_u32();
    let separator_color = context.host_config.separator.line_color.to_color()?;

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

        // If there is no overlap we can technically skip drawing the item
        // unless we're in the debug mode which specifies transparent masks, in
        // which case we just create a dummy 1 pixel sized intersection so the item
        // is still drawn but completely masked out.
        let maybe_intersection = match maybe_intersection {
            Some(intersection) => Some(intersection),
            None => {
                if context.debug_mode.transparent_masks {
                    Some(imageproc::rect::Rect::at(0, 0).of_size(1, 1))
                } else {
                    None
                }
            }
        };

        let Some(intersection) = maybe_intersection else {
            // If there is no overlap, we can skip drawing the item.
            // We already account for the `transparent_masks` debug mode above.
            continue;
        };

        // If we're in the debug mode which specifies we should draw outlines, draw an outline
        // for the child item.
        if context.debug_mode.outlines {
            let color = next_color();
            let mut image_mut = image.borrow_mut();

            draw_hollow_rect_mut(&mut *image_mut, item_rect, color);
        }

        // Create the masked child image.
        let child_image = MaskedImage::from_mask(image.clone(), intersection);

        // Call `draw` on the child item.
        child.draw(&item_context, tree, child_image, scratch)?;
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
    let has_separator = item.get_separator();
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
    let has_separator = item.get_separator();
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
