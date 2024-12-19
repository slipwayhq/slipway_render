use std::{cell::RefCell, rc::Rc};

use adaptive_cards::{StringOrNumber, Table, TableColumnDefinition};
use taffy::{Dimension, TaffyTree};

use crate::{
    element_layout_data::{ElementTaffyData, TableLayoutData, TablePart},
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

        let grid_style = self.grid_style.unwrap_or(context.inherited.style);

        self.layout_data.borrow_mut().table_data = Some(TableLayoutData {
            part: TablePart::Table,
            show_grid_lines: self.show_grid_lines,
            grid_style,
            table_column_definition: None,
        });

        // Set the column definition on all the cells.
        for row in rows.iter() {
            row.layout_data.borrow_mut().table_data = Some(TableLayoutData {
                part: TablePart::Row,
                show_grid_lines: self.show_grid_lines,
                grid_style,
                table_column_definition: None,
            });

            if let Some(cells) = &row.cells {
                for (cell_index, cell) in cells.iter().enumerate() {
                    let column = columns.get(cell_index).unwrap_or(&default_column);
                    cell.layout_data.borrow_mut().table_data = Some(TableLayoutData {
                        part: TablePart::Row,
                        show_grid_lines: self.show_grid_lines,
                        grid_style,
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
        container_draw_override(self, context, tree, taffy_data, image.clone(), scratch)?;

        Ok(())
        // TODO: first_row_as_header
    }
}
