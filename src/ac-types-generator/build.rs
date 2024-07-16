use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    generate_adaptive_card_typed_schema_types();
    copy_adaptive_card_typed_schema_types();
}

fn generate_adaptive_card_typed_schema_types() {
    // Run the `cargo typify` command
    let json_src = "../../adaptive-cards-data/schema/typed-schema.schema.json";
    let output = Command::new("cargo")
        .arg("typify")
        .arg(json_src)
        .output()
        .expect("Failed to execute cargo typify");

    if !output.status.success() {
        panic!("cargo typify failed with error: {:?}", output);
    }

    println!("cargo:rerun-if-changed={}", Path::new(json_src).display());
}

fn copy_adaptive_card_typed_schema_types() {
    // Copy the generated typed schema types to the src directory
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
