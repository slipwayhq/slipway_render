use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    vec,
};
use walkdir::WalkDir;

use crate::AdaptiveCard;

#[test]
fn it_should_parse_samples() {
    // Ideally we'd get samples which are expected to fail from here:
    // https://github.com/microsoft/AdaptiveCards/blob/main/source/nodejs/tests/test-adaptive-card-schema/src/test-adaptive-card-schema.ts
    // However the list is out of date so instead we validate all samples against the
    // adaptive cards schema and expect them to fail to parse if the fail to validate.
    let compiled_schema = load_adaptive_cards_schema();

    let sample_dir_root = PathBuf::from("../../adaptive-cards-data/samples");
    let ignore_folders = vec!["HostConfig", "Templates", "v1.6"];

    let mut fail_count = 0;
    for sample_dir in get_folders_except(&sample_dir_root, &ignore_folders) {
        parse_samples_in_folder(&sample_dir, &compiled_schema, &mut fail_count);
    }

    assert!(fail_count > 0, "Expected at least one failure to parse.",);
}

fn load_adaptive_cards_schema() -> JSONSchema {
    // Read the JSON Schema file
    let mut schema_file =
        File::open("../../adaptive-cards-data/schema/adaptive-card.schema.json").unwrap();
    let mut schema_data = String::new();
    schema_file.read_to_string(&mut schema_data).unwrap();
    let schema: Value = serde_json::from_str(&schema_data).unwrap();

    // Compile the schema
    JSONSchema::options()
        .with_draft(Draft::Draft6)
        .compile(&schema)
        .expect("A valid schema")
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
        let maybe_card: Result<AdaptiveCard<TestLayoutData>, serde_json::Error> =
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

#[derive(Default, serde::Deserialize)]
struct TestLayoutData {
    #[allow(dead_code)]
    pub foo: String,
}
