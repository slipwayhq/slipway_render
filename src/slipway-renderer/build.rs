use std::path::PathBuf;

fn main() {
    let src = PathBuf::from("../../adaptive-cards-data/schema/typed-schema-1.6/");
    let dst = PathBuf::from("./src/adaptive_cards_types/generated.rs");

    ac_types_generator::generate(src.clone(), dst).expect("Failed to generate types");

    println!("cargo:rerun-if-changed={}", src.display());
}
