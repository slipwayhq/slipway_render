use std::io::Read;

use serde::{Deserialize, Serialize};

const COLOR_DARK: &str = include_str!("../themes/color_dark.json");
const COLOR_LIGHT: &str = include_str!("../themes/color_dark.json");

#[no_mangle]
pub fn step() {
    let mut input_string = String::new();

    std::io::stdin()
        .read_to_string(&mut input_string)
        .expect("should read from stdin");

    let input: Input = serde_json::from_str(&input_string).expect("should parse JSON from stdin");

    let theme_str = match input.name {
        ThemeName::ColorDark => COLOR_DARK,
        ThemeName::ColorLight => COLOR_LIGHT,
    };

    let host_config = serde_json::from_str(theme_str).expect("should parse theme JSON");

    let output = Output { host_config };

    let stdout = std::io::stdout();
    let handle = stdout.lock();
    serde_json::to_writer(handle, &output).expect("should serialize JSON to stdout");
}

#[derive(Deserialize)]
struct Input {
    name: ThemeName,
}

#[derive(Deserialize)]
enum ThemeName {
    #[serde(alias = "color_dark")]
    ColorDark,

    #[serde(alias = "color_light")]
    ColorLight,
}

#[derive(Serialize)]
struct Output {
    #[serde(alias = "hostConfig")]
    host_config: serde_json::Value,
}
