use std::{borrow::Cow, cell::RefCell, rc::Rc};

use adaptive_cards::HorizontalAlignment;
use adaptive_cards_host_config::TextStyleConfig;
use image::Rgba;
use imageproc::{drawing::Canvas, integral_image::ArrayData};
use parley::{
    Alignment, FontFamily, FontWeight, Glyph, GlyphRun, PositionedLayoutItem, StyleProperty,
};
use swash::{
    scale::{image::Content, Render, ScaleContext, Scaler, Source, StrikeWith},
    zeno::{Format, Vector},
    FontRef,
};
use taffy::{AvailableSpace, Size, Style, TaffyTree};

use crate::{
    element_layout_data::ElementTaffyData,
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::*,
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    masked_image::MaskedImage,
    utils::{ClampToU32, TaffyLayoutUtils},
};

use super::utils::apply_horizontal_alignment;

pub(super) trait TextContainer {
    fn color(&self) -> Option<adaptive_cards::Colors>;
    fn font_type(&self) -> Option<adaptive_cards::FontType>;
    fn is_subtle(&self) -> bool;
    fn size(&self) -> Option<adaptive_cards::FontSize>;
    fn weight(&self) -> Option<adaptive_cards::FontWeight>;
    fn wrap(&self) -> bool;
    fn max_lines(&self) -> Option<u32>;
    fn max_width(&self) -> Option<f32>;
    fn horizontal_alignment(&self) -> Option<adaptive_cards::HorizontalAlignment>;
    fn text(&self) -> &str;
}

#[derive(Debug)]
pub(crate) struct TextBlockNodeContext {
    pub text: String,
    pub color: Rgba<u8>,
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: f32,
    pub max_lines: Option<u32>,
    pub wrap: bool,
    pub offset: RefCell<taffy::Point<i32>>,
    pub horizontal_alignment: HorizontalAlignment,
    pub max_width: Option<f32>,
}

impl TextBlockNodeContext {
    /// Measure using the built-in taffy size estimation.
    pub fn measure_quick(
        &mut self,
        known_dimensions: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
        context: &LayoutContext,
        scratch: &mut LayoutScratch,
    ) -> taffy::Size<f32> {
        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });

        let (layout_context, font_context, _scale_context) = scratch.for_text_mut();

        let layout = prepare_layout(
            self,
            width_constraint,
            context,
            layout_context,
            font_context,
        );

        let width = layout.width();
        let height = layout.height();

        taffy::Size { width, height }
    }

    /// Measure by computing our own pixel bounds.
    pub fn measure(
        &self,
        known_dimensions: taffy::geometry::Size<Option<f32>>,
        available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
        context: &LayoutContext,
        scratch: &mut LayoutScratch,
    ) -> Size<f32> {
        // https://docs.rs/parley/0.2.0/parley/
        // https://github.com/linebender/parley/blob/main/examples/swash_render/src/main.rs
        // https://github.com/DioxusLabs/taffy/blob/main/examples/cosmic_text.rs

        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });

        let width_constraint = if let Some(max_width) = self.max_width {
            match width_constraint {
                None => Some(max_width),
                Some(width_constraint) => Some(width_constraint.min(max_width)),
            }
        } else {
            width_constraint
        };

        println!(
            "width_constraint for text '{}': {:?}",
            self.text, width_constraint
        );

        let (layout_context, font_context, scale_context) = scratch.for_text_mut();

        let layout = prepare_layout(
            self,
            width_constraint,
            context,
            layout_context,
            font_context,
        );

        // Compute our own pixel bounds.
        // https://github.com/linebender/parley/issues/165
        let mut bounds: Option<taffy::Rect<i32>> = None;

        let mut apply_pixel = |x: i32, y: i32, _pixel: &Rgba<u8>| match bounds.as_mut() {
            None => {
                bounds = Some(taffy::Rect::<i32> {
                    left: x,
                    right: x,
                    top: y,
                    bottom: y,
                });
            }
            Some(bounds) => {
                if x < bounds.left {
                    bounds.left = x;
                }
                if x > bounds.right {
                    bounds.right = x;
                }
                if y < bounds.top {
                    bounds.top = y;
                }
                if y > bounds.bottom {
                    bounds.bottom = y;
                }
            }
        };

        render_layout(self, &mut apply_pixel, layout, scale_context);

        let offset = match bounds {
            None => taffy::Point { x: 0, y: 0 },
            Some(bounds) => taffy::Point {
                x: bounds.left.min(0), // We don't want to offset centered or right-aligned text back to the left.
                y: bounds.top,
            },
        };

        let result = match bounds {
            None => Default::default(),
            Some(bounds) => Size {
                width: (bounds.right - bounds.left + 1) as f32,
                height: (bounds.bottom - bounds.top + 1) as f32,
            },
        };

        *self.offset.borrow_mut() = offset;

        result
    }
}

pub(super) fn text_layout_override<TParent: TextContainer>(
    parent: &TParent,
    context: &LayoutContext,
    fallback_style: Option<&TextStyleConfig>,
    baseline_style: Style,
    tree: &mut TaffyTree<NodeContext>,
) -> Result<ElementTaffyData, RenderError> {
    let container_style = context
        .host_config
        .container_styles
        .from(context.inherited.style);

    let color_config = container_style.foreground_colors.from(
        parent
            .color()
            .unwrap_or_else(|| fallback_style.to_ac_color()),
    );

    let is_subtle = if parent.is_subtle() {
        true
    } else {
        // If they haven't explicitly made the text block subtle, then see if the style has it set.
        fallback_style.map(|style| style.is_subtle).unwrap_or(false)
    };

    let color = if is_subtle {
        &color_config.subtle
    } else {
        &color_config.default
    }
    .to_color()?;

    let font_type = context.host_config.font_types.from(
        parent
            .font_type()
            .unwrap_or_else(|| fallback_style.to_ac_type()),
    );

    // We need to get the resolved family, otherwise we might end up with a
    // generic name like "sans-serif" or a font stack like "Arial, sans-serif".
    let font_family = context.get_resolved_font_family(&font_type.font_family)?;

    // Font size and weight are currently integers in the schema (matching the Adaptive Cards repository).
    // We can potentially switch them to "number" once this issue is resolved:
    // https://github.com/oxidecomputer/typify/issues/442
    let font_size = font_type
        .font_sizes
        .from(parent.size().unwrap_or_else(|| fallback_style.to_ac_size()))
        .clamp_to_u32() as f32;

    let weight_identifier = parent
        .weight()
        .unwrap_or_else(|| fallback_style.to_ac_weight());

    let font_weight = font_type
        .font_weights
        .from(weight_identifier)
        .clamp_to_u32() as f32;

    let max_lines = parent.max_lines();

    let wrap = parent.wrap();

    let max_width = parent.max_width();

    // Style
    let mut style = baseline_style;
    let horizontal_alignment =
        apply_horizontal_alignment(parent.horizontal_alignment(), &mut style, context);

    let node_context = NodeContext::Text(TextBlockNodeContext {
        text: parent.text().to_string(),
        color,
        font_family,
        font_size,
        font_weight,
        max_lines,
        max_width,
        wrap,
        offset: RefCell::new(Default::default()),
        horizontal_alignment,
    });

    // Handled by parent
    // self.is_visible
    // self.separator
    // self.spacing
    // self.height

    tree.new_leaf_with_context(style, node_context)
        .err_context(context)
        .map(ElementTaffyData::from)
}

fn prepare_layout(
    text_context: &TextBlockNodeContext,
    width_constraint: Option<f32>,
    _context: &LayoutContext,
    layout_context: &mut parley::LayoutContext,
    font_context: &mut parley::FontContext,
) -> parley::Layout<[u8; 4]> {
    const DISPLAY_SCALE: f32 = 1.0;

    let text = &text_context.text;
    let mut builder = layout_context.ranged_builder(font_context, text, DISPLAY_SCALE);

    builder.push_default(FontFamily::Named(Cow::from(&text_context.font_family)));

    // Set default styles that apply to the entire layout
    builder.push_default(StyleProperty::LineHeight(1.3));
    builder.push_default(StyleProperty::FontSize(text_context.font_size));
    builder.push_default(StyleProperty::FontWeight(FontWeight::new(
        text_context.font_weight,
    )));
    builder.push_default(StyleProperty::Brush(text_context.color.data()));

    // Build the builder into a Layout
    let mut layout = builder.build(text);

    // Run line-breaking and alignment on the Layout
    let width_constraint = if text_context.wrap {
        width_constraint
    } else {
        None
    };
    layout.break_all_lines(width_constraint);
    layout.align(
        width_constraint,
        match text_context.horizontal_alignment {
            HorizontalAlignment::Center => Alignment::Middle,
            HorizontalAlignment::Right => Alignment::End,
            HorizontalAlignment::Left => Alignment::Start,
        },
    );
    layout
}

fn render_layout(
    text_context: &TextBlockNodeContext,
    apply_pixel: &mut impl FnMut(i32, i32, &Rgba<u8>),
    layout: parley::Layout<[u8; 4]>,
    scale_context: &mut swash::scale::ScaleContext,
) {
    let max_lines = text_context.max_lines.unwrap_or(u32::MAX) as usize;

    // Iterate over laid out lines
    for line in layout.lines().take(max_lines) {
        // Iterate over GlyphRun's within each line
        for item in line.items() {
            if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                render_glyph_run(apply_pixel, scale_context, &glyph_run);
            }
        }
    }
}

pub(super) fn text_draw_override(
    context: &LayoutContext,
    tree: &TaffyTree<NodeContext>,
    taffy_data: &ElementTaffyData,
    image: Rc<RefCell<MaskedImage>>,
    scratch: &mut LayoutScratch,
) -> Result<(), RenderError> {
    let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
    let absolute_rect = node_layout.absolute_rect(context);

    let node_context = tree.get_node_context(taffy_data.node_id).unwrap();
    let NodeContext::Text(text_context) = node_context; /* else {
                                                            panic!("Expected TextBlockNodeContext in TextBlock draw_override.");
                                                        };*/
    let offset = text_context.offset.borrow();
    let x_offset = absolute_rect.left() - offset.x;
    let y_offset = absolute_rect.top() - offset.y;

    let (layout_context, font_context, scale_context) = scratch.for_text_mut();

    let width_constraint = Some(absolute_rect.width() as f32);

    let layout = prepare_layout(
        text_context,
        width_constraint,
        context,
        layout_context,
        font_context,
    );

    let mut image_mut = image.borrow_mut();

    let mut apply_pixel = |x: i32, y: i32, pixel: &Rgba<u8>| {
        let Ok(x) = u32::try_from(x + x_offset) else {
            return;
        };
        let Ok(y) = u32::try_from(y + y_offset) else {
            return;
        };
        image_mut.draw_pixel(x, y, *pixel);
    };

    render_layout(text_context, &mut apply_pixel, layout, scale_context);

    Ok(())
}

fn render_glyph_run(
    apply_pixel: &mut impl FnMut(i32, i32, &Rgba<u8>),
    context: &mut ScaleContext,
    glyph_run: &GlyphRun<[u8; 4]>,
) {
    // Resolve properties of the GlyphRun
    let mut run_x = glyph_run.offset();
    let run_y = glyph_run.baseline();
    let style = glyph_run.style();
    let color = style.brush;

    // Get the "Run" from the "GlyphRun"
    let run = glyph_run.run();

    // Resolve properties of the Run
    let font = run.font();
    let font_size = run.font_size();
    let normalized_coords = run.normalized_coords();

    // Convert from parley::Font to swash::FontRef
    let font_ref = FontRef::from_index(font.data.as_ref(), font.index as usize).unwrap();

    // Build a scaler. As the font properties are constant across an entire run of glyphs
    // we can build one scaler for the run and reuse it for each glyph.
    let mut scaler = context
        .builder(font_ref)
        .size(font_size)
        .hint(true)
        .normalized_coords(normalized_coords)
        .build();

    // Iterates over the glyphs in the GlyphRun
    for glyph in glyph_run.glyphs() {
        let glyph_x = run_x + glyph.x;
        let glyph_y = run_y - glyph.y;
        run_x += glyph.advance;

        render_glyph(apply_pixel, &mut scaler, color, glyph, glyph_x, glyph_y);
    }
}

fn render_glyph(
    apply_pixel: &mut impl FnMut(i32, i32, &Rgba<u8>),
    scaler: &mut Scaler,
    color: [u8; 4],
    glyph: Glyph,
    glyph_x: f32,
    glyph_y: f32,
) {
    // Compute the fractional offset
    // You'll likely want to quantize this in a real renderer
    let offset = Vector::new(glyph_x.fract().round(), glyph_y.fract().round());

    // Render the glyph using swash
    let rendered_glyph = Render::new(
        // Select our source order
        &[
            Source::ColorOutline(0),
            Source::ColorBitmap(StrikeWith::BestFit),
            Source::Outline,
        ],
    )
    // Select the simple alpha (non-subpixel) format
    .format(Format::Alpha)
    // Apply the fractional offset
    .offset(offset)
    // Render the image
    .render(scaler, glyph.id)
    .unwrap();

    let glyph_width = rendered_glyph.placement.width;
    let glyph_height = rendered_glyph.placement.height;
    let glyph_x = glyph_x.floor() as i32 + rendered_glyph.placement.left;
    let glyph_y = glyph_y.floor() as i32 - rendered_glyph.placement.top;

    match rendered_glyph.content {
        Content::Mask => {
            let mut i = 0;
            for pixel_y in 0..glyph_height {
                for pixel_x in 0..glyph_width {
                    let x = glyph_x + pixel_x as i32;
                    let y = glyph_y + pixel_y as i32;
                    let alpha = rendered_glyph.data[i];
                    let color = Rgba([color[0], color[1], color[2], alpha]);
                    apply_pixel(x, y, &color);
                    i += 1;
                }
            }
        }
        Content::SubpixelMask => unimplemented!(),
        Content::Color => {
            let row_size = glyph_width as usize * 4;
            for (pixel_y, row) in rendered_glyph.data.chunks_exact(row_size).enumerate() {
                for (pixel_x, pixel) in row.chunks_exact(4).enumerate() {
                    let x = glyph_x + pixel_x as i32;
                    let y = glyph_y + pixel_y as i32;
                    let color = Rgba(pixel.try_into().expect("Not RGBA"));
                    apply_pixel(x, y, &color);
                }
            }
        }
    };
}
