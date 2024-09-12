use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let target_dir = get_target_dir();

    copy_to_out_dir("adaptive_cards_renderer_component.json", &target_dir);
    copy_to_out_dir(
        "../../adaptive-cards-data/schema/adaptive-card.schema.json",
        &target_dir,
    );
    copy_to_out_dir(
        "../../adaptive-cards-data/schema/host-config-with-defaults.schema.json",
        &target_dir,
    );
}

fn get_target_dir() -> PathBuf {
    // Get the path to the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = PathBuf::from(out_dir);

    // Determine the target directory
    let target_dir = out_dir_path
        .ancestors()
        .nth(3)
        .expect("Failed to find target directory");

    target_dir.to_owned()
}

fn copy_to_out_dir(source_file: &str, target_dir: &Path) {
    let dest_file = Path::new(target_dir).join(PathBuf::from(source_file).file_name().unwrap());

    // Copy the file
    println!("Copying {} to {:?}", source_file, dest_file);
    fs::copy(source_file, dest_file).unwrap();

    // Re-run the build script if the source file changes
    println!("cargo:rerun-if-changed={}", source_file);
}
