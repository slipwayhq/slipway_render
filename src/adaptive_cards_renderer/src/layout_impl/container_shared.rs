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
    BlockElementHeight, ContainerStyle, ElementMethods, StackableItemMethods,
    StringOrBlockElementHeight,
};
use imageproc::drawing::{draw_hollow_rect_mut, draw_line_segment_mut};
use taffy::{prelude::length, Dimension, Display, FlexDirection, Rect, Size, Style, TaffyTree};

use super::{
    utils::{
        draw_background, get_margins_for_bleed, parse_dimension,
        vertical_content_alignment_to_justify_content,
    },
    HasChildElements, VerticalContainer,
};

pub(super) fn vertical_container_layout_override<
    TParent: VerticalContainer<TElement>,
    TElement: ElementMethods + Layoutable,
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
    let child_elements_context = context
        .for_child_str("items")
        .with_vertical_content_alignment(parent.get_vertical_content_alignment())
        .with_style(parent.get_style());

    // Get the child elements.
    let child_elements = parent.get_child_elements();

    // Delegate to the shared container layout function.
    container_layout_override_inner(
        context,
        baseline_style,
        tree,
        child_elements_context,
        child_elements,
    )
}

// The shared container layout logic for AdaptiveCard and Container.
pub(super) fn container_layout_override_inner<TElement: ElementMethods + Layoutable>(
    context: &LayoutContext,
    baseline_style: Style,
    tree: &mut TaffyTree<NodeContext>,
    child_elements_context: LayoutContext,
    child_elements: &[TElement],
) -> Result<ElementTaffyData, RenderError> {
    // This will contain one node id for each child element, in the same order as the child_elements array.
    let mut child_element_node_ids = Vec::new();

    // This will contain the complete set of child node ids, including decorative items like separators.
    let mut child_node_ids = Vec::new();

    // If any of the child elements have a separator, we need to know the line thickness we should draw.
    let separator_line_thickness = context.host_config.separator.line_thickness.clamp_to_u32();

    // Used to determine if we're drawing the first or last child item.
    let element_count = child_elements.len();

    // For each child element...
    for (index, element) in child_elements.iter().enumerate() {
        // Determine the placement of the element within the container relative to any siblings.
        let element_position = match index {
            0 if element_count == 1 => Placement::SoleVertical,
            0 => Placement::Top,
            i if i == element_count - 1 => Placement::Bottom,
            _ => Placement::WithinVertical,
        };

        // Save the placement to the element's layout data so we can use it when drawing the element.
        element.layout_data().borrow_mut().placement = Some(element_position);

        // If the element has a separator, we need to add some spacing as defined by
        // the host config, plus additional spacing for the separator line thickness.
        let has_separator = element.get_separator();
        let spacing = context.host_config.spacing.from(element)
            + match has_separator {
                true => separator_line_thickness,
                false => 0,
            };

        // If the element has any spacing, add a node to the Taffy tree to represent it.
        if spacing > 0 {
            match element_position {
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
                _ => {}
            }
        }

        // Create a context for the child element.
        let element_context = child_elements_context.for_child(index.to_string());

        // Create a baseline style for the child element, which we will build upon.
        let mut element_baseline_style = Style::default();

        // Apply the height of the element to the style.
        match element.get_height() {
            StringOrBlockElementHeight::String(height) => {
                element_baseline_style.size.height = parse_dimension(&height, &element_context)?;
            }
            StringOrBlockElementHeight::BlockElementHeight(height) => match height {
                BlockElementHeight::Auto => {
                    element_baseline_style.flex_basis = Dimension::Auto;
                    element_baseline_style.flex_grow = 0.;
                    element_baseline_style.flex_shrink = 0.;
                }
                BlockElementHeight::Stretch => {
                    element_baseline_style.flex_basis = Dimension::Auto;
                    element_baseline_style.flex_grow = 1.;
                    element_baseline_style.flex_shrink = 1.;
                }
            },
        };

        // Call `layout` on the child element, which returns its node id in the Taffy tree.
        let element_node_id = element.layout(&element_context, element_baseline_style, tree)?;

        // Add the node id to the child_element_node_ids array so it can be used in the
        // draw pass to fetch the child element's final position.
        child_element_node_ids.push(element_node_id);

        child_node_ids.push(element_node_id);
    }

    // Next we build up the container style based on the host config and element properties.
    let padding = context.host_config.spacing.padding() as f32;

    // Use the vertical content alignment (which was populated by the caller of this function)
    // to determine the flexbox justify content property.
    let justify_content = vertical_content_alignment_to_justify_content(
        child_elements_context.inherited.vertical_content_alignment,
    );

    let container_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: Some(justify_content),
        padding: Rect {
            top: length(padding),
            left: length(padding),
            right: length(padding),
            bottom: length(padding),
        },
        ..baseline_style
    };

    // Finally add ourself to the taffy tree and return the node id other metadata.
    tree.new_with_children(container_style, &child_node_ids)
        .err_context(context)
        .map(|node_id| ElementTaffyData {
            node_id,
            child_element_node_ids,
        })
}

// The shared container draw logic for AdaptiveCard and Container.
#[allow(clippy::too_many_arguments)] // Fine for now.
pub(super) fn container_draw_override<
    TParent: HasChildElements<TElement>,
    TElement: StackableItemMethods + Layoutable,
>(
    parent: &TParent,
    context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    taffy_data: &ElementTaffyData,
    image: Rc<RefCell<MaskedImage>>,
    scratch: &mut LayoutScratch,
    style: Option<&ContainerStyle>,
    child_elements_context_name: &str,
) -> Result<(), RenderError> {
    // Draw the container background, if necessary.
    if let Some(&style) = style {
        draw_background(style, context, tree, taffy_data, &image)?;
    }

    // Create the child context.
    let child_elements_context = context.for_child_str(child_elements_context_name);

    // Get the child elements.
    let child_elements = parent.get_child_elements();

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

    // Fetch the child element node ids from the layout data, so we can match them
    // to the child elements array.
    let child_element_node_ids = &taffy_data.child_element_node_ids;

    // For each visible child element...
    for (i, element) in child_elements
        .iter()
        .enumerate() // Important: We call enumerate before filtering, so the index is correct.
        .filter(|(_, e)| e.get_is_visible())
    {
        // Get the element's calculated layout data from the Taffy tree.
        let element_layout = tree
            .layout(child_element_node_ids[i])
            .err_context(&child_elements_context)?;

        // Create a context for the child element.
        let element_context =
            child_elements_context.for_child_origin(i.to_string(), element_layout.location);

        // And get the element's absolute rectangle.
        let element_rect = element_layout.absolute_rect(&element_context);

        let element_placement = element
            .layout_data()
            .borrow()
            .placement
            .expect("Element placement not set");

        // Draw the separator, if necessary.
        match element_placement {
            Placement::Top
            | Placement::Bottom
            | Placement::SoleVertical
            | Placement::WithinVertical => {
                draw_horizontal_separator(
                    context,
                    i,
                    element_rect,
                    element,
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
                    element_rect,
                    element,
                    separator_line_thickness,
                    separator_color,
                    &image,
                );
            }
        }

        // Calculate the intersection of the element's rectangle with the container's rectangle.
        let maybe_intersection = absolute_rect.intersect(element_rect);

        // If there is no overlap we can technically skip drawing the element
        // unless we're in the debug mode which specifies transparent masks, in
        // which case we just create a dummy 1 pixel sized intersection so the element
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
            // If there is no overlap, we can skip drawing the element.
            // We already account for the `transparent_masks` debug mode above.
            continue;
        };

        // If we're in the debug mode which specifies we should draw outlines, draw an outline
        // for the child element.
        if context.debug_mode.outlines {
            let color = next_color();
            let mut image_mut = image.borrow_mut();

            draw_hollow_rect_mut(&mut *image_mut, element_rect, color);
        }

        // Create the masked child image.
        let child_image = MaskedImage::from_mask(image.clone(), intersection);

        // Call `draw` on the child element.
        element.draw(&element_context, tree, child_image, scratch)?;
    }
    Ok(())
}

/// Draws a horizontal separator line between child elements, if the child element has
/// its separator property set to true.
fn draw_horizontal_separator<TElement: StackableItemMethods + Layoutable>(
    context: &LayoutContext,
    element_index: usize,
    element_rect: imageproc::rect::Rect,
    element: &TElement,
    separator_line_thickness: u32,
    separator_color: image::Rgba<u8>,
    image: &Rc<RefCell<MaskedImage>>,
) {
    let has_separator = element.get_separator();
    if has_separator && element_index > 0 {
        let spacing = context.host_config.spacing.from(element);
        let half_spacing = (spacing / 2) as f32;

        // The bottom of the horizontal line will be half the separator spacing above the top of the element,
        // minus an additional pixel.
        let y_bottom_float = element_rect.top() as f32 - half_spacing - 1.;

        // The top of the horizontal line is going to be half the spacing above the top of the element,
        // minus the line thickness.
        let y_top_float = y_bottom_float - separator_line_thickness as f32 + 1.;

        let x_start = element_rect.left() as f32;
        let x_end = element_rect.right() as f32;

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

/// Draws a vertical separator line between child elements, if the child element has
/// its separator property set to true.
fn draw_vertical_separator<TElement: StackableItemMethods + Layoutable>(
    context: &LayoutContext,
    element_index: usize,
    element_rect: imageproc::rect::Rect,
    element: &TElement,
    separator_line_thickness: u32,
    separator_color: image::Rgba<u8>,
    image: &Rc<RefCell<MaskedImage>>,
) {
    let has_separator = element.get_separator();
    if has_separator && element_index > 0 {
        let spacing = context.host_config.spacing.from(element);
        let half_spacing = (spacing / 2) as f32;

        // The right of the horizontal line will be half the separator spacing to the left of the element,
        // minus an additional pixel.
        let x_right_float = element_rect.left() as f32 - half_spacing - 1.;

        // The left of the horizontal line is going to be half the spacing to the left of the element,
        // minus the line thickness.
        let x_left_float = x_right_float - separator_line_thickness as f32 + 1.;

        let y_start = element_rect.top() as f32;
        let y_end = element_rect.bottom() as f32;

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
