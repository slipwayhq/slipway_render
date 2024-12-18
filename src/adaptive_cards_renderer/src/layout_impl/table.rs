use std::{cell::RefCell, rc::Rc};

use adaptive_cards::{StringOrNumber, Table, TableColumnDefinition};
use taffy::{Dimension, TaffyTree};

use crate::{
    element_layout_data::{ElementTaffyData, TableLayoutData},
    errors::RenderError,
    layout_context::LayoutContext,
    layout_scratch::LayoutScratch,
    masked_image::MaskedImage,
    ElementLayoutData,
};

use super::{
    container_shared::{container_draw_override, container_layout_override},
    measure::NodeContext,
    Layoutable,
};

impl Layoutable for Table<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        mut baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        let rows = self.rows.as_deref().unwrap_or(&[]);
        let columns = self.columns.as_deref().unwrap_or(&[]);

        let default_column = TableColumnDefinition {
            horizontal_cell_content_alignment: None,
            vertical_cell_content_alignment: None,
            width: StringOrNumber::Number(1.0),
            type_: None,
        };

        // Set the column definition on all the cells.
        for row in rows.iter() {
            row.layout_data.borrow_mut().table_data = Some(TableLayoutData {
                no_border: true,
                table_column_definition: None,
            });
            if let Some(cells) = &row.cells {
                for (cell_index, cell) in cells.iter().enumerate() {
                    let column = columns.get(cell_index).unwrap_or(&default_column);
                    cell.layout_data.borrow_mut().table_data = Some(TableLayoutData {
                        no_border: true,
                        table_column_definition: Some(column.clone()),
                    });
                }
            }
        }

        baseline_style.min_size.width = Dimension::Percent(1.);
        container_layout_override(self, context, baseline_style, tree)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        container_draw_override(self, context, tree, taffy_data, image, scratch)
    }
}
// let child_items_context = context
//     .for_child_str("rows")
//     .with_vertical_content_alignment(self.vertical_cell_content_alignment)
//     .with_horizontal_content_alignment(self.horizontal_cell_content_alignment);

// let rows = self.rows.as_deref().unwrap_or(&[]);

// let row_count = rows.len();

// let max_columns = rows
//     .iter()
//     .map(|r| r.cells.map(|c| c.len()).unwrap_or(0))
//     .max()
//     .unwrap_or(0);

// let default_column = TableColumnDefinition {
//     horizontal_cell_content_alignment: None,
//     vertical_cell_content_alignment: None,
//     width: StringOrNumber::Number(1.0),
//     type_: None,
// };

// // Fill in missing columns with default columns
// let mut columns: Vec<&TableColumnDefinition> = {
//     let mut columns: Vec<_> = self.columns.as_deref().unwrap_or(&[]).iter().collect();
//     columns.extend(std::iter::repeat(&default_column).take(max_columns - columns.len()));
//     columns
// };

// // This will contain one node id for each child, in the same order as the children array.
// let mut child_item_node_ids = Vec::new();

// // This will contain the complete set of child node ids, including decorative items like separators.
// let mut child_node_ids = Vec::new();

// // If we're showing grid lines, we draw a line between each cell plus the default spacing.
// let half_spacing = context.host_config.spacing.default;
// let separator_line_thickness = if self.show_grid_lines {
//     context.host_config.separator.line_thickness
// } else {
//     0
// };

// // For each child item...
// for (row_index, row) in rows.iter().enumerate() {
//     let item_position = match row_index {
//         0 if row_count == 1 => Placement::SoleVertical,
//         0 => Placement::Top,
//         i if i == row_count - 1 => Placement::Bottom,
//         _ => Placement::WithinVertical,
//     };

//     let (top_spacing, bottom_spacing) = match item_position {
//         Placement::Top => (half_spacing + separator_line_thickness, 0),
//         Placement::WithinVertical => (2 * half_spacing + separator_line_thickness, 0),
//         Placement::Bottom => (
//             2 * half_spacing + separator_line_thickness,
//             half_spacing + separator_line_thickness,
//         ),
//         Placement::SoleVertical => (
//             half_spacing + separator_line_thickness,
//             half_spacing + separator_line_thickness,
//         ),
//         _ => unreachable!(),
//     };

//     let add_spacing = |spacing: i64| -> Result<(), RenderError> {
//         if spacing <= 0 {
//             return Ok(());
//         }

//         let spacer_style = Style {
//             size: Size {
//                 height: length(spacing as f32),
//                 width: Dimension::Auto,
//             },
//             ..Style::default()
//         };
//         let spacer_node_id = tree.new_leaf(spacer_style).err_context(context)?;
//         child_node_ids.push(spacer_node_id);
//         Ok(())
//     };

//     add_spacing(top_spacing)?;

//     // Create a context for the child item.
//     let item_context = child_items_context.for_child(row_index.to_string());

//     // Create a baseline style for the child item, which we will build upon.
//     let mut item_baseline_style = Style::default();
//     item_baseline_style.flex_basis = Dimension::Auto;
//     item_baseline_style.flex_grow = 0.;
//     item_baseline_style.flex_shrink = 0.;

//     for (cell_index, cell) in row.cells.iter().enumerate() {

//         // Call `layout` on the child item, which returns its node id in the Taffy tree.
//         let item_node_id = row.layout(&item_context, item_baseline_style, tree)?;
//     }

//     // Add the node id to the child_item_node_ids array so it can be used in the
//     // draw pass to fetch the child item's final position.
//     child_item_node_ids.push(item_node_id);

//     child_node_ids.push(item_node_id);

//     add_spacing(bottom_spacing)?;
// }
