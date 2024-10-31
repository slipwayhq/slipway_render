use taffy::Size;

use crate::fonts::FontCache;

/// The Taffy node context.
pub(super) enum NodeContext {
    Text,
}

/// The Taffy measure function for measuring Adaptive Card elements.
pub(super) fn measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    _available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    node_context: Option<&mut NodeContext>,
    _font_cache: &FontCache,
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
        Some(NodeContext::Text) => Size {
            width: 100.,
            height: 10.,
        },
    }
}
