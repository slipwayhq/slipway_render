use std::path::PathBuf;

fn main() {
    generate_adaptive_cards_types();
}

fn generate_adaptive_cards_types() {
    let src = PathBuf::from("../../adaptive-cards-data/schema/typed-schema-1.6/");
    let dst = PathBuf::from("./src/generated.rs");

    adaptive_cards_types_generator::generate(src.clone(), dst).expect("Failed to generate types");

    println!("cargo:rerun-if-changed={}", src.display());
}
