use std::{cell::RefCell, rc::Rc};

use image::Rgba;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use taffy::{Dimension, NodeId, Size, Style, TaffyTree};

use crate::{
    errors::RenderError,
    layoutable::{LayoutContext, Layoutable},
    masked_image::{MaskedImage, SlipwayCanvas},
    AdaptiveCard,
};

use super::{
    container::{container_draw_override, container_layout_override},
    NodeContext,
};

impl Layoutable for AdaptiveCard {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233
    // Reference: https://github.com/microsoft/AdaptiveCards/blob/728044c67510871445d23533fb9830ac57fbbf99/source/nodejs/adaptivecards/src/card-elements.ts#L7820-L7888

    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<NodeId, RenderError> {
        let baseline_style = Style {
            size: Size {
                width: Dimension::Percent(1.),
                height: Dimension::Percent(1.),
            },
            ..baseline_style
        };

        let child_elements_context = context.for_child_str("body");
        let child_elements = self.body.as_deref().unwrap_or(&[]);

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
        node_id: NodeId,
        image: Rc<RefCell<MaskedImage>>,
    ) -> Result<(), RenderError> {
        // Fill the background with white.
        {
            let (width, height) = image.dimensions();
            let mut image_mut = image.borrow_mut();
            draw_filled_rect_mut(
                &mut *image_mut,
                Rect::at(0, 0).of_size(width, height),
                Rgba([255, 255, 255, 255]),
            );
        }

        let child_elements_context = context.for_child_str("body");
        let child_elements = self.body.as_deref().unwrap_or(&[]);

        container_draw_override(
            context,
            tree,
            node_id,
            image,
            child_elements_context,
            child_elements,
        )
    }
}

#[cfg(test)]
mod tests {
    type HostConfigBuilder = crate::host_config::generated::builder::HostConfig;
    type SpacingsConfigBuilder = crate::host_config::generated::builder::SpacingsConfig;
    use crate::{host_config::generated::HostConfig, layoutable::DebugMode, render::render};

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
                    "style": "accent",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "1"
                        }
                    ]
                },
                {
                    "type": "Container",
                    "style": "accent",
                    "height": "stretch",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "2"
                        }
                    ]
                },
                {
                    "type": "Container",
                    "style": "accent",
                    "height": "stretch",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "3"
                        }
                    ]
                },
                {
                    "type": "Container",
                    "style": "accent",
                    "minHeight": "300px",
                    "height": "stretch",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "4"
                        }
                    ]
                },
                {
                    "type": "Container",
                    "style": "accent",
                    "items": [
                        {
                            "type": "TextBlock",
                            "text": "5"
                        }
                    ]
                }
            ]
        }"#;

        let image = render(
            &HostConfig::try_from(HostConfigBuilder::default()).unwrap(),
            json_data,
            500,
            800,
            DebugMode::TransparentMasks,
        )
        .unwrap();

        // Save image to file output.png
        image.save("output.png").unwrap();

        todo!("Test the output image");
    }
}
