use taffy::Size;

use crate::{layout_context::LayoutContext, layout_scratch::LayoutScratch};

use super::text_block::TextBlockNodeContext;

/// The Taffy node context.
pub(crate) enum NodeContext {
    Text(TextBlockNodeContext),
}

/// The Taffy measure function for measuring Adaptive Card elements.
pub(crate) fn measure(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    node_context: Option<&mut NodeContext>,
    context: &LayoutContext,
    scratch: &mut LayoutScratch,
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
            NodeContext::Text(text_context) => {
                text_context.measure(known_dimensions, available_space, context, scratch)
            }
        },
    }
}
