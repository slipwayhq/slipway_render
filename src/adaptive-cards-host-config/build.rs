use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    generate_host_config_schema_with_defaults();
    generate_host_config_types();
    copy_host_config_types();
}

fn generate_host_config_schema_with_defaults() {
    let src = PathBuf::from("../../adaptive-cards-data/schema/host-config.schema.json");
    let dst =
        PathBuf::from("../../adaptive-cards-data/schema/host-config-with-defaults.schema.json");

    populate_schema_defaults::process_schema_path(src.clone(), dst);

    println!("cargo:rerun-if-changed={}", src.display());
}

fn generate_host_config_types() {
    // Run the `cargo typify` command
    let json_src = "../../adaptive-cards-data/schema/host-config-with-defaults.schema.json";
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

fn copy_host_config_types() {
    // Copy the generated typed schema types to the src directory
    let src = Path::new("../../adaptive-cards-data/schema/host-config-with-defaults.schema.rs");
    let dst = Path::new("./src/generated.rs");

    if let Err(e) = fs::rename(src, dst) {
        panic!(
            "Failed to move {} to {}: {}",
            src.display(),
            dst.display(),
            e
        );
    }

    println!("cargo:rerun-if-changed={}", src.display());
}
