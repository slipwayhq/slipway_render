pub(super) struct LayoutScratch {
    parley_layout_context: parley::LayoutContext,
    parley_font_context: parley::FontContext,
    swash_scale_context: swash::scale::ScaleContext,
}

impl LayoutScratch {
    pub fn new(
        parley_layout_context: parley::LayoutContext,
        parley_font_context: parley::FontContext,
        swash_scale_context: swash::scale::ScaleContext,
    ) -> Self {
        LayoutScratch {
            parley_layout_context,
            parley_font_context,
            swash_scale_context,
        }
    }

    pub fn for_text_mut(
        &mut self,
    ) -> (
        &mut parley::LayoutContext,
        &mut parley::FontContext,
        &mut swash::scale::ScaleContext,
    ) {
        (
            &mut self.parley_layout_context,
            &mut self.parley_font_context,
            &mut self.swash_scale_context,
        )
    }
}
