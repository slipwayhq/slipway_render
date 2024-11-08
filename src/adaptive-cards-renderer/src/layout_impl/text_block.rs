use std::{cell::RefCell, rc::Rc};

use adaptive_cards::TextBlock;
use image::Rgba;
use imageproc::{drawing::Canvas, rect::Rect};
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
    element_layout_data::{ElementLayoutData, ElementTaffyData},
    errors::{RenderError, TaffyErrorToRenderError},
    layout_context::LayoutContext,
    layout_impl::measure::NodeContext,
    layout_scratch::LayoutScratch,
    layoutable::Layoutable,
    masked_image::MaskedImage,
    utils::TaffyLayoutUtils,
};

pub(crate) struct TextBlockNodeContext {
    pub text: String,
}

impl TextBlockNodeContext {
    pub fn measure(
        &self,
        known_dimensions: taffy::geometry::Size<Option<f32>>,
        available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
        font_context: &mut parley::FontContext,
        layout_context: &mut parley::LayoutContext,
    ) -> Size<f32> {
        // https://docs.rs/parley/0.2.0/parley/
        // https://github.com/linebender/parley/blob/main/examples/swash_render/src/main.rs
        // https://github.com/DioxusLabs/taffy/blob/main/examples/cosmic_text.rs

        let width_constraint = known_dimensions.width.or(match available_space.width {
            AvailableSpace::MinContent => Some(0.0),
            AvailableSpace::MaxContent => None,
            AvailableSpace::Definite(width) => Some(width),
        });
        let text = &self.text;

        let layout = prepare_layout(layout_context, font_context, text, width_constraint);

        let width = layout.width();
        let height = layout.height();

        Size { width, height }
    }
}

impl Layoutable for TextBlock<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
    ) -> Result<ElementTaffyData, RenderError> {
        tree.new_leaf_with_context(
            Style { ..baseline_style },
            NodeContext::text(self.text.clone()),
        )
        .err_context(context)
        .map(ElementTaffyData::from)
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
        let absolute_rect = node_layout.absolute_rect(context);

        let (layout_context, font_context, scale_context) = scratch.for_text_mut();

        let width_constraint = Some(absolute_rect.width() as f32);
        let text = &self.text;

        let layout = prepare_layout(layout_context, font_context, text, width_constraint);

        let mut image_mut = image.borrow_mut();

        // Iterate over laid out lines
        for line in layout.lines() {
            // Iterate over GlyphRun's within each line
            for item in line.items() {
                if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                    render_glyph_run(scale_context, &glyph_run, &mut image_mut, absolute_rect);
                }
            }
        }
        Ok(())
    }
}

fn prepare_layout(
    layout_context: &mut parley::LayoutContext,
    font_context: &mut parley::FontContext,
    text: &String,
    width_constraint: Option<f32>,
) -> parley::Layout<[u8; 4]> {
    const DISPLAY_SCALE: f32 = 1.0;
    let mut builder = layout_context.ranged_builder(font_context, text, DISPLAY_SCALE);

    builder.push_default(FontFamily::Named("Open Sans".into()));

    // Set default styles that apply to the entire layout
    builder.push_default(StyleProperty::LineHeight(1.3));
    builder.push_default(StyleProperty::FontSize(16.0));

    // Set a style that applies to the first 4 characters
    builder.push(StyleProperty::FontWeight(FontWeight::new(600.0)), 0..4);

    // Build the builder into a Layout
    let mut layout = builder.build(text);

    // Run line-breaking and alignment on the Layout
    layout.break_all_lines(width_constraint);
    layout.align(width_constraint, Alignment::Start);
    layout
}

fn render_glyph_run(
    context: &mut ScaleContext,
    glyph_run: &GlyphRun<[u8; 4]>,
    image: &mut MaskedImage,
    absolute_rect: Rect,
) {
    // Resolve properties of the GlyphRun
    let mut run_x = glyph_run.offset() + absolute_rect.left() as f32;
    let run_y = glyph_run.baseline() + absolute_rect.top() as f32;
    let style = glyph_run.style();
    let color = style.brush;

    println!("Rendering glyph run at {}x{}", run_x, run_y);

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
        println!("Rendering glyph at relative {}x{}", glyph.x, glyph.y);
        let glyph_x = run_x + glyph.x;
        let glyph_y = run_y - glyph.y;
        run_x += glyph.advance;

        render_glyph(image, &mut scaler, color, glyph, glyph_x, glyph_y);
    }
}

fn render_glyph(
    image: &mut MaskedImage,
    scaler: &mut Scaler,
    color: [u8; 4],
    glyph: Glyph,
    glyph_x: f32,
    glyph_y: f32,
) {
    // Compute the fractional offset
    // You'll likely want to quantize this in a real renderer
    let offset = Vector::new(glyph_x.fract(), glyph_y.fract());

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
    let Ok(glyph_x) = u32::try_from(glyph_x.floor() as i32 + rendered_glyph.placement.left) else {
        return;
    };
    let Ok(glyph_y) = u32::try_from(glyph_y.floor() as i32 - rendered_glyph.placement.top) else {
        return;
    };

    println!(
        "Rendering glyph at {}x{}: {}x{}",
        glyph_x, glyph_y, glyph_width, glyph_height
    );

    match rendered_glyph.content {
        Content::Mask => {
            let mut i = 0;
            for pixel_y in 0..glyph_height {
                for pixel_x in 0..glyph_width {
                    let x = glyph_x + pixel_x;
                    let y = glyph_y + pixel_y;
                    let alpha = rendered_glyph.data[i];
                    let color = Rgba([color[0], color[1], color[2], alpha]);
                    image.draw_pixel(x, y, color);
                    i += 1;
                }
            }
        }
        Content::SubpixelMask => unimplemented!(),
        Content::Color => {
            let row_size = glyph_width as usize * 4;
            for (pixel_y, row) in rendered_glyph.data.chunks_exact(row_size).enumerate() {
                for (pixel_x, pixel) in row.chunks_exact(4).enumerate() {
                    let x = glyph_x + pixel_x as u32;
                    let y = glyph_y + pixel_y as u32;
                    let color = Rgba(pixel.try_into().expect("Not RGBA"));
                    image.draw_pixel(x, y, color);
                }
            }
        }
    };
}
