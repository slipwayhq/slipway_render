use std::{cell::RefCell, rc::Rc};

use adaptive_cards::{HasLayoutData, StringOrNumber, TableColumnDefinition};
use image::Rgba;
use taffy::{AvailableSpace, TaffyTree};

use crate::{
    element_layout_data::{ElementTaffyData, FactSetLayoutData},
    errors::RenderError,
    layout_context::LayoutContext,
    layout_scratch::LayoutScratch,
    masked_image::MaskedImage,
    ElementLayoutData,
};

use super::{measure::NodeContext, text_block::TextBlockNodeContext, ClampToU32};

impl crate::layoutable::Layoutable for adaptive_cards::FactSet<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
        scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        if self.layout_data().borrow().fact_set_data.is_none() {
            let titles: Vec<_> = self.facts.iter().map(|fact| fact.title.as_str()).collect();
            // let values: Vec<_> = self.facts.iter().map(|fact| fact.value.as_str()).collect();

            let mut title_contexts = titles_to_text_block_contexts(titles.as_slice(), context)?;
            let max_title_width: Vec<_> = title_contexts
                .iter_mut()
                .map(|c| {
                    c.measure_quick(
                        taffy::Size::<Option<f32>>::NONE,
                        taffy::Size::<AvailableSpace>::max_content(),
                        context,
                        scratch,
                    )
                    .width
                })
                .collect();

            let max_width = context.host_config.fact_set.title.max_width;

            let max_title_width = max_title_width
                .iter()
                .cloned()
                .max_by(|a, b| a.partial_cmp(b).expect("All widths should be real numbers"))
                .unwrap_or(0.)
                .ceil() as i64;

            let title_column_width =
                max_title_width.min(max_width) + (2 * context.host_config.spacing.small) + 50;

            let table = adaptive_cards::Table::<ElementLayoutData> {
                first_row_as_header: false,
                show_grid_lines: true,
                columns: Some(vec![TableColumnDefinition {
                    horizontal_cell_content_alignment: None,
                    vertical_cell_content_alignment: None,
                    width: StringOrNumber::String(format!("{}px", title_column_width)),
                    type_: None,
                }]),
                rows: Some(
                    self.facts
                        .iter()
                        .map(|fact| adaptive_cards::TableRow::<ElementLayoutData> {
                            cells: Some(vec![
                                adaptive_cards::TableCell {
                                    items: vec![adaptive_cards::Element::TextBlock(Box::new(
                                        adaptive_cards::TextBlock::<ElementLayoutData> {
                                            text: fact.title.clone(),
                                            weight: Some(adaptive_cards::FontWeight::Bolder),
                                            ..Default::default()
                                        },
                                    ))],
                                    ..Default::default()
                                },
                                adaptive_cards::TableCell {
                                    items: vec![adaptive_cards::Element::TextBlock(Box::new(
                                        adaptive_cards::TextBlock::<ElementLayoutData> {
                                            text: fact.value.clone(),
                                            ..Default::default()
                                        },
                                    ))],
                                    ..Default::default()
                                },
                            ]),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            };

            self.layout_data().borrow_mut().fact_set_data = Some(FactSetLayoutData {
                table: Box::new(table),
            });
        }

        let layout_data = self.layout_data().borrow();
        let table = layout_data
            .fact_set_data
            .as_ref()
            .expect("Fact set data should have been created")
            .table
            .as_ref();

        table.layout_override(context, baseline_style, tree, scratch)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        let layout_data = self.layout_data().borrow();
        let table = layout_data
            .fact_set_data
            .as_ref()
            .expect("Fact set data should have been created")
            .table
            .as_ref();

        table.draw_override(context, tree, taffy_data, image, scratch)
    }
}

fn titles_to_text_block_contexts(
    titles: &[&str],
    context: &LayoutContext,
) -> Result<Vec<TextBlockNodeContext>, RenderError> {
    let font_type = &context.host_config.font_types.default;
    let font_family = context.get_resolved_font_family(&font_type.font_family)?;
    let font_size = font_type.font_sizes.default.clamp_to_u32() as f32;
    let font_weight = font_type.font_weights.default.clamp_to_u32() as f32;
    let color = Rgba::<u8>([0, 0, 0, 255]);
    Ok(titles
        .iter()
        .map(|title| TextBlockNodeContext {
            text: title.to_string(),
            color,
            font_family: font_family.clone(),
            font_size,
            font_weight,
            max_lines: None,
            wrap: false,
            offset: RefCell::new(Default::default()),
            horizontal_alignment: adaptive_cards::HorizontalAlignment::Left,
        })
        .collect())
}
