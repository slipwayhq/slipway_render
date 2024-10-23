use adaptive_cards_renderer::host_config::HostConfig;
use base64::prelude::*;
use image::{ImageBuffer, RgbaImage};
use serde::{Deserialize, Serialize};

#[no_mangle]
pub fn step() {
    let input: Input =
        serde_json::from_reader(std::io::stdin()).expect("should parse JSON from stdin");

    let (width, height) = get_render_image_size(&input.canvas);

    let image = adaptive_cards_renderer::render::render(
        &input.card,
        &input.host_config.unwrap_or_else(|| {
            HostConfig::builder()
                .try_into()
                .expect("Default host config should be valid")
        }),
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

    let stdout = std::io::stdout();
    let handle = stdout.lock();
    serde_json::to_writer(handle, &output).expect("should serialize JSON to stdout");
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

#[derive(Deserialize)]
struct Input {
    card: adaptive_cards_renderer::adaptive_cards::AdaptiveCard,

    #[serde(alias = "hostConfig")]
    host_config: Option<adaptive_cards_renderer::host_config::HostConfig>,

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

#[derive(Serialize, Clone, Debug)]
struct CanvasResult {
    width: u32,
    height: u32,
    data: String,
}
