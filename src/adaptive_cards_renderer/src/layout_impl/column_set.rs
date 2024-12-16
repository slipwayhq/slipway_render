use std::{cell::RefCell, rc::Rc};

use adaptive_cards::{Column, HasLayoutData, StringOrNumber};
use taffy::{prelude::length, Dimension, Display, FlexDirection, Size, Style, TaffyTree};

use crate::{
    element_layout_data::{ElementLayoutData, ElementTaffyData, Placement},
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::ValidSpacing,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::AsLayoutable,
    masked_image::MaskedImage,
    utils::ClampToU32,
};

use super::{
    container_shared::{apply_container_style_padding, container_draw_override, PaddingBehavior},
    utils::{
        get_margins_for_bleed, parse_dimension, vertical_content_alignment_to_justify_content,
    },
};

impl AsLayoutable for Column<ElementLayoutData> {
    fn as_layoutable(&self) -> &dyn crate::layoutable::Layoutable {
        self
    }
}

impl crate::layoutable::Layoutable for adaptive_cards::ColumnSet<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        // Parse the min height, if specified.
        let min_height = match &self.min_height {
            Some(min_height) => parse_dimension(min_height, context)?,
            None => Dimension::Auto,
        };

        // Create a mutable baseline style with the min height.
        let mut baseline_style = Style {
            min_size: Size {
                width: Dimension::Percent(1.),
                height: min_height,
            },
            ..baseline_style
        };

        // If the container is set to bleed, we need to add appropriate margins to the baseline style.
        if self.bleed() {
            let placement = self.layout_data.borrow().placement();
            baseline_style.margin = get_margins_for_bleed(&placement, context.host_config);
        }

        // Create the child context.
        let child_elements_context = context
            .for_child_str("columns")
            .with_horizontal_alignment(self.horizontal_alignment)
            .with_style(self.style);

        // Get the child elements.
        let child_elements = self.columns.as_deref().unwrap_or_default();

        // This will contain one node id for each child element, in the same order as the child_elements array.
        let mut child_element_node_ids = Vec::new();

        // This will contain the complete set of child node ids, including decorative items like separators.
        let mut child_node_ids = Vec::new();

        // If any of the child elements have a separator, we need to know the line thickness we should draw.
        let separator_line_thickness = context.host_config.separator.line_thickness.clamp_to_u32();

        // Used to determine if we're drawing the first or last child item.
        let element_count = child_elements.len();

        // Get the sum of all the weighted widths of the child elements.
        let sum_of_weighted_widths = child_elements.iter().fold(0., |acc, column| {
            acc + match column.width {
                Some(StringOrNumber::Number(width)) => width,
                _ => 0.,
            }
        });

        // For each child element...
        for (index, column) in child_elements.iter().enumerate() {
            // Determine the placement of the element within the container relative to any siblings.
            let element_position = match index {
                0 if element_count == 1 => Placement::SoleHorizontal,
                0 => Placement::Left,
                i if i == element_count - 1 => Placement::Right,
                _ => Placement::WithinHorizontal,
            };

            // Save the placement to the element's layout data so we can use it when drawing the element.
            column.layout_data().borrow_mut().placement = Some(element_position);

            // If the element has a separator, we need to add some spacing as defined by
            // the host config, plus additional spacing for the separator line thickness.
            let has_separator = column.get_separator();
            let spacing = context
                .host_config
                .spacing
                .from_spacing(column.get_spacing())
                + match has_separator {
                    true => separator_line_thickness,
                    false => 0,
                };

            // If the element has any spacing, add a node to the Taffy tree to represent it.
            if spacing > 0 {
                match element_position {
                    Placement::Right | Placement::WithinHorizontal => {
                        let spacer_style = Style {
                            size: Size {
                                height: Dimension::Auto,
                                width: length(spacing as f32),
                            },
                            ..Style::default()
                        };
                        let spacer_node_id = tree.new_leaf(spacer_style).err_context(context)?;
                        child_node_ids.push(spacer_node_id);
                    }
                    _ => {}
                }
            }

            // Create a context for the child element.
            let element_context = child_elements_context.for_child(index.to_string());

            // Create a baseline style for the child element, which we will build upon.
            let mut element_baseline_style = Style::default();

            const AUTO_STR: &str = "auto";
            const STRETCH_STR: &str = "stretch";
            // Apply the height of the element to the style.
            match &column.width {
                None => {
                    // Default to the same as "auto".
                    element_baseline_style.flex_basis = Dimension::Auto;
                    element_baseline_style.flex_grow = 0.;
                    element_baseline_style.flex_shrink = 0.;
                }
                Some(StringOrNumber::String(width)) => match width.as_ref() {
                    AUTO_STR => {
                        element_baseline_style.flex_basis = Dimension::Auto;
                        element_baseline_style.flex_grow = 0.;
                        element_baseline_style.flex_shrink = 0.;
                    }
                    STRETCH_STR => {
                        element_baseline_style.flex_basis = Dimension::Auto;
                        element_baseline_style.flex_grow = 1.;
                        element_baseline_style.flex_shrink = 1.;
                    }
                    _ => {
                        element_baseline_style.size.width =
                            parse_dimension(width, &element_context)?;
                    }
                },
                Some(StringOrNumber::Number(width)) => {
                    element_baseline_style.flex_basis =
                        Dimension::Percent((width / sum_of_weighted_widths) as f32);
                    element_baseline_style.flex_grow = 1.;
                    element_baseline_style.flex_shrink = 1.;
                }
            };

            // Call `layout` on the child element, which returns its node id in the Taffy tree.
            let element_node_id = column.layout(&element_context, element_baseline_style, tree)?;

            // Add the node id to the child_element_node_ids array so it can be used in the
            // draw pass to fetch the child element's final position.
            child_element_node_ids.push(element_node_id);

            child_node_ids.push(element_node_id);
        }

        // Next we build up the container style based on the host config and element properties.
        apply_container_style_padding(
            PaddingBehavior::ForStyle(self.style),
            context.host_config,
            &mut baseline_style,
        );

        // Use the vertical content alignment (which was populated by the caller of this function)
        // to determine the flexbox justify content property.
        let justify_content = vertical_content_alignment_to_justify_content(
            child_elements_context.inherited.vertical_content_alignment,
        );

        baseline_style.display = Display::Flex;
        baseline_style.flex_direction = FlexDirection::Row;
        baseline_style.justify_content = Some(justify_content);

        // Finally add ourself to the taffy tree and return the node id other metadata.
        tree.new_with_children(baseline_style, &child_node_ids)
            .err_context(context)
            .map(|node_id| ElementTaffyData {
                node_id,
                child_element_node_ids,
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
        // Delegate to the shared container draw function.
        container_draw_override(
            self, context, tree, taffy_data, image, scratch, self.style, "columns",
        )
    }
}
