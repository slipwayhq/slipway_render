use std::{cell::RefCell, rc::Rc};

use adaptive_cards::HasLayoutData;
use adaptive_cards_host_config::{FactSetTextConfig, TextStyleConfig};
use taffy::TaffyTree;

use crate::{
    element_layout_data::ElementTaffyData,
    errors::{RenderError, TaffyErrorToRenderError},
    layout_context::LayoutContext,
    layout_scratch::LayoutScratch,
    masked_image::MaskedImage,
    ElementLayoutData,
};

use super::{
    measure::NodeContext,
    text_shared::{text_draw_override, text_layout_override, TextContainer},
};

impl crate::layoutable::Layoutable for adaptive_cards::Fact<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
        _scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        let title_result = layout_child_item(
            &self.title,
            &context.host_config.fact_set.title,
            context,
            baseline_style.clone(),
            tree,
        )?;

        let value_result = layout_child_item(
            &self.value,
            &context.host_config.fact_set.value,
            context,
            baseline_style.clone(),
            tree,
        )?;

        let child_taffy_data_results = vec![title_result, value_result];
        let child_item_node_ids = child_taffy_data_results
            .iter()
            .map(|result| result.node_id)
            .collect::<Vec<_>>();

        let child_elements = child_taffy_data_results
            .into_iter()
            .map(|result| ElementLayoutData {
                taffy_data: Some(result),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let layout_data = self.layout_data();
        let mut data_mut = layout_data.borrow_mut();
        data_mut.virtual_elements = Some(child_elements);

        tree.new_with_children(baseline_style, &child_item_node_ids)
            .err_context(context)
            .map(|node_id| ElementTaffyData {
                node_id,
                child_item_node_ids,
            })
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        // Because we set virtual elements, draw_override will be called once per virtual element.
        text_draw_override(context, tree, taffy_data, Rc::clone(&image), scratch)
    }
}

fn layout_child_item(
    text: &str,
    fallback_style: &FactSetTextConfig,
    context: &LayoutContext,
    baseline_style: taffy::Style,
    tree: &mut TaffyTree<NodeContext>,
) -> Result<ElementTaffyData, RenderError> {
    let fallback_text_style = get_text_style_config(fallback_style);
    let text_container = FactText { text };
    text_layout_override(
        &text_container,
        context,
        Some(&fallback_text_style),
        baseline_style,
        tree,
    )
}

fn get_text_style_config(input: &FactSetTextConfig) -> TextStyleConfig {
    TextStyleConfig {
        color: input.color,
        font_type: input.font_type,
        is_subtle: input.is_subtle,
        size: input.size,
        weight: input.weight,
    }
}

struct FactText<'a> {
    text: &'a str,
}

impl TextContainer for FactText<'_> {
    fn color(&self) -> Option<adaptive_cards::Colors> {
        None
    }

    fn font_type(&self) -> Option<adaptive_cards::FontType> {
        None
    }

    fn is_subtle(&self) -> bool {
        false
    }

    fn size(&self) -> Option<adaptive_cards::FontSize> {
        None
    }

    fn weight(&self) -> Option<adaptive_cards::FontWeight> {
        None
    }

    fn wrap(&self) -> bool {
        true
    }

    fn max_lines(&self) -> Option<u32> {
        None
    }

    fn horizontal_alignment(&self) -> Option<adaptive_cards::HorizontalAlignment> {
        None
    }

    fn text(&self) -> &str {
        self.text
    }
}
