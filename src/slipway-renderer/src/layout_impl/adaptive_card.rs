use std::{cell::RefCell, rc::Rc};

use taffy::{
    prelude::FromLength, FlexDirection, LengthPercentageAuto, NodeId, Rect, Style, TaffyTree,
};

use crate::{
    errors::{RenderError, TaffyErrorToRenderError},
    layoutable::{LayoutContext, Layoutable, TaffyLayoutUtils},
    masked_image::MaskedImage,
    AdaptiveCard, Element,
};

use super::ValidSpacing;

struct IndexedElement<'a> {
    index: usize,
    element: &'a Element,
}

impl Layoutable for AdaptiveCard {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233
    // Reference: https://github.com/microsoft/AdaptiveCards/blob/728044c67510871445d23533fb9830ac57fbbf99/source/nodejs/adaptivecards/src/card-elements.ts#L7820-L7888

    fn layout_override(
        &self,
        context: &LayoutContext,
        tree: &mut TaffyTree<()>,
    ) -> Result<NodeId, RenderError> {
        let body_context = context.for_child_str("body");

        let mut element_node_ids = Vec::new();

        if let Some(body) = &self.body {
            for (index, element) in body.iter().enumerate() {
                let element_context = body_context.for_child(index.to_string());
                let inner_node_id = element.as_layoutable().layout(&element_context, tree)?;

                let spacing = context.host_config.spacing.from(element.as_element());

                let element_node_id = tree
                    .new_with_children(
                        Style {
                            margin: Rect {
                                top: LengthPercentageAuto::from_length(spacing as f32),
                                ..Rect::zero()
                            },
                            ..Default::default()
                        },
                        &[inner_node_id],
                    )
                    .err_context(context)?;

                element_node_ids.push(element_node_id);
            }
        }

        tree.new_with_children(
            Style {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            &element_node_ids,
        )
        .err_context(context)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<()>,
        node_id: NodeId,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        let body_context = context.for_child_str("body");

        let node_layout = tree.layout(node_id).err_context(context)?;
        let actual_rect = node_layout.actual_rect();

        let children_node_ids = tree.children(node_id).err_context(context)?;

        if let Some(body) = &self.body {
            for (i, element) in body
                .iter()
                .enumerate() // Enumerate before filtering, so the index is correct.
                .filter(|(_, e)| e.as_element().get_is_visible())
            {
                let element_layout = tree
                    .layout(children_node_ids[i])
                    .err_context(&body_context)?;

                let element_context = body_context.for_child(i.to_string());
                let element_rect = element_layout.actual_rect();

                let maybe_intersection = actual_rect.intersect(element_rect);

                let Some(intersection) = maybe_intersection else {
                    // If there is no overlap, we can skip drawing the element.
                    continue;
                };

                let child_image = MaskedImage::from_mask(image.clone(), intersection);

                element
                    .as_layoutable()
                    .draw(&element_context, tree, child_image)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    type HostConfigBuilder = crate::host_config::generated::builder::HostConfig;
    use crate::{host_config::generated::HostConfig, render::render};

    #[test]
    fn mixed_container_heights() {
        let json_data = r#"
        {
            "type": "AdaptiveCard",
            "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
            "version": "1.5",
            "minHeight": "800px",
            "body": [
                {
                    "type": "Container",
                    "items": []
                },
                {
                    "type": "Container",
                    "height": "stretch",
                    "style": "attention",
                    "items": []
                },
                {
                    "type": "Container",
                    "height": "stretch",
                    "items": []
                },
                {
                    "type": "Container",
                    "minHeight": "300px",
                    "height": "stretch",
                    "style": "attention",
                    "items": []
                },
                {
                    "type": "Container",
                    "items": []
                }
            ]
        }"#;

        let image = render(
            &HostConfig::try_from(HostConfigBuilder::default()).unwrap(),
            json_data,
            500,
            800,
        )
        .unwrap();

        // Save image to file output.png
        image.save("output.png").unwrap();
    }
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
