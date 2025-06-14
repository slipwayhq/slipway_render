use adaptive_cards_host_config::HostConfig;
use adaptive_cards_renderer::{
    ElementLayoutData,
    host_context::{HostContext, ResolvedFont},
};
use image::{ImageBuffer, RgbaImage};
use serde::{Deserialize, Serialize};

wit_bindgen::generate!({
    world: "slipway",
});

struct Component;

export!(Component);

impl Guest for Component {
    fn run(input: String) -> Result<String, ComponentError> {
        let input: Input = serde_json::from_str(&input).expect("should parse JSON from stdin");

        let card: adaptive_cards::AdaptiveCard<ElementLayoutData> =
            serde_path_to_error::deserialize(input.card).map_err(|e| ComponentError {
                message: "Failed to parse Adaptive Card JSON".to_string(),
                inner: vec![format!("{e}")],
            })?;

        let (width, height) = get_render_image_size(&input.canvas);

        let image = adaptive_cards_renderer::render::render(
            &card,
            &input.host_config.unwrap_or_else(|| {
                HostConfig::builder()
                    .try_into()
                    .expect("Default host config should be valid")
            }),
            &SlipwayHostContext {},
            width,
            height,
            if input.debug {
                adaptive_cards_renderer::DebugMode::with_outlines()
            } else {
                adaptive_cards_renderer::DebugMode::none()
            },
        )
        .map_err(|e| ComponentError {
            message: "Failed to render image".to_string(),
            inner: vec![format!("{}", e)],
        })?;

        let output_image = get_output_image(&input.canvas, image);

        let output = Output {
            canvas: CanvasResult {
                width: output_image.width(),
                height: output_image.height(),
                data: slipway_host::encode_bin(output_image.into_vec().as_slice()),
            },
        };

        Ok(serde_json::to_string(&output).expect("should serialize output to JSON"))
    }
}

struct SlipwayHostContext;

impl HostContext for SlipwayHostContext {
    fn try_resolve_font(&self, family: &str) -> Option<ResolvedFont> {
        slipway_host::font(family).map(|resolved_font| ResolvedFont {
            family: resolved_font.family,
            data: resolved_font.data,
        })
    }

    fn load_image_from_url(
        &self,
        url: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<RgbaImage, adaptive_cards_renderer::host_context::ComponentError> {
        let options = match body {
            Some(body) => {
                let bytes = serde_json::to_vec(body).expect("Request body should serialize.");
                Some(slipway_host::RequestOptions {
                    method: None,
                    body: Some(bytes),
                    headers: None,
                    timeout_ms: None,
                })
            }
            None => None,
        };

        let image_result = slipway_host::fetch_bin(url, options.as_ref()).map_err(|e| {
            adaptive_cards_renderer::host_context::ComponentError {
                message: e.message,
                inner: e.inner,
            }
        })?;

        let image = if image_result.headers.iter().any(|(k, v)| {
            k.eq_ignore_ascii_case("content-type") && v.eq_ignore_ascii_case("application/json")
        }) {
            let json: serde_json::Value =
                serde_json::from_slice(&image_result.body).map_err(|e| {
                    adaptive_cards_renderer::host_context::ComponentError {
                        message: format!(
                            "Failed to parse image JSON from URL result body: {}",
                            url
                        ),
                        inner: vec![format!("{}", e)],
                    }
                })?;

            let json_object = json.as_object().ok_or_else(|| {
                adaptive_cards_renderer::host_context::ComponentError {
                    message: format!("Image JSON from URL result body is not an object: {}", url),
                    inner: vec![format!("Found: {:#?}", json)],
                }
            })?;

            if !json_object.contains_key("canvas") {
                return Err(adaptive_cards_renderer::host_context::ComponentError {
                    message: format!("Image JSON from URL result body is not a canvas: {}", url),
                    inner: vec![format!("Found: {:#?}", json_object)],
                });
            }

            let canvas_result: CanvasResultContainer =
                serde_json::from_value(json).map_err(|e| {
                    adaptive_cards_renderer::host_context::ComponentError {
                        message: "Failed to parse canvas from callout result.".to_string(),
                        inner: vec![format!("{}", e)],
                    }
                })?;

            Ok(canvas_result_to_image(&canvas_result.canvas))
        } else {
            let image = image::load_from_memory(&image_result.body).map_err(|e| {
                adaptive_cards_renderer::host_context::ComponentError {
                    message: format!("Failed to parse image bytes from URL: {}", url),
                    inner: vec![format!("{}", e)],
                }
            })?;

            // Images loaded from the internet are typically in straight alpha format,
            // so convert to premultiplied alpha for internal compositing.
            let straight_alpha_image = image.to_rgba8();
            let premultiplied_image =
                adaptive_cards_renderer::image_to_premultiplied_alpha(straight_alpha_image);

            Ok(premultiplied_image)
        }?;

        slipway_host::log_info(&format!(
            "Loaded image from URL: {} ({}x{})",
            url,
            image.width(),
            image.height()
        ));

        Ok(image)
    }

    fn log_warn(&self, message: &str) {
        slipway_host::log_warn(message);
    }

    fn log_debug(&self, message: &str) {
        slipway_host::log_debug(message);
    }
}

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
            let rgba_bytes =
                slipway_host::decode_bin(data).expect("canvas data should be valid base64");
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
    let rgba_bytes =
        slipway_host::decode_bin(&canvas.data).expect("canvas data should be valid base64");
    let image: RgbaImage = ImageBuffer::from_raw(canvas.width, canvas.height, rgba_bytes)
        .expect("canvas data should be valid image data");
    image
}

#[derive(Deserialize)]
struct Input {
    card: serde_json::Value,

    #[serde(alias = "hostConfig")]
    host_config: Option<HostConfig>,

    canvas: Canvas,

    #[serde(default)]
    debug: bool,
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
