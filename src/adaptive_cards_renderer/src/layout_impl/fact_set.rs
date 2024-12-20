use std::{cell::RefCell, rc::Rc};

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
    ElementLayoutData,
};

use super::measure::NodeContext;

impl crate::layoutable::Layoutable for adaptive_cards::FactSet<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
        scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        let mut style = baseline_style;

        // Create the child context.
        let child_items_context = context.for_child_str("facts");

        // This will contain the complete set of child node ids, including decorative items like separators.
        let mut child_item_node_ids = Vec::new();

        for (index, fact) in self.facts.iter().enumerate() {
            // Create a context for the child item.
            let item_context = child_items_context.for_child(index.to_string());

            // Create a baseline style for the child item, which we will build upon.
            let item_baseline_style = Style::default();

            // Call `layout` on the child item, which returns its node id in the Taffy tree.
            // The Fact will add the `title` and `value` nodes as children, however because
            // the FactSet is a grid we need to fetch those children and add them as direct
            // children of the FacSet.
            let item_node_id = fact.layout(&item_context, item_baseline_style, tree, scratch)?;
            let fact_child_node_ids = tree.children(item_node_id).err_context(&item_context)?;
            tree.remove(item_node_id).err_context(context)?;
            child_item_node_ids.extend(fact_child_node_ids);
        }

        let title_config = &context.host_config.fact_set.title;
        let value_config = &context.host_config.fact_set.value;

        style.display = taffy::Display::Grid;

        style.gap = taffy::Size {
            width: LengthPercentage::Length(context.host_config.fact_set.spacing as f32),
            height: LengthPercentage::Length(context.host_config.fact_set.spacing as f32),
        };

        style.grid_template_columns = vec![
            TrackSizingFunction::Single(NonRepeatedTrackSizingFunction {
                min: MinTrackSizingFunction::Auto,
                max: MaxTrackSizingFunction::Fixed(LengthPercentage::Length(
                    title_config.max_width as f32,
                )),
            }),
            TrackSizingFunction::Single(NonRepeatedTrackSizingFunction {
                min: MinTrackSizingFunction::Auto,
                max: MaxTrackSizingFunction::Fixed(LengthPercentage::Length(
                    value_config.max_width as f32,
                )),
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
        _taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        let child_items_context = context.for_child_str("facts");
        for (index, fact) in self.facts.iter().enumerate() {
            let item_context = child_items_context.for_child(index.to_string());
            fact.draw(item_context, tree, Rc::clone(&image), scratch)?;
        }

        Ok(())
    }
}
