use std::path::PathBuf;

fn main() {
    ac_types_generator::generate(
        PathBuf::from("../../adaptive-cards-data/schema/typed-schema-1.6/"),
        PathBuf::from("./src/adaptive_cards_types/generated.rs"),
    )
    .expect("Failed to generate types");

    // Include additional build logic here if necessary
}
