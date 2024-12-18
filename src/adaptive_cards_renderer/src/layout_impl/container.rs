use std::{cell::RefCell, rc::Rc};

use adaptive_cards::Container;
use taffy::{Dimension, TaffyTree};

use crate::{
    element_layout_data::{ElementLayoutData, ElementTaffyData},
    errors::RenderError,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::Layoutable,
    masked_image::MaskedImage,
};

use super::container_shared::{container_draw_override, container_layout_override};

impl Layoutable for Container<ElementLayoutData> {
    // Reference: https://github.com/AvaloniaUI/Avalonia/blob/3deddbe3050f67d2819d1710b2f1062b7b15868e/src/Avalonia.Controls/StackPanel.cs#L233
    // Reference: https://github.com/microsoft/AdaptiveCards/blob/728044c67510871445d23533fb9830ac57fbbf99/source/nodejs/adaptivecards/src/card-elements.ts#L7820-L7888

    fn layout_override(
        &self,
        context: &LayoutContext,
        mut baseline_style: taffy::Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
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
