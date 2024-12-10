use serde::{Deserialize, Serialize};
use serde_json::Value;

mod apply;

#[allow(warnings)]
mod bindings;

use bindings::{ComponentError, Guest};

struct Component;

impl Guest for Component {
    fn run(input: String) -> Result<String, ComponentError> {
        let input: Input = serde_json::from_str(&input).expect("should parse JSON from stdin");

        let mut data = input.data;

        apply::apply_instructions(&mut data, input.instructions);

        let output = Output { data };
        Ok(serde_json::to_string(&output).expect("should serialize output to JSON"))
    }
}

bindings::export!(Component with_types_in bindings);

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
