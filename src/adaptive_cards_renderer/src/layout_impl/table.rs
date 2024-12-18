use std::{cell::RefCell, rc::Rc};

use adaptive_cards::{StringOrNumber, Table, TableColumnDefinition};
use imageproc::drawing::draw_hollow_rect_mut;
use taffy::{Dimension, TaffyTree};

use crate::{
    element_layout_data::{ElementTaffyData, TableLayoutData},
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::{ContainerStyleToConfig, StringToColor},
    layout_context::LayoutContext,
    layout_scratch::LayoutScratch,
    masked_image::MaskedImage,
    utils::TaffyLayoutUtils,
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

        let grid_lines_color =
            get_grid_lines_color(context, self.show_grid_lines, self.grid_style)?;

        self.layout_data.borrow_mut().table_data = Some(TableLayoutData {
            no_border: true,
            grid_lines_color,
            table_column_definition: None,
        });

        // Set the column definition on all the cells.
        for row in rows.iter() {
            row.layout_data.borrow_mut().table_data = Some(TableLayoutData {
                no_border: true,
                grid_lines_color,
                table_column_definition: None,
            });

            if let Some(cells) = &row.cells {
                for (cell_index, cell) in cells.iter().enumerate() {
                    let column = columns.get(cell_index).unwrap_or(&default_column);
                    cell.layout_data.borrow_mut().table_data = Some(TableLayoutData {
                        no_border: true,
                        grid_lines_color,
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
        {
            let border_color =
                get_grid_lines_color(context, self.show_grid_lines, self.grid_style)?;
            let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
            let rect = node_layout.absolute_rect(context);
            let mut image_mut = image.borrow_mut();
            draw_hollow_rect_mut(&mut *image_mut, rect, border_color);
        }

        container_draw_override(self, context, tree, taffy_data, image, scratch)?;

        Ok(())
        // TODO: first_row_as_header
    }
}

fn get_grid_lines_color(
    context: &LayoutContext<'_, '_, '_>,
    show_grid_lines: bool,
    grid_style: adaptive_cards::ContainerStyle,
) -> Result<image::Rgba<u8>, RenderError> {
    let separator_color = if show_grid_lines {
        let style_config = context.host_config.container_styles.from(grid_style);
        if let Some(border_color_str) = style_config
            .table_grid_lines_color
            .as_ref()
            .or(style_config.border_color.as_ref())
        {
            Some(border_color_str.to_color()?)
        } else {
            None
        }
    } else {
        None
    };
    Ok(separator_color.unwrap_or(image::Rgba::<u8>([0, 0, 0, 0])))
}
