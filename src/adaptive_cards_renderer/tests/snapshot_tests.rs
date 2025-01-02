use std::path::Path;

use adaptive_cards_host_config::HostConfig;
use adaptive_cards_renderer::{
    default_host_config,
    host_context::{HostContext, ResolvedFont},
    render::render_from_str,
    DebugMode,
};
use image::RgbaImage;

const CARD_EXTENSION: &str = ".card.json";

#[test]
fn snapshots() {
    let search_path = format!("snapshot_inputs/*{}", CARD_EXTENSION);

    insta::with_settings!({}, {
        insta::glob!(&search_path, |path| {
            let path_string = path.to_string_lossy();
            let card_prefix = path_string.strip_suffix(CARD_EXTENSION).unwrap();

            let spec = load_spec_file_for(card_prefix);

            println!("Running snapshot test for {}", card_prefix);

            let json_data = std::fs::read_to_string(path).unwrap();
            let (image, card) = render_from_str(
                &json_data,
                &spec.host_config.unwrap_or_else(default_host_config),
                &MockHostContext {},
                spec.width,
                spec.height,
                DebugMode::none(),
            )
            .unwrap();

            write_image_for(card_prefix, &image);

            insta::assert_json_snapshot!(card);
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

#[derive(Debug, serde::Deserialize)]
struct SnapshotTestSpec {
    width: u32,
    height: u32,
    host_config: Option<HostConfig>,
}

// For our tests we're always going to use Roboto, so the results are consistent across platforms.
const ROBOTO_TTF: &[u8] = include_bytes!("../../../fonts/Roboto.ttf");
const ROBOTO_MONO_TTF: &[u8] = include_bytes!("../../../fonts/RobotoMono.ttf");
struct MockHostContext {}
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

    fn run_callout(
        &self,
        _handle: &str,
        _input: &serde_json::Value,
    ) -> Result<RgbaImage, adaptive_cards_renderer::host_context::ComponentError> {
        unimplemented!()
    }

    fn load_image_from_url(
        &self,
        _url: &str,
    ) -> Result<RgbaImage, adaptive_cards_renderer::host_context::ComponentError> {
        unimplemented!()
    }

    fn warn(&self, message: &str) {
        println!("Warning: {}", message);
    }
}
