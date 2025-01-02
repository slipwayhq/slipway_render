use adaptive_cards_host_config::HostConfig;
use adaptive_cards_renderer::{
    host_context::{HostContext, ResolvedFont},
    ElementLayoutData,
};
use base64::prelude::*;
use image::{ImageBuffer, RgbaImage};
use serde::{Deserialize, Serialize};

#[allow(warnings)]
mod bindings;

use bindings::{ComponentError, Guest};

struct SlipwayHostContext;
impl HostContext for SlipwayHostContext {
    fn try_resolve_font(&self, family: &str) -> Option<ResolvedFont> {
        bindings::font::try_resolve(family).map(|resolved_font| ResolvedFont {
            family: resolved_font.family,
            data: resolved_font.data,
        })
    }

    fn run_callout(
        &self,
        handle: &str,
        input: &serde_json::Value,
    ) -> Result<RgbaImage, adaptive_cards_renderer::host_context::ComponentError> {
        let result = bindings::callout::run(
            handle,
            &serde_json::to_string(input).expect("Callout input should serialize."),
        )
        .map_err(|e| adaptive_cards_renderer::host_context::ComponentError {
            message: e.message,
        })?;

        let canvas_result: CanvasResultContainer = serde_json::from_str(&result).map_err(|e| {
            adaptive_cards_renderer::host_context::ComponentError {
                message: format!("Failed to parse canvas from callout result\n{}", e),
            }
        })?;

        Ok(canvas_result_to_image(&canvas_result.canvas))
    }

    fn load_image_from_url(
        &self,
        url: &str,
    ) -> Result<RgbaImage, adaptive_cards_renderer::host_context::ComponentError> {
        // If the URL is of the form `component://handle/path`...
        let image_bytes = if let Some((handle, path)) =
            url.strip_prefix("component://").and_then(|rest| {
                let mut parts = rest.splitn(2, '/');
                Some((parts.next()?, parts.next()?))
            }) {
            bindings::callout::get_bin(handle, path).map_err(|e| {
                adaptive_cards_renderer::host_context::ComponentError { message: e.message }
            })?
        } else {
            let image_result = bindings::http::request_bin(url, None).map_err(|e| {
                adaptive_cards_renderer::host_context::ComponentError { message: e.message }
            })?;

            image_result.body
        };

        let image = image::load_from_memory(&image_bytes)
            .map_err(|e| adaptive_cards_renderer::host_context::ComponentError {
                message: format!("Failed to load image from callout: {}", e),
            })?
            .to_rgba8();

        bindings::log::warn(&format!(
            "Loaded image from URL: {} ({}x{})",
            url,
            image.width(),
            image.height()
        ));
        Ok(image)
    }

    fn warn(&self, message: &str) {
        bindings::log::warn(message);
    }
}

struct Component;

impl Guest for Component {
    fn run(input: String) -> Result<String, ComponentError> {
        let input: Input = serde_json::from_str(&input).expect("should parse JSON from stdin");

        let (width, height) = get_render_image_size(&input.canvas);

        let image = adaptive_cards_renderer::render::render(
            &input.card,
            &input.host_config.unwrap_or_else(|| {
                HostConfig::builder()
                    .try_into()
                    .expect("Default host config should be valid")
            }),
            &SlipwayHostContext {},
            width,
            height,
            adaptive_cards_renderer::DebugMode::none(),
        )
        .expect("should render image");

        let output_image = get_output_image(&input.canvas, image);

        let output = Output {
            canvas: CanvasResult {
                width: output_image.width(),
                height: output_image.height(),
                data: BASE64_STANDARD.encode(output_image.to_vec()),
            },
        };

        Ok(serde_json::to_string(&output).expect("should serialize output to JSON"))
    }
}

bindings::export!(Component with_types_in bindings);

fn get_render_image_size(canvas: &Canvas) -> (u32, u32) {
    if let Some(rect) = canvas.rect {
        (rect.width, rect.height)
    } else {
        (canvas.width, canvas.height)
    }
}

fn get_output_image(canvas: &Canvas, input_image: RgbaImage) -> RgbaImage {
    if let Some(rect) = canvas.rect {
        let mut output_image = if let Some(data) = &canvas.data {
            let rgba_bytes = BASE64_STANDARD
                .decode(data)
                .expect("canvas data should be valid base64");
            let image: RgbaImage = ImageBuffer::from_raw(canvas.width, canvas.height, rgba_bytes)
                .expect("canvas data should be valid image data");
            image
        } else {
            RgbaImage::new(canvas.width, canvas.height)
        };

        for x in 0..rect.width {
            for y in 0..rect.height {
                let pixel = input_image.get_pixel(x, y);
                output_image.put_pixel(x + rect.x, y + rect.y, *pixel);
            }
        }
        output_image
    } else {
        input_image
    }
}

fn canvas_result_to_image(canvas: &CanvasResult) -> RgbaImage {
    let rgba_bytes = BASE64_STANDARD
        .decode(&canvas.data)
        .expect("canvas data should be valid base64");
    let image: RgbaImage = ImageBuffer::from_raw(canvas.width, canvas.height, rgba_bytes)
        .expect("canvas data should be valid image data");
    image
}

#[derive(Deserialize)]
struct Input {
    card: adaptive_cards::AdaptiveCard<ElementLayoutData>,

    #[serde(alias = "hostConfig")]
    host_config: Option<HostConfig>,

    canvas: Canvas,
}

#[derive(Deserialize, Clone, Debug)]
struct Canvas {
    width: u32,
    height: u32,
    data: Option<String>,
    rect: Option<Rect>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Serialize)]
struct Output {
    canvas: CanvasResult,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CanvasResult {
    width: u32,
    height: u32,
    data: String,
}

#[derive(Deserialize, Clone, Debug)]
struct CanvasResultContainer {
    canvas: CanvasResult,
}
