#[allow(clippy::enum_variant_names)]
mod generated;
mod utils;

#[cfg(test)]
mod tests {
    use jsonschema::{Draft, JSONSchema};
    use serde_json::Value;
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::{Path, PathBuf},
        vec,
    };
    use walkdir::WalkDir;

    use generated::AdaptiveCard;

    use super::*;

    #[test]
    fn it_should_parse_samples() {
        // Read the JSON Schema file
        let mut schema_file =
            File::open("../../adaptive-cards-data/schema/adaptive-card.schema.json").unwrap();
        let mut schema_data = String::new();
        schema_file.read_to_string(&mut schema_data).unwrap();
        let schema: Value = serde_json::from_str(&schema_data).unwrap();

        // Compile the schema
        let compiled_schema = JSONSchema::options()
            .with_draft(Draft::Draft6)
            .compile(&schema)
            .expect("A valid schema");

        // https://github.com/microsoft/AdaptiveCards/blob/main/source/nodejs/tests/test-adaptive-card-schema/src/test-adaptive-card-schema.ts
        // let should_fail = vec![
        //     "v1.2/Scenarios/SimpleFallback.json",       // Uses custom types
        //     "v1.2/Elements/ActionFallback.json",        // Uses custom types
        //     "v1.2/Elements/Action.Submit.Style.json",   // Uses custom "other" style
        //     "v1.2/Elements/Action.ShowCard.Style.json", // Uses custom "other" style
        //     "v1.2/Elements/Action.OpenUrl.Style.json",  // Uses custom "other" style
        //     "Tests/Input.Text.InlineAction.ShowCard.json", // Purposefully incorrect for test purposes
        //     "Tests/TypeIsRequired.json",                   // Purposefully omits type
        //     "Tests/FallbackParsing.json",                  // Uses custom types
        //     "Tests/EmptyFallbackCard.json",                // Uses custom types
        //     "Tests/DeepFallback.json",                     // Uses custom types
        //     "Tests/CustomParsingTestUsingProgressBar.json", // Uses custom types
        //     "Tests/AdditionalProperty.json",               // Uses custom property
        //     "Tests/AdaptiveCard.UnknownElements.json",     // Uses custom type
        //     "Tests/Action.CustomParsing.json",             // Uses custom type
        //     "Elements/Container.Style.json",               // ADDED
        //     "Elements/Action.ShowCard.Style.json",         // ADDED
        //     "Tests/Carousel.ForbiddenActions.json",        // ADDED
        //     "Tests/Carousel.ForbiddenElements.json",       // ADDED
        //     "Tests/Carousel.HostConfig.json",              // ADDED
        //     "Tests/Carousel.NestedForbiddenElements.json", // ADDED
        //     "Tests/Carousel.Properties.json",              // ADDED
        //     "Tests/Carousel.rtl.json",                     // ADDED
        //     "Tests/Carousel.rtl.Vertical.json",            // ADDED
        //     "Tests/Carousel.SelectAction.In.ParentContainer.json", // ADDED
        //     "Tests/Carousel.ShowCard.json",                // ADDED
        //     "Tests/Carousel.Styles.json",                  // ADDED
        //     "Tests/Carousel.ToggleVisibility.json",        // ADDED
        // ];

        let sample_dir_root = PathBuf::from("../../adaptive-cards-data/samples");
        let ignore_folders = vec!["HostConfig", "Templates", "v1.6"];

        let mut fail_count = 0;
        for sample_dir in get_folders_except(&sample_dir_root, &ignore_folders) {
            parse_samples_in_folder(&sample_dir, &compiled_schema, &mut fail_count);
        }

        assert!(fail_count > 0, "Expected at least one failure to parse.",);
    }

    fn get_folders_except(sample_dir_root: &Path, except: &[&str]) -> Vec<PathBuf> {
        sample_dir_root
            .read_dir()
            .expect("Failed to read sample directory")
            .filter_map(|entry| {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.is_dir() && except.iter().all(|&e| path.file_name().unwrap() != e) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    }

    fn parse_samples_in_folder(
        sample_dir: &Path,
        compiled_schema: &JSONSchema,
        fail_count: &mut usize,
    ) {
        for json_file in load_json_files(sample_dir) {
            // Read the JSON file
            let file = File::open(&json_file).unwrap();
            let reader = BufReader::new(file);
            let json_data: Value = serde_json::from_reader(reader).unwrap();

            let should_fail = if let Err(_errors) = compiled_schema.validate(&json_data) {
                // println!(
                //     "Failed to validate {:?}: {:#?}",
                //     json_file,
                //     errors.collect::<Vec<_>>()
                // );
                true
            } else {
                false
            };

            let file = File::open(&json_file).expect("Failed to open file");
            let reader = BufReader::new(file);
            let maybe_card: Result<AdaptiveCard, serde_json::Error> =
                serde_json::from_reader(reader);

            if should_fail {
                *fail_count += 1;
                if maybe_card.is_ok() {
                    panic!("Expected to fail: {:?}", json_file);
                }
            } else if maybe_card.is_err() {
                panic!(
                    "Failed to parse {:?}: {}",
                    json_file,
                    maybe_card.err().unwrap()
                );
            }
        }
    }

    fn load_json_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = PathBuf> {
        WalkDir::new(path).into_iter().filter_map(|entry| {
            let entry = entry.ok()?;

            let path = entry.path();
            if path.is_file() && path.extension()?.to_str()? == "json" {
                Some(path.to_path_buf())
            } else {
                None
            }
        })
    }
}
