use quote::{format_ident, quote};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs};
use syn::{parse_str, Type};
use walkdir::WalkDir;

#[allow(dead_code)]
mod typed_schema_types;
use typed_schema_types::{Class, Enum, EnumValue};

const PRINT_RAW: bool = false;
const MAX_ENTITIES: usize = usize::MAX;

const FALLBACK_OPTION: &str = "FallbackOption";

pub fn generate(in_path: PathBuf, out_path: PathBuf) -> anyhow::Result<()> {
    let tokens = generate_inner(in_path)?;
    let output = quote! {
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
    let json_files = load_json_files(ac_schema_folder_path);

    let mut generated_additional_types = Vec::new();

    let mut tokens = Vec::new();
    for json_file in json_files {
        println!("{}", json_file.file_name);
        let item_type = json_file.value["classType"].as_str().unwrap_or("Class");

        match item_type {
            "Class" => {
                let class: Class = serde_json::from_value(json_file.value.clone())
                    .inspect_err(|_| println!("{:?}", json_file.value))?;
                tokens.push(process_class(
                    json_file.type_name,
                    class,
                    &mut generated_additional_types,
                ));
            }
            "Enum" => {
                let enum_: Enum = serde_json::from_value(json_file.value.clone())
                    .inspect_err(|_| println!("{:?}", json_file.value))?;
                tokens.push(process_enum(json_file.type_name, enum_));
            }
            _ => panic!("Unknown type: {}", item_type),
        }

        if tokens.len() == MAX_ENTITIES {
            break;
        }
    }

    Ok(tokens)
}

struct LoadedJson {
    value: Value,
    file_name: String,
    type_name: String,
}
fn load_json_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = LoadedJson> {
    WalkDir::new(path).into_iter().filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.extension()?.to_str()? == "json" {
            let file = File::open(path).ok()?;
            let reader = BufReader::new(file);
            let file_name = path.file_name()?.to_str()?.to_string();
            let file_name_without_extension = path.file_stem()?.to_str()?.to_string();
            Some(LoadedJson {
                value: serde_json::from_reader(reader).ok()?,
                file_name,
                type_name: sanitize_ident(&file_name_without_extension),
            })
        } else {
            None
        }
    })
}

fn process_class(
    type_name: String,
    input: Class,
    generated_additional_types: &mut Vec<String>,
) -> proc_macro2::TokenStream {
    let struct_name = format_ident!("{}", type_name);

    let mut additional_types = Vec::new();

    let mut fields = Vec::new();

    for p in input.properties.iter() {
        let (is_optional, name_str, json_name_str) = match p.0.strip_suffix('?') {
            Some(prefix) => (true, sanitize_ident(prefix), prefix),
            None => (false, sanitize_ident(p.0), p.0.as_str()),
        };

        let name = format_ident!("{}", name_str);

        let (field_type_str, additional_type) =
            sanitize_type(&p.1.type_, generated_additional_types);

        if let Some(additional_type) = additional_type {
            additional_types.push(additional_type);
        }

        let field_type: Type = parse_str(&field_type_str)
            .unwrap_or_else(|_| panic!("Failed to parse type: {}", field_type_str));
        fields.push(quote! {
            pub #name: #field_type,
        });
    }

    let result = quote! {
        #(#additional_types)*

        pub struct #struct_name {
            #(#fields)*
        }
    };

    println!("{}", result);

    result
}

fn process_enum(type_name: String, input: Enum) -> proc_macro2::TokenStream {
    let enum_name = format_ident!("{}", type_name);
    let variants = input.values.iter().map(|e| {
        let name_str = match e {
            EnumValue::Variant0(s) => s,
            EnumValue::Variant1 {
                description: _,
                value,
                version: _,
            } => value,
        };

        let name = format_ident!("{}", name_str);

        quote! {
            #name,
        }
    });

    let result = quote! {
        pub enum #enum_name {
            #(#variants)*
        }
    };

    println!("{}", result);

    result
}

fn sanitize_ident(ident: &str) -> String {
    let result = ident.replace(['.', '$'], "_");

    match result.as_str() {
        "type" => "type_".to_string(),
        _ => result,
    }
}

fn sanitize_type(
    type_name: &str,
    generated_additional_types: &mut Vec<String>,
) -> (String, Option<proc_macro2::TokenStream>) {
    // let ident = ident.replace(['-', '<', '>'], "_");

    let type_names = type_name.split('|').collect::<Vec<_>>();

    if type_names.len() == 1 {
        let sanitized = sanitize_type_inner(type_names[0]);
        return (sanitized, None);
    }

    let type_name_str = type_names
        .iter()
        .map(|&t| sanitize_ident(t))
        .collect::<Vec<_>>()
        .join("_or_");

    let use_box = type_names.iter().any(|&t| t == FALLBACK_OPTION);

    if generated_additional_types.contains(&type_name_str) {
        return (type_name_str, None);
    }

    generated_additional_types.push(type_name_str.clone());

    let variants = type_names.iter().map(|&t| {
        let name = format_ident!("{}", sanitize_ident(t));
        let inner_type: Type = parse_str(&sanitize_type_inner(t))
            .unwrap_or_else(|_| panic!("Failed to parse type: {}", t));

        if use_box && t != FALLBACK_OPTION {
            return quote! {
                #name(Box<#inner_type>),
            };
        }

        quote! {
            #name(#inner_type),
        }
    });
    let type_name = format_ident!("{}", type_name_str);
    let additional_type = quote! {
        pub enum #type_name {
            #(#variants)*
        }
    };

    (type_name_str, Some(additional_type))
}

fn sanitize_type_inner(type_name: &str) -> String {
    if let Some(prefix) = type_name.strip_suffix('?') {
        return format!("Option<{}>", sanitize_type_inner(prefix));
    }

    if let Some(prefix) = type_name.strip_suffix("[]") {
        return format!("Vec<{}>", sanitize_type_inner(prefix));
    }

    match type_name {
        "string" => "String".to_string(),
        "uri-reference" => "String".to_string(),
        "uri" => "String".to_string(),
        "number" => "f64".to_string(),
        "boolean" => "bool".to_string(),
        "Dictionary<string>" => "std::collections::HashMap<String,String>".to_string(),
        "object" => "serde_json::Value".to_string(),
        _ => sanitize_ident(type_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_generate_tokens() {
        let tokens = generate_inner(PathBuf::from("../../ac-schema/typed-schema-1.6")).unwrap();

        assert!(!tokens.is_empty());
    }
}
