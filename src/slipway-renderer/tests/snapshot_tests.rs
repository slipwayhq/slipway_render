use slipway_renderer::{default_host_config, render, DebugMode};

#[test]
fn snapshots() {
    insta::with_settings!({}, {
        insta::glob!("snapshot_inputs/*.json", |path| {
            let json_data = std::fs::read_to_string(path).unwrap();
            let (image, card) = render(
                &default_host_config(),
                &json_data,
                1000,
                800,
                DebugMode::full(),
            )
            .unwrap();

            image.save(path.with_extension("png")).unwrap();
            insta::assert_json_snapshot!(card);
        });
    });
}
