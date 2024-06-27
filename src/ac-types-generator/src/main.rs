use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[allow(dead_code)]
mod typed_schema_types;
use typed_schema_types::{Class, Enum};

fn main() -> anyhow::Result<()> {
    let current_dir = env::current_dir()?;
    let path = PathBuf::from("../ac-schema/typed-schema-1.6/");
    println!(
        "Loading JSON files in {:?} relative to {:?}",
        path, current_dir
    );
    let json_files = load_json_files(path);

    for json_file in json_files {
        println!("{}: {:?}", json_file.file_name, json_file.value);
        let item_type = json_file.value["classType"].as_str().unwrap_or("Class");

        match item_type {
            "Class" => {
                let class: Class = serde_json::from_value(json_file.value)?;
                process_class(json_file.file_name, class);
            }
            "Enum" => {
                let enum_: Enum = serde_json::from_value(json_file.value)?;
                process_enum(json_file.file_name, enum_);
            }
            _ => panic!("Unknown type: {}", item_type),
        }
    }

    Ok(())
}

fn load_json_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = LoadedJson> {
    WalkDir::new(path).into_iter().filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.extension()?.to_str()? == "json" {
            let file = File::open(path).ok()?;
            let reader = BufReader::new(file);
            let file_name = path.file_name()?.to_str()?.to_string();
            Some(LoadedJson {
                value: serde_json::from_reader(reader).ok()?,
                file_name,
            })
        } else {
            None
        }
    })
}

fn process_class(file_name: String, value: Class) {
    println!("Processing class: {}", file_name);
}

fn process_enum(file_name: String, value: Enum) {
    println!("Processing enum: {}", file_name);
}

struct LoadedJson {
    value: Value,
    file_name: String,
}
