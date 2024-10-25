use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::Value;

mod apply;

#[no_mangle]
pub fn step() {
    let input: Input =
        serde_json::from_reader(std::io::stdin()).expect("should parse JSON from stdin");

    let mut data = input.data;

    apply::apply_instructions(&mut data, input.instructions);

    let output = Output { data };

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer(&mut handle, &output).expect("should serialize JSON to stdout");
    handle.flush().expect("should flush stdout");
}

#[derive(Deserialize)]
struct Input {
    data: Value,
    instructions: Vec<Instruction>,
}

#[derive(Serialize)]
struct Output {
    data: Value,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Instruction {
    #[serde(rename = "set")]
    Set { path: String, value: Value },

    #[serde(rename = "delete")]
    Delete { path: String },
}
