use std::path::Path;

use adaptive_cards_renderer::{default_host_config, render, DebugMode};
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

            let json_data = std::fs::read_to_string(path).unwrap();
            let (image, card) = render(
                &default_host_config(),
                &json_data,
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
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct SnapshotTestSpec {
    width: u32,
    height: u32,
}
