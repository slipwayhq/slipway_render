use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Run the `cargo typify` command
    let output = Command::new("cargo")
        .arg("typify")
        .arg("../../adaptive-cards-data/schema/typed-schema.schema.json")
        .output()
        .expect("Failed to execute cargo typify");

    if !output.status.success() {
        panic!("cargo typify failed with error: {:?}", output);
    }

    let src = Path::new("../../adaptive-cards-data/schema/typed-schema.schema.rs");
    let dst = Path::new("./src/typed_schema_types.rs");

    if let Err(e) = fs::copy(src, dst) {
        panic!(
            "Failed to copy {} to {}: {}",
            src.display(),
            dst.display(),
            e
        );
    }

    println!("cargo:rerun-if-changed={}", src.display());
}
