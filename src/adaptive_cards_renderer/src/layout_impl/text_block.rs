use std::{cell::RefCell, rc::Rc};

use adaptive_cards::TextBlock;
use taffy::{Style, TaffyTree};

use crate::{
    element_layout_data::{ElementLayoutData, ElementTaffyData},
    errors::RenderError,
    host_config_utils::*,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::Layoutable,
    masked_image::MaskedImage,
};

use super::text_shared::{text_draw_override, text_layout_override, TextContainer};

impl Layoutable for TextBlock<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
        _scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        // Default or heading
        let maybe_text_style = if context.inherited.within_header {
            Some(&context.host_config.text_styles.column_header)
        } else {
            context.host_config.text_styles.from(self.style)
        };

        text_layout_override(self, context, maybe_text_style, baseline_style, tree)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        text_draw_override(context, tree, taffy_data, image, scratch)
    }
}

impl TextContainer for TextBlock<ElementLayoutData> {
    fn color(&self) -> Option<adaptive_cards::Colors> {
        self.color
    }

    fn font_type(&self) -> Option<adaptive_cards::FontType> {
        self.font_type
    }

    fn is_subtle(&self) -> bool {
        self.is_subtle
    }

    fn size(&self) -> Option<adaptive_cards::FontSize> {
        self.size
    }

    fn weight(&self) -> Option<adaptive_cards::FontWeight> {
        self.weight
    }

    fn wrap(&self) -> bool {
        self.wrap
    }

    fn max_lines(&self) -> Option<u32> {
        self.max_lines
            .map(|m| m.round().clamp(u32::MIN as f64, u32::MAX as f64) as u32)
    }

    fn max_width(&self) -> Option<f32> {
        None
    }

    fn horizontal_alignment(&self) -> Option<adaptive_cards::HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn text(&self) -> &str {
        &self.text
    }
}
