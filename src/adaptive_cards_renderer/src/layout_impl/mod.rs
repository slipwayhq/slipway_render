use ::image::Rgba;
use adaptive_cards::{
    AdaptiveCard, Column, ColumnSet, Container, ContainerStyle, Element, HasLayoutData,
    HorizontalAlignment, StackableToggleable, Table, TableCell, TableRow, VerticalAlignment,
};
use adaptive_cards_host_config::HostConfig;
use container_shared::PaddingBehavior;

use crate::{
    element_layout_data::{ElementLayoutData, Placement, TablePart},
    errors::RenderError,
    host_config_utils::{ContainerStyleToConfig, StringToColor},
    layout_context::LayoutContext,
    layoutable::Layoutable,
    utils::ClampToU32,
    TRANSPARENT,
};

mod adaptive_card;
mod column;
mod column_set;
mod container;
mod container_shared;
mod fact_set;
mod image;
pub(crate) mod measure;
mod table;
mod table_cell;
mod table_row;
mod text_block;
mod text_shared;
mod utils;

// Unimplemented adaptive card items.
impl crate::layoutable::Layoutable for adaptive_cards::ImageSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::RichTextBlock<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::TextRun<ElementLayoutData> {}

impl crate::layoutable::Layoutable for adaptive_cards::ActionSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionExecute<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionOpenUrl<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionShowCard<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionSubmit<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::ActionToggleVisibility<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputChoiceSet<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputDate<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputNumber<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputText<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputTime<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::InputToggle<ElementLayoutData> {}
impl crate::layoutable::Layoutable for adaptive_cards::Media<ElementLayoutData> {}

enum ItemsContainerOrientation {
    Vertical,
    Horizontal,
}

trait ItemsContainer<TItem>: HasLayoutData<ElementLayoutData>
where
    TItem: StackableToggleable + Layoutable,
{
    fn children(&self) -> &[TItem];

    fn min_height(&self) -> Option<&str>;

    fn bleed(&self) -> bool;

    fn placement(&self) -> Placement;

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment>;

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment>;

    fn style(&self) -> Option<ContainerStyle>;

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::Style
    }

    fn children_collection_name(&self) -> &'static str {
        "items"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Vertical
    }

    fn apply_child_items_layout_context<'cfg, 'ctx, 'render>(
        &self,
        layout_context: LayoutContext<'cfg, 'ctx, 'render>,
    ) -> LayoutContext<'cfg, 'ctx, 'render> {
        layout_context
    }

    fn border_thickness(&self, host_config: &HostConfig) -> u32 {
        let layout_data = self.layout_data().borrow();
        let table_data = layout_data.table_data.as_ref();
        if let Some(table_data) = table_data {
            if table_data.part != TablePart::Table || !table_data.show_grid_lines {
                // If we're a row or a cell, we don't want to draw a border.
                return 0;
            }

            let style_config = host_config.container_styles.from(table_data.grid_style);

            // If we're the outer table, draw the border the same thickness as the grid lines.
            if let Some(table_grid_lines_thickness) = style_config.table_grid_lines_thickness {
                return table_grid_lines_thickness.clamp_to_u32();
            }

            // Fall back to the separator thickness.
            return host_config.separator.line_thickness.clamp_to_u32();
        }

        let maybe_style = self.style();
        if let Some(style) = maybe_style {
            let style_config = host_config.container_styles.from(style);

            if let Some(border_thickness) = style_config.border_thickness {
                if style_config.border_color.is_some() {
                    return border_thickness.clamp_to_u32();
                }
            }
        }

        0
    }

    fn border_color(&self, host_config: &HostConfig) -> Result<Rgba<u8>, RenderError> {
        let layout_data = self.layout_data().borrow();
        let table_data = layout_data.table_data.as_ref();
        if let Some(table_data) = table_data {
            let style_config = host_config.container_styles.from(table_data.grid_style);

            if table_data.part != TablePart::Table {
                // If we're a row or a cell, we don't want to draw a border.
                return Ok(TRANSPARENT);
            }

            // If we're the outer table, draw the border in the same color as the grid lines.
            if let Some(table_grid_lines_color) = style_config.table_grid_lines_color.as_ref() {
                let border_color = table_grid_lines_color.to_color()?;
                return Ok(border_color);
            }

            // Fall back to the border color.
            if let Some(border_color_str) = style_config.border_color.as_ref() {
                let border_color = border_color_str.to_color()?;
                return Ok(border_color);
            }

            // Fall back to the separator color.
            return host_config.separator.line_color.to_color();
        }

        let maybe_style = self.style();
        if let Some(style) = maybe_style {
            let style_config = host_config.container_styles.from(style);
            return style_config.border_color.to_color();
        }

        Ok(TRANSPARENT)
    }

    fn background_color(&self, host_config: &HostConfig) -> Result<Rgba<u8>, RenderError> {
        let layout_data = self.layout_data().borrow();
        let table_data = layout_data.table_data.as_ref();
        if let Some(table_data) = table_data {
            if table_data.part == TablePart::Table {
                // The style for the table only colors the grid lines, not the background.
                return Ok(TRANSPARENT);
            }
        }

        let maybe_style = self.style();
        if let Some(style) = maybe_style {
            let style_config = host_config.container_styles.from(style);

            if let Some(background_color_str) = style_config.background_color.as_ref() {
                let background_color = background_color_str.to_color()?;
                return Ok(background_color);
            }
        }

        Ok(TRANSPARENT)
    }

    fn separator_thickness(&self, host_config: &HostConfig) -> u32 {
        let layout_data = self.layout_data().borrow();
        let table_data = layout_data.table_data.as_ref();
        if let Some(table_data) = table_data {
            if table_data.part != TablePart::Cell {
                // For the table and rows, the separators used on children are the grid lines,
                // so we use the grid line thickness.
                if !table_data.show_grid_lines {
                    return 0;
                }

                let style_config = host_config.container_styles.from(table_data.grid_style);

                if let Some(table_grid_lines_thickness) = style_config.table_grid_lines_thickness {
                    return table_grid_lines_thickness.clamp_to_u32();
                }

                // Fall back to the separator thickness.
                return host_config.separator.line_thickness.clamp_to_u32();
            }
        }

        host_config.separator.line_thickness.clamp_to_u32()
    }

    fn separator_color(&self, host_config: &HostConfig) -> Result<Rgba<u8>, RenderError> {
        let layout_data = self.layout_data().borrow();
        let table_data = layout_data.table_data.as_ref();
        if let Some(table_data) = table_data {
            if table_data.part != TablePart::Cell {
                // For the table and rows, the separators used on children are the grid lines,
                // so we use the grid line color.
                if !table_data.show_grid_lines {
                    return Ok(TRANSPARENT);
                }

                let style_config = host_config.container_styles.from(table_data.grid_style);

                if let Some(table_grid_lines_color) = style_config.table_grid_lines_color.as_ref() {
                    let border_color = table_grid_lines_color.to_color()?;
                    return Ok(border_color);
                }

                // Fall back to the border color.
                if let Some(border_color_str) = style_config.border_color.as_ref() {
                    let border_color = border_color_str.to_color()?;
                    return Ok(border_color);
                }

                // Or failing that fall back to the separator color.
            }
        }

        host_config.separator.line_color.to_color()
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for AdaptiveCard<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.body.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        None
    }

    fn bleed(&self) -> bool {
        false
    }

    fn placement(&self) -> Placement {
        Placement::SoleVertical
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        None
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::Default
    }

    fn children_collection_name(&self) -> &'static str {
        "body"
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for Container<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_slice()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for Column<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }
}

impl ItemsContainer<Column<ElementLayoutData>> for ColumnSet<ElementLayoutData> {
    fn children(&self) -> &[Column<ElementLayoutData>] {
        self.columns.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        None
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }

    fn children_collection_name(&self) -> &'static str {
        "columns"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Horizontal
    }
}

impl ItemsContainer<TableRow<ElementLayoutData>> for Table<ElementLayoutData> {
    fn children(&self) -> &[TableRow<ElementLayoutData>] {
        self.rows.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        None
    }

    fn bleed(&self) -> bool {
        false
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_cell_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_cell_content_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        // Use TableLayoutData.grid_style for grid styling.
        None
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::None
    }

    fn children_collection_name(&self) -> &'static str {
        "rows"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Vertical
    }
}

impl ItemsContainer<TableCell<ElementLayoutData>> for TableRow<ElementLayoutData> {
    fn children(&self) -> &[TableCell<ElementLayoutData>] {
        self.cells.as_deref().unwrap_or_default()
    }

    fn min_height(&self) -> Option<&str> {
        None
    }

    fn bleed(&self) -> bool {
        false
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_cell_content_alignment
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.horizontal_cell_content_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::None
    }

    fn children_collection_name(&self) -> &'static str {
        "cells"
    }

    fn orientation(&self) -> ItemsContainerOrientation {
        ItemsContainerOrientation::Horizontal
    }

    fn apply_child_items_layout_context<'cfg, 'ctx, 'render>(
        &self,
        layout_context: LayoutContext<'cfg, 'ctx, 'render>,
    ) -> LayoutContext<'cfg, 'ctx, 'render> {
        let layout_data = self.layout_data().borrow();
        let table_data = layout_data.table_data.as_ref();

        if let Some(table_data) = table_data {
            if table_data.is_header {
                return layout_context.within_header();
            }
        }

        layout_context
    }
}

impl ItemsContainer<Element<ElementLayoutData>> for TableCell<ElementLayoutData> {
    fn children(&self) -> &[Element<ElementLayoutData>] {
        self.items.as_slice()
    }

    fn min_height(&self) -> Option<&str> {
        self.min_height.as_deref()
    }

    fn bleed(&self) -> bool {
        self.bleed.unwrap_or(false)
    }

    fn placement(&self) -> Placement {
        self.layout_data.borrow().placement()
    }

    fn vertical_content_alignment(&self) -> Option<VerticalAlignment> {
        self.vertical_content_alignment.or_else(|| {
            self.layout_data
                .borrow()
                .table_column_definition()
                .vertical_cell_content_alignment
        })
    }

    fn horizontal_content_alignment(&self) -> Option<HorizontalAlignment> {
        self.layout_data
            .borrow()
            .table_column_definition()
            .horizontal_cell_content_alignment
    }

    fn style(&self) -> Option<ContainerStyle> {
        self.style
    }

    fn padding_behavior(&self) -> PaddingBehavior {
        PaddingBehavior::Narrow
    }
}
