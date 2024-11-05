use core::panic;
use itertools::Itertools;
use load::Loaded;
use proc_macro2::token_stream;
use quote::quote;
use relationships::get_relationships;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{env, fs};

mod common_prefix;
mod layout_data;
mod load;
mod process_class;
mod process_enum;
mod relationships;
mod sanitize;

#[allow(dead_code, clippy::to_string_trait_impl, clippy::wrong_self_convention)]
mod typed_schema_types;
use typed_schema_types::{Class, Enum};

const PRINT_RAW: bool = false;

struct GeneratedAdditionalTypes {
    list: Vec<GeneratedAdditionalType>,
}

impl GeneratedAdditionalTypes {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn add(
        &mut self,
        type_name: &str,
        contains_layoutable: bool,
        token_stream: token_stream::TokenStream,
    ) -> &GeneratedAdditionalType {
        self.list.push(GeneratedAdditionalType {
            type_name: type_name.to_string(),
            contains_layoutable,
            token_stream,
        });

        self.list.last().unwrap()
    }

    fn is_layoutable(&self, type_name: &str) -> bool {
        self.list
            .iter()
            .any(|t| t.type_name == type_name && t.contains_layoutable)
    }

    fn contains(&self, type_name: &str) -> bool {
        self.list.iter().any(|t| t.type_name == type_name)
    }
}

struct GeneratedAdditionalType {
    pub type_name: String,
    pub contains_layoutable: bool,
    pub token_stream: proc_macro2::TokenStream,
}

pub fn generate(in_path: PathBuf, out_path: PathBuf) -> anyhow::Result<()> {
    let tokens = generate_inner(in_path)?;
    let output = quote! {
        #(#tokens)*
    };

    if PRINT_RAW {
        fs::write(out_path, output.to_string().replace("# [", "\n#[")).unwrap();
    } else {
        let syntax_tree = syn::parse2(output).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        fs::write(out_path, formatted).unwrap();
    }
    Ok(())
}

fn generate_inner(ac_schema_folder_path: PathBuf) -> anyhow::Result<Vec<proc_macro2::TokenStream>> {
    let loaded_types = load_types(ac_schema_folder_path)?;

    let mut tokens = Vec::new();

    for loaded_enum in loaded_types
        .enums
        .values()
        .sorted_by_key(|e| e.file_name.clone())
    {
        tokens.push(process_enum::process_enum(loaded_enum));
    }

    let relationships = get_relationships(&loaded_types.classes);

    // output_class_debug_information(&relationships, &mut tokens);

    let mut generated_additional_types = GeneratedAdditionalTypes::new();

    let mut impl_as_trait_macro_tokens = Vec::new();

    for loaded_class in loaded_types
        .classes
        .values()
        .sorted_by_key(|c| c.file_name.clone())
    {
        let process_class_result = process_class::process_class(
            loaded_class,
            &relationships,
            &mut generated_additional_types,
        );
        tokens.push(process_class_result.class_tokens);
        impl_as_trait_macro_tokens.push(process_class_result.impl_as_trait_macro_tokens);
    }

    tokens.push(quote! {
        #[macro_export]
        macro_rules! impl_as_trait {
            ($trait_name:path, $as_trait_name:ident, $method_name:ident, $layout_data: ident) => {

                pub(crate) trait $as_trait_name {
                    fn $method_name(&self) -> &dyn $trait_name;
                }

                #(#impl_as_trait_macro_tokens)*
            };
        }
    });

    Ok(tokens)
}

// fn output_class_debug_information(
//     relationships: &relationships::Relationships<'_>,
//     tokens: &mut Vec<proc_macro2::TokenStream>,
// ) {
//     for class_id in relationships.classes.keys() {
//         let class = relationships.classes.get(class_id).unwrap();

//         let unique_ident = format_ident!("{}__ID", class.type_name);
//         tokens.push(quote! { const #unique_ident: &str = #class_id; });

//         let is_layoutable = relationships.is_layoutable(class_id);
//         for ancestor in relationships.ancestors.get(class_id).unwrap() {
//             let unique_ident = format_ident!("{}__{}", class.type_name, ancestor.id);
//             tokens.push(quote! { const #unique_ident: bool = #is_layoutable; });
//         }
//     }
// }

struct LoadedTypes {
    classes: HashMap<String, Loaded<Class>>,
    enums: HashMap<String, Loaded<Enum>>,
}

fn load_types(ac_schema_folder_path: PathBuf) -> anyhow::Result<LoadedTypes> {
    let current_dir = env::current_dir()?;
    println!(
        "Loading JSON files in {:?} relative to {:?}",
        ac_schema_folder_path, current_dir
    );

    let json_files = load::load_json_files(ac_schema_folder_path);

    let mut classes_by_id = HashMap::new();
    let mut enums_by_id = HashMap::new();

    let mut seen_ids = HashSet::new();
    for json_file in json_files {
        println!("{}", json_file.file_name);

        if seen_ids.contains(&json_file.id) {
            panic!("Duplicate id: {}", json_file.id)
        }
        seen_ids.insert(json_file.id.clone());

        let item_type = json_file.value["classType"].as_str().unwrap_or("Class");

        match item_type {
            "Class" => {
                add_to_map(&mut classes_by_id, &json_file)?;
            }
            "Enum" => {
                add_to_map(&mut enums_by_id, &json_file)?;
            }
            _ => panic!("Unknown type: {}", item_type),
        }
    }

    Ok(LoadedTypes {
        classes: classes_by_id,
        enums: enums_by_id,
    })
}

fn add_to_map<T>(
    map: &mut HashMap<String, Loaded<T>>,
    json_file: &Loaded<serde_json::Value>,
) -> anyhow::Result<()>
where
    T: serde::de::DeserializeOwned,
{
    let class: T = serde_json::from_value(json_file.value.clone())
        .inspect_err(|_| println!("{:?}", json_file.value))?;

    map.insert(
        json_file.id.clone(),
        Loaded::<T> {
            value: class,
            file_name: json_file.file_name.clone(),
            type_name: json_file.type_name.clone(),
            id: json_file.id.clone(),
        },
    );

    Ok(())
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
