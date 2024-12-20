use std::{cell::RefCell, rc::Rc};

use adaptive_cards_host_config::{FactSetTextConfig, TextStyleConfig};
use taffy::{
    LengthPercentage, MaxTrackSizingFunction, MinTrackSizingFunction,
    NonRepeatedTrackSizingFunction, Style, TaffyTree, TrackSizingFunction,
};

use crate::{
    element_layout_data::ElementTaffyData,
    errors::{RenderError, TaffyErrorToRenderError},
    layout_context::LayoutContext,
    layout_scratch::LayoutScratch,
    masked_image::MaskedImage,
    utils::TaffyLayoutUtils,
    ElementLayoutData,
};

use super::{
    measure::NodeContext,
    text_shared::{text_draw_override, text_layout_override, TextContainer},
};

impl crate::layoutable::Layoutable for adaptive_cards::FactSet<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
        _scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        let mut style = baseline_style;

        // Create the child context.
        let child_items_context = context.for_child_str("facts");

        let title_config = &context.host_config.fact_set.title;
        let value_config = &context.host_config.fact_set.value;

        // This will contain the complete set of child node ids, including decorative items like separators.
        let mut child_item_node_ids = Vec::new();

        for (index, fact) in self.facts.iter().enumerate() {
            let item_context = child_items_context.for_child(index.to_string());

            {
                let layout_result = layout_fact_text(
                    &fact.title,
                    title_config,
                    &item_context.for_child_str("title"),
                    Style::default(),
                    tree,
                )?;

                child_item_node_ids.push(layout_result.node_id);
            }
            {
                let layout_result = layout_fact_text(
                    &fact.value,
                    value_config,
                    &item_context.for_child_str("value"),
                    Style::default(),
                    tree,
                )?;

                child_item_node_ids.push(layout_result.node_id);
            }
        }

        style.display = taffy::Display::Grid;

        style.gap = taffy::Size {
            width: LengthPercentage::Length(context.host_config.fact_set.spacing as f32),
            height: LengthPercentage::Length(context.host_config.fact_set.spacing as f32),
        };

        style.grid_template_columns = vec![
            TrackSizingFunction::Single(NonRepeatedTrackSizingFunction {
                min: MinTrackSizingFunction::Fixed(LengthPercentage::Length(0.)),
                max: MaxTrackSizingFunction::MaxContent,
            }),
            TrackSizingFunction::Single(NonRepeatedTrackSizingFunction {
                min: MinTrackSizingFunction::Fixed(LengthPercentage::Length(0.)),
                max: if value_config.max_width == 0 {
                    MaxTrackSizingFunction::Fraction(1.)
                } else {
                    MaxTrackSizingFunction::MaxContent
                },
            }),
        ];

        // Finally add ourself to the taffy tree and return the node id other metadata.
        tree.new_with_children(style, &child_item_node_ids)
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
        let child_item_node_ids = &taffy_data.child_item_node_ids;

        let child_items_context = context.for_child_str("facts");
        for (i, _fact) in self.facts.iter().enumerate() {
            let item_context = child_items_context.for_child(i.to_string());
            draw_fact_text(
                child_item_node_ids[2 * i],
                "title",
                &item_context,
                tree,
                &image,
                scratch,
            )?;
            draw_fact_text(
                child_item_node_ids[2 * i + 1],
                "value",
                &item_context,
                tree,
                &image,
                scratch,
            )?;
        }

        Ok(())
    }
}

fn layout_fact_text(
    text: &str,
    fallback_style: &FactSetTextConfig,
    context: &LayoutContext,
    baseline_style: taffy::Style,
    tree: &mut TaffyTree<NodeContext>,
) -> Result<ElementTaffyData, RenderError> {
    let fallback_text_style = get_text_style_config(fallback_style);
    let text_container = FactText {
        text,
        max_width: if fallback_style.max_width == 0 {
            None
        } else {
            Some(fallback_style.max_width as f32)
        },
    };
    text_layout_override(
        &text_container,
        context,
        Some(&fallback_text_style),
        baseline_style,
        tree,
    )
}

fn draw_fact_text(
    node_id: taffy::NodeId,
    text_type: &str,
    item_context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    image: &Rc<RefCell<MaskedImage>>,
    scratch: &mut LayoutScratch,
) -> Result<(), RenderError> {
    let item_layout = tree.layout(node_id).err_context(item_context)?;
    let item_context = item_context.for_child_str_origin(text_type, item_layout.location);
    let absolute_rect = item_layout.absolute_rect(&item_context);
    let masked_image = MaskedImage::from_mask(image.clone(), absolute_rect);
    let taffy_data = ElementTaffyData {
        node_id,
        child_item_node_ids: vec![],
    };
    text_draw_override(&item_context, tree, &taffy_data, masked_image, scratch)?;
    Ok(())
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
    max_width: Option<f32>,
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

    fn max_width(&self) -> Option<f32> {
        self.max_width
    }

    fn horizontal_alignment(&self) -> Option<adaptive_cards::HorizontalAlignment> {
        None
    }

    fn text(&self) -> &str {
        self.text
    }
}
