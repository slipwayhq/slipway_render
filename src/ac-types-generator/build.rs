use std::fs;
use std::path::Path;

fn main() {
    let src = Path::new("../../ac-schema/typed-schema.schema.rs");
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
