use std::io::{Read, Write};

use adaptive_cards_host_config::HostConfig;
use serde::{Deserialize, Serialize};

const COLOR_DARK: &str = include_str!("../themes/color_dark.json");
const COLOR_LIGHT: &str = include_str!("../themes/color_light.json");

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

    // By deserializing to the HostConfig struct we ensure that all default values are populated
    // before we serialize it back to JSON.
    // This makes it more intuitive to modify the theme using the `modify` component, because you're
    // modifying the final JSON structure which the renderer will use.
    let host_config: HostConfig = serde_json::from_str(theme_str).expect("should parse theme JSON");

    let output = Output { host_config };

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer(&mut handle, &output).expect("should serialize JSON to stdout");
    handle.flush().expect("should flush stdout");
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
    #[serde(rename = "hostConfig")]
    host_config: HostConfig,
}
