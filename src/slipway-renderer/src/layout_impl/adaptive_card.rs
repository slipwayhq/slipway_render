use std::{cell::RefCell, rc::Rc};

use imageproc::rect::Rect;

use crate::{
    errors::RenderError,
    layoutable::{HasLayoutData, LayoutContext, Layoutable},
    masked_image::MaskedImage,
    size::Size,
    AdaptiveCard, BlockElementHeight, Element, StringOrBlockElementHeight,
};

use super::ValidSpacing;

struct IndexedElement<'a> {
    index: usize,
    element: &'a Element,
}

impl Layoutable for AdaptiveCard {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233

    fn measure_override(
        &self,
        context: &LayoutContext,
        available_size: Size,
    ) -> Result<Size, RenderError> {
        let body_context = context.for_child_str("body");

        let mut size = Size::new(0, 0);

        if let Some(body) = &self.body {
            let mut remaining_height = available_size.height();
            let indexed_elements = body
                .iter()
                .enumerate()
                .map(|(index, element)| IndexedElement { index, element })
                .collect::<Vec<_>>();

            let (stretched_children, non_stretched_children): (Vec<_>, Vec<_>) = indexed_elements
                .iter()
                .filter(|e| e.element.as_element().get_is_visible())
                .partition(|e| {
                    matches!(
                        e.element.as_element().get_height(),
                        StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Stretch)
                    )
                });

            for IndexedElement { index, element } in non_stretched_children.iter() {
                let spacing = context.host_config.spacing.from(element.as_element());
                let element_context = body_context.for_child(index.to_string());
                let desired_size = element.as_layoutable().measure(
                    &element_context,
                    Size::new(
                        available_size.width(),
                        remaining_height.saturating_sub(spacing),
                    ),
                )?;

                let height_with_spacing = desired_size.height() + spacing;

                size = Size::new(
                    size.width().max(desired_size.width()),
                    size.height() + height_with_spacing,
                );
                remaining_height = remaining_height.saturating_sub(height_with_spacing);
            }

            let mut remaining_stretched_children = stretched_children;

            // We will keep measuring the stretched elements until they fit within the available height.
            while !remaining_stretched_children.is_empty() {
                let stretched_children_count = remaining_stretched_children.len();
                let stretched_children_height = remaining_height / stretched_children_count as u32;

                let mut new_remaining_stretched_children = Vec::new();

                for item in remaining_stretched_children.into_iter() {
                    let IndexedElement { index, element } = item;
                    let spacing = context.host_config.spacing.from(element.as_element());
                    let element_context = body_context.for_child(index.to_string());
                    let desired_size = element.as_layoutable().measure(
                        &element_context,
                        Size::new(
                            available_size.width(),
                            stretched_children_height.saturating_sub(spacing),
                        ),
                    )?;

                    let height_with_spacing = desired_size.height() + spacing;

                    // If the desired height is greater than the available stretched height,
                    // then we need to re-measure the other stretched elements with the remaining height.
                    if height_with_spacing > stretched_children_height {
                        size = Size::new(
                            size.width().max(desired_size.width()),
                            size.height() + height_with_spacing,
                        );
                        remaining_height = remaining_height.saturating_sub(height_with_spacing);
                    } else {
                        new_remaining_stretched_children.push(item);
                    }
                }

                // All elements were within available stretched height, so we can break.
                if stretched_children_count == new_remaining_stretched_children.len() {
                    break;
                }

                remaining_stretched_children = new_remaining_stretched_children;
            }
        }

        Ok(size)
    }

    fn arrange_override(
        &self,
        context: &LayoutContext,
        final_rect: Rect,
    ) -> Result<Rect, RenderError> {
        let body_context = context.for_child_str("body");

        let mut child_rect = final_rect;
        let mut previous_child_height: u32 = 0;

        if let Some(body) = &self.body {
            for (i, element) in body
                .iter()
                .enumerate() // Enumerate before filtering, so the index is correct.
                .filter(|(_, e)| e.as_element().get_is_visible())
            {
                let element_context = body_context.for_child(i.to_string());

                let spacing = context.host_config.spacing.from(element.as_element());

                let desired_size = element.as_layoutable().desired_size();

                child_rect = Rect::at(
                    child_rect.left(),
                    child_rect.top()
                        + i32::try_from(previous_child_height).expect("height was too large"),
                )
                .of_size(
                    final_rect.width().min(desired_size.width()),
                    desired_size.height(),
                );

                previous_child_height = desired_size.height() + spacing;

                element
                    .as_layoutable()
                    .arrange(&element_context, child_rect)?;
            }
        }
        Ok(final_rect)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        let body_context = context.for_child_str("body");

        let actual_rect = self.actual_rect();

        if let Some(body) = &self.body {
            for (i, element) in body
                .iter()
                .enumerate() // Enumerate before filtering, so the index is correct.
                .filter(|(_, e)| e.as_element().get_is_visible())
            {
                let element_context = body_context.for_child(i.to_string());
                let element_rect = element.as_layoutable().actual_rect();

                let maybe_intersection = actual_rect.intersect(element_rect);

                let Some(intersection) = maybe_intersection else {
                    // If there is no overlap, we can skip drawing the element.
                    continue;
                };

                let child_image = MaskedImage::from_mask(image.clone(), intersection);

                element
                    .as_layoutable()
                    .draw(&element_context, child_image)?;
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
