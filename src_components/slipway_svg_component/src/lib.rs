use base64::prelude::*;
use std::sync::Arc;

use image::RgbaImage;
use resvg::tiny_skia::{self, Pixmap};
use serde::{Deserialize, Serialize};
use usvg::fontdb;

#[allow(warnings)]
mod bindings;

use bindings::{ComponentError, Guest};

struct Component;

impl Guest for Component {
    fn run(input: String) -> Result<String, ComponentError> {
        let input: Input = serde_json::from_str(&input).expect("should parse JSON from stdin");

        let mut pixels = Pixmap::new(input.width, input.height).ok_or(ComponentError {
            message: "Rendered image cannot be greater than i32::MAX/4".to_string(),
        })?;

        let options = usvg::Options {
            font_resolver: create_font_resolver(),
            ..usvg::Options::default()
        };

        let tree: usvg::Tree =
            usvg::Tree::from_data(input.svg.as_bytes(), &options).map_err(|error| {
                ComponentError {
                    message: error.to_string(),
                }
            })?;

        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixels.as_mut());

        let img = RgbaImage::from_vec(input.width, input.height, pixels.take()).ok_or(
            ComponentError {
                message: "Could not create ImageBuffer from bytes".to_string(),
            },
        )?;

        let output = Output {
            canvas: CanvasResult {
                width: img.width(),
                height: img.height(),
                data: BASE64_STANDARD.encode(img.to_vec()),
            },
        };

        Ok(serde_json::to_string(&output).expect("should serialize output to JSON"))
    }
}

pub fn create_font_resolver<'a>() -> usvg::FontResolver<'a> {
    usvg::FontResolver {
        select_font: slipway_font_selector(),
        select_fallback: usvg::FontResolver::default_fallback_selector(),
    }
}

pub fn slipway_font_selector() -> usvg::FontSelectionFn<'static> {
    Box::new(move |font, fontdb| {
        let mut name_list = Vec::new();
        for family in font.families() {
            let (family_str, family) = match family {
                usvg::FontFamily::Serif => ("serif", fontdb::Family::Serif),
                usvg::FontFamily::SansSerif => ("sans-serif", fontdb::Family::SansSerif),
                usvg::FontFamily::Cursive => ("cursive", fontdb::Family::Cursive),
                usvg::FontFamily::Fantasy => ("fantasy", fontdb::Family::Fantasy),
                usvg::FontFamily::Monospace => ("monospace", fontdb::Family::Monospace),
                usvg::FontFamily::Named(s) => (s.as_str(), fontdb::Family::Name(s)),
            };

            name_list.push(family);

            let query = fontdb::Query {
                families: &[family],
                weight: fontdb::Weight::default(),
                stretch: fontdb::Stretch::default(),
                style: fontdb::Style::default(),
            };

            let id = fontdb.query(&query);

            if id.is_none() {
                bindings::log::warn(&format!(
                    "No match for '{}' font-family. Requesting from host.",
                    family_str,
                ));
                let maybe_resolved_font = bindings::font::try_resolve(family_str);
                if let Some(resolved_font) = maybe_resolved_font {
                    bindings::log::warn(&format!(
                        "Found '{}' font-family in host: {}",
                        family_str, resolved_font.family
                    ));
                    Arc::make_mut(fontdb).load_font_data(resolved_font.data);
                    let id = fontdb.query(&query);
                    if id.is_some() {
                        bindings::log::warn(&format!(
                            "Re-querying found '{}' font-family.",
                            family_str
                        ));
                    } else {
                        bindings::log::warn(&format!(
                            "Re-querying didn't find '{}' font-family",
                            family_str
                        ));
                    }
                }
            }
        }

        let stretch = match font.stretch() {
            usvg::FontStretch::UltraCondensed => fontdb::Stretch::UltraCondensed,
            usvg::FontStretch::ExtraCondensed => fontdb::Stretch::ExtraCondensed,
            usvg::FontStretch::Condensed => fontdb::Stretch::Condensed,
            usvg::FontStretch::SemiCondensed => fontdb::Stretch::SemiCondensed,
            usvg::FontStretch::Normal => fontdb::Stretch::Normal,
            usvg::FontStretch::SemiExpanded => fontdb::Stretch::SemiExpanded,
            usvg::FontStretch::Expanded => fontdb::Stretch::Expanded,
            usvg::FontStretch::ExtraExpanded => fontdb::Stretch::ExtraExpanded,
            usvg::FontStretch::UltraExpanded => fontdb::Stretch::UltraExpanded,
        };

        let style = match font.style() {
            usvg::FontStyle::Normal => fontdb::Style::Normal,
            usvg::FontStyle::Italic => fontdb::Style::Italic,
            usvg::FontStyle::Oblique => fontdb::Style::Oblique,
        };

        let query = fontdb::Query {
            families: &name_list,
            weight: fontdb::Weight(font.weight()),
            stretch,
            style,
        };

        let id = fontdb.query(&query);
        if id.is_none() {
            bindings::log::warn(&format!(
                "No match for '{}' font-family.",
                font.families()
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }

        id
    })
}

bindings::export!(Component with_types_in bindings);

#[derive(Deserialize)]
struct Input {
    width: u32,
    height: u32,
    svg: String,
}

#[derive(Serialize)]
struct Output {
    canvas: CanvasResult,
}

#[derive(Serialize, Clone, Debug)]
struct CanvasResult {
    width: u32,
    height: u32,
    data: String,
}
