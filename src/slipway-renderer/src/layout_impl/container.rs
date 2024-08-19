use std::{cell::RefCell, rc::Rc};

use imageproc::drawing::draw_hollow_rect_mut;
use taffy::{
    prelude::FromLength, Dimension, Display, FlexDirection, JustifyContent, LengthPercentageAuto,
    Rect, Size, Style, TaffyTree,
};

use crate::{
    errors::{RenderError, TaffyErrorToRenderError},
    layoutable::{ElementTaffyData, LayoutContext, Layoutable, TaffyLayoutUtils},
    masked_image::MaskedImage,
    BlockElementHeight, Container, Element, StringOrBlockElementHeight,
};

use super::{next_color, parse_dimension, NodeContext, ValidSpacing};

impl Layoutable for Container {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233
    // Reference: https://github.com/microsoft/AdaptiveCards/blob/728044c67510871445d23533fb9830ac57fbbf99/source/nodejs/adaptivecards/src/card-elements.ts#L7820-L7888

    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        let min_height = match &self.min_height {
            Some(min_height) => parse_dimension(min_height, context)?,
            None => Dimension::Auto,
        };

        let baseline_style = Style {
            min_size: Size {
                width: Dimension::Percent(1.),
                height: min_height,
            },
            ..baseline_style
        };

        let child_elements_context = context.for_child_str("items");
        let child_elements = &self.items;

        container_layout_override(
            context,
            baseline_style,
            tree,
            child_elements_context,
            child_elements,
        )
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        let child_elements_context = context.for_child_str("items");
        let child_elements = &self.items;

        container_draw_override(
            context,
            tree,
            taffy_data,
            image,
            child_elements_context,
            child_elements,
        )
    }
}

pub(super) fn container_layout_override(
    context: &LayoutContext,
    baseline_style: Style,
    tree: &mut TaffyTree<NodeContext>,
    child_elements_context: LayoutContext,
    child_elements: &[Element],
) -> Result<ElementTaffyData, RenderError> {
    let mut child_element_node_ids = Vec::new();
    let mut child_node_ids = Vec::new();

    for (index, element) in child_elements.iter().enumerate() {
        let element_context = child_elements_context.for_child(index.to_string());

        let as_element = element.as_element();
        let spacing = context.host_config.spacing.from(as_element);

        let mut element_baseline_style = Style {
            margin: Rect {
                top: LengthPercentageAuto::from_length(spacing as f32),
                ..Rect::auto()
            },
            ..Style::default()
        };

        match as_element.get_height() {
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

        let element_node_id =
            element
                .as_layoutable()
                .layout(&element_context, element_baseline_style, tree)?;

        child_element_node_ids.push(element_node_id);
        child_node_ids.push(element_node_id);
    }

    tree.new_with_children(
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: Some(JustifyContent::FlexStart),
            ..baseline_style
        },
        &child_node_ids,
    )
    .err_context(context)
    .map(|node_id| ElementTaffyData {
        node_id,
        child_element_node_ids,
    })
}

pub(super) fn container_draw_override(
    context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    taffy_data: &ElementTaffyData,
    image: Rc<RefCell<MaskedImage>>,
    child_elements_context: LayoutContext,
    child_elements: &[Element],
) -> Result<(), RenderError> {
    let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
    let absolute_rect = node_layout.absolute_rect(context);

    if context.debug_mode.outlines {
        let color = next_color();
        let mut image_mut = image.borrow_mut();

        draw_hollow_rect_mut(&mut *image_mut, absolute_rect, color);
    }

    // let child_node_ids = tree.children(taffy_data.node_id).err_context(context)?;
    let child_element_node_ids = &taffy_data.child_element_node_ids;

    for (i, element) in child_elements
        .iter()
        .enumerate() // Enumerate before filtering, so the index is correct.
        .filter(|(_, e)| e.as_element().get_is_visible())
    {
        let element_layout = tree
            .layout(child_element_node_ids[i])
            .err_context(&child_elements_context)?;

        let element_context =
            child_elements_context.for_child_origin(i.to_string(), element_layout.location);
        let element_rect = element_layout.absolute_rect(&element_context);

        let maybe_intersection = absolute_rect.intersect(element_rect);

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
            continue;
        };

        if context.debug_mode.outlines {
            let color = next_color();
            let mut image_mut = image.borrow_mut();

            draw_hollow_rect_mut(&mut *image_mut, element_rect, color);
        }

        let child_image = MaskedImage::from_mask(image.clone(), intersection);

        element
            .as_layoutable()
            .draw(&element_context, tree, child_image)?;
    }
    Ok(())
}

// Test:
// {
//     "type": "AdaptiveCard",
//     "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
//     "version": "1.5",
//     "minHeight": "800px",
//     "body": [
//         {
//             "type": "Container"
//         },
//         {
//             "type": "Container",
//             "height": "stretch"
//         },
//         {
//             "type": "Container",
//             "height": "stretch"
//         },
//         {
//             "type": "Container",
//             "minHeight": "300px",
//             "height": "stretch"
//         },
//         {
//             "type": "Container"
//         }
//     ]
// }
