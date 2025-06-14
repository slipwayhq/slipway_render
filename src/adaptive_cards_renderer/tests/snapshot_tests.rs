use std::{cell::RefCell, path::Path};

use adaptive_cards::AdaptiveCard;
use adaptive_cards_host_config::HostConfig;
use adaptive_cards_renderer::{
    DebugMode, ElementLayoutData, default_host_config,
    host_context::{ComponentError, HostContext, ResolvedFont},
    render::render_from_str,
};
use image::RgbaImage;

const CARD_EXTENSION: &str = ".card.json";

#[test]
fn snapshots() {
    // Get test name from SNAPSHOT_TEST_NAME environment variable.
    let search_path = std::env::var("SNAPSHOT_TEST_NAME")
        .map(|name| format!("snapshot_inputs/{}{}", name, CARD_EXTENSION))
        .unwrap_or_else(|_| format!("snapshot_inputs/*{}", CARD_EXTENSION));

    let debug_mode = match std::env::var("SNAPSHOT_DEBUG_MODE").as_deref() {
        Ok("full") => DebugMode::full(),
        Ok("outlines") => DebugMode::with_outlines(),
        Ok("transparent_masks") => DebugMode::with_transparent_masks(),
        _ => DebugMode::none(),
    };

    insta::with_settings!({}, {
        insta::glob!(&search_path, |path| {
            let path_string = path.to_string_lossy();
            let card_prefix = path_string.strip_suffix(CARD_EXTENSION).unwrap();

            let spec = load_spec_file_for(card_prefix);

            println!("Running snapshot test for {}", card_prefix);

            let json_data = std::fs::read_to_string(path).unwrap();

            let mock_host_context = MockHostContext {
                debug_lines: RefCell::new(Vec::new()),
            };
            let (image, card) = render_from_str(
                &json_data,
                &spec.host_config.unwrap_or_else(default_host_config),
                &mock_host_context,
                spec.width,
                spec.height,
                debug_mode,
            )
            .unwrap();

            write_image_for(card_prefix, &image);

            let snapshot = Snapshot {
                card,
                debug_lines: mock_host_context.debug_lines.into_inner(),
            };

            insta::assert_json_snapshot!(snapshot);
        });
    });
}

fn write_image_for(card_prefix: &str, image: &RgbaImage) {
    let image_path = format!("{}{}", card_prefix, ".png");
    image.save(image_path).unwrap();
}

fn load_spec_file_for(card_prefix: &str) -> SnapshotTestSpec {
    let spec_file_string = format!("{}{}", card_prefix, ".spec.json");
    let spec_file_path = Path::new(&spec_file_string);
    if spec_file_path.exists() {
        serde_json::from_str::<SnapshotTestSpec>(&std::fs::read_to_string(spec_file_path).unwrap())
            .unwrap()
    } else {
        SnapshotTestSpec {
            width: 1000,
            height: 800,
            host_config: None,
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct Snapshot {
    card: AdaptiveCard<ElementLayoutData>,
    debug_lines: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct SnapshotTestSpec {
    width: u32,
    height: u32,
    host_config: Option<HostConfig>,
}

// For our tests we're always going to use Roboto, so the results are consistent across platforms.
const ROBOTO_TTF: &[u8] = include_bytes!("../../../fonts/Roboto.ttf");
const ROBOTO_MONO_TTF: &[u8] = include_bytes!("../../../fonts/RobotoMono.ttf");
const AIRPLANE_PNG: &[u8] =
    include_bytes!("../../../src/adaptive_cards_renderer/tests/assets/airplane.png");
struct MockHostContext {
    debug_lines: RefCell<Vec<String>>,
}
impl HostContext for MockHostContext {
    fn try_resolve_font(&self, family: &str) -> Option<ResolvedFont> {
        if family.to_lowercase().contains("mono") {
            Some(ResolvedFont {
                family: "Roboto Mono".to_string(),
                data: ROBOTO_MONO_TTF.to_vec(),
            })
        } else {
            Some(ResolvedFont {
                family: "Roboto".to_string(),
                data: ROBOTO_TTF.to_vec(),
            })
        }
    }

    fn load_image_from_url(
        &self,
        url: &str,
        _body: Option<&serde_json::Value>,
    ) -> Result<RgbaImage, ComponentError> {
        match url {
            "https://adaptivecards.io/content/airplane.png" => {
                Ok(adaptive_cards_renderer::image_to_premultiplied_alpha(
                    image::load_from_memory(AIRPLANE_PNG).unwrap().to_rgba8(),
                ))
            }
            _ => panic!("Image not found: {}", url),
        }
    }

    fn log_warn(&self, message: &str) {
        println!("Warning: {}", message);
    }

    fn log_debug(&self, message: &str) {
        self.debug_lines.borrow_mut().push(message.to_string());
    }
}
