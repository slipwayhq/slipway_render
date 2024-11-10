use taffy::Size;

use super::text_block::TextBlockNodeContext;

/// The Taffy node context.
pub(crate) enum NodeContext {
    Text(TextBlockNodeContext),
}

impl NodeContext {
    pub fn text(text: String) -> Self {
        NodeContext::Text(TextBlockNodeContext {
            text,
            offset: Default::default(),
        })
    }
}

/// The Taffy measure function for measuring Adaptive Card elements.
pub(crate) fn measure(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    node_context: Option<&mut NodeContext>,
    parley_font_context: &mut parley::FontContext,
    parley_layout_context: &mut parley::LayoutContext,
    swash_scale_context: &mut swash::scale::ScaleContext,
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
        Some(node_context) => match node_context {
            NodeContext::Text(context) => context.measure(
                known_dimensions,
                available_space,
                parley_font_context,
                parley_layout_context,
                swash_scale_context,
            ),
        },
    }
}
