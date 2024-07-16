use core::panic;
use load::Loaded;
use quote::quote;
use relationships::get_relationships;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{env, fs};

mod load;
mod process_class;
mod process_enum;
mod relationships;
mod sanitize;

#[allow(dead_code, clippy::to_string_trait_impl, clippy::wrong_self_convention)]
mod typed_schema_types;
use typed_schema_types::{Class, Enum};

const PRINT_RAW: bool = false;

pub fn generate(in_path: PathBuf, out_path: PathBuf) -> anyhow::Result<()> {
    let tokens = generate_inner(in_path)?;
    let output = quote! {
        #![allow(dead_code)]
        #(#tokens)*
    };

    if PRINT_RAW {
        fs::write(out_path, output.to_string()).unwrap();
    } else {
        let syntax_tree = syn::parse2(output).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        fs::write(out_path, formatted).unwrap();
    }
    Ok(())
}

fn generate_inner(ac_schema_folder_path: PathBuf) -> anyhow::Result<Vec<proc_macro2::TokenStream>> {
    let current_dir = env::current_dir()?;
    println!(
        "Loading JSON files in {:?} relative to {:?}",
        ac_schema_folder_path, current_dir
    );
    let json_files = {
        let mut json_files = load::load_json_files(ac_schema_folder_path).collect::<Vec<_>>();

        // Make sure we don't keep changing the order in the generated code.
        json_files.sort_by_key(|f| f.file_name.clone());
        json_files
    };

    let mut generated_additional_types = Vec::new();
    let mut classes_by_id = HashMap::new();
    let mut seen_ids = HashSet::new();

    let mut tokens = Vec::new();
    for json_file in json_files {
        println!("{}", json_file.file_name);

        if seen_ids.contains(&json_file.id) {
            panic!("Duplicate id: {}", json_file.id)
        }
        seen_ids.insert(json_file.id.clone());

        let item_type = json_file.value["classType"].as_str().unwrap_or("Class");

        match item_type {
            "Class" => {
                let class: Class = serde_json::from_value(json_file.value.clone())
                    .inspect_err(|_| println!("{:?}", json_file.value))?;

                classes_by_id.insert(
                    json_file.id.clone(),
                    Loaded::<Class> {
                        value: class,
                        file_name: json_file.file_name.clone(),
                        type_name: json_file.type_name.clone(),
                        id: json_file.id.clone(),
                    },
                );
            }
            "Enum" => {
                let enum_: Enum = serde_json::from_value(json_file.value.clone())
                    .inspect_err(|_| println!("{:?}", json_file.value))?;
                tokens.push(process_enum::process_enum(json_file.type_name, enum_));
            }
            _ => panic!("Unknown type: {}", item_type),
        }
    }

    let relationships = get_relationships(&classes_by_id);

    let mut classes_ordered = classes_by_id.values().collect::<Vec<_>>();
    classes_ordered.sort_by_key(|c| c.file_name.clone());

    for loaded_class in classes_ordered.iter() {
        tokens.push(process_class::process_class(
            loaded_class,
            relationships
                .ancestors
                .get(&loaded_class.id)
                .unwrap_or_else(|| panic!("No ancestors for {}", loaded_class.id)),
            relationships
                .descendants
                .get(&loaded_class.id)
                .unwrap_or_else(|| panic!("No descendants for {}", loaded_class.id)),
            &relationships.descendants,
            &classes_by_id,
            &mut generated_additional_types,
        ));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_generate_tokens() {
        let tokens = generate_inner(PathBuf::from(
            "../../adaptive-cards-data/schema/typed-schema-1.6",
        ))
        .unwrap();

        assert!(!tokens.is_empty());
    }
}
