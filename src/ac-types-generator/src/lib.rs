use convert_case::{Case, Casing};
use core::panic;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hasher;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs};
use syn::{parse_str, Type};
use walkdir::WalkDir;

#[allow(dead_code, clippy::to_string_trait_impl, clippy::wrong_self_convention)]
mod typed_schema_types;
use typed_schema_types::{Class, Enum, EnumValue};

const PRINT_RAW: bool = false;
const MAX_ENTITIES: usize = usize::MAX;

const FALLBACK_OPTION: &str = "FallbackOption";

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
        let mut json_files = load_json_files(ac_schema_folder_path).collect::<Vec<_>>();

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
                tokens.push(process_enum(json_file.type_name, enum_));
            }
            _ => panic!("Unknown type: {}", item_type),
        }

        if tokens.len() == MAX_ENTITIES {
            break;
        }
    }

    let class_parents = get_class_parents_map(&classes_by_id);
    let class_children = get_class_children_map(&class_parents, &classes_by_id);
    let class_ancestors = flatten_map(&class_parents);
    let class_descendants = flatten_map(&class_children);

    let mut classes_ordered = classes_by_id.values().collect::<Vec<_>>();
    classes_ordered.sort_by_key(|c| c.file_name.clone());

    for loaded_class in classes_ordered.iter() {
        tokens.push(process_class(
            loaded_class,
            class_ancestors.get(&loaded_class.id),
            class_descendants.get(&loaded_class.id),
            &class_descendants,
            &classes_by_id,
            &mut generated_additional_types,
        ));

        if tokens.len() == MAX_ENTITIES {
            break;
        }
    }

    Ok(tokens)
}

fn get_class_children_map<'c>(
    class_inherits_from: &'c HashMap<String, HashSet<&'c Loaded<Class>>>,
    classes_by_id: &'c HashMap<String, Loaded<Class>>,
) -> HashMap<String, HashSet<&'c Loaded<Class>>> {
    let mut class_inheritors = HashMap::new();
    for (id, dependencies) in class_inherits_from.iter() {
        let parent_class = get_or_panic(classes_by_id, id);
        for dependency in dependencies {
            class_inheritors
                .entry(dependency.id.clone())
                .or_insert_with(HashSet::new)
                .insert(parent_class);
        }
    }
    class_inheritors
}

fn get_class_parents_map(
    classes_by_id: &HashMap<String, Loaded<Class>>,
) -> HashMap<String, HashSet<&Loaded<Class>>> {
    let mut class_inherits_from = HashMap::new();
    for (_id, loaded_class) in classes_by_id.iter() {
        if let Some(extends_str) = loaded_class.value.extends.as_ref() {
            let extends_list = extends_str
                .split(',')
                .map(|s| s.trim())
                .collect::<HashSet<_>>();
            for extends in extends_list {
                let parent_class = get_or_panic(classes_by_id, extends);
                class_inherits_from
                    .entry(loaded_class.id.clone())
                    .or_insert_with(HashSet::new)
                    .insert(parent_class);
            }
        }
    }
    class_inherits_from
}

// TODO: Test that more distant ancestors / descendants are further down the resulting vector.
fn flatten_map<'c>(
    map: &HashMap<String, HashSet<&'c Loaded<Class>>>,
) -> HashMap<String, Vec<&'c Loaded<Class>>> {
    let mut result = HashMap::new();
    for id in map.keys() {
        let mut flattened = flatten_map_for(id, map);
        flattened.sort_by_key(|d| d.file_name.clone());
        result.insert(id.clone(), flattened);
    }
    result
}

fn flatten_map_for<'c>(
    class_id: &str,
    map: &HashMap<String, HashSet<&'c Loaded<Class>>>,
) -> Vec<&'c Loaded<Class>> {
    let mut result = Vec::new();
    flatten_map_for_inner(class_id, map, &mut result);
    result
}

fn flatten_map_for_inner<'c>(
    class_id: &str,
    map: &HashMap<String, HashSet<&'c Loaded<Class>>>,
    result: &mut Vec<&'c Loaded<Class>>,
) {
    if let Some(parents) = map.get(class_id) {
        for parent in parents {
            result.push(parent);
            flatten_map_for_inner(parent.id.as_str(), map, result);
        }
    }
}

fn get_or_panic<'a>(
    classes_by_id: &'a HashMap<String, Loaded<Class>>,
    id: &str,
) -> &'a Loaded<Class> {
    let parent_class = classes_by_id
        .get(id)
        .unwrap_or_else(|| panic!("Failed to find class {}", id));
    parent_class
}

struct Loaded<T> {
    value: T,
    file_name: String,
    type_name: String,
    id: String,
}

impl<T> PartialEq for Loaded<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Loaded<T> {}

impl<T> std::hash::Hash for Loaded<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
fn load_json_files<P: AsRef<Path>>(path: P) -> impl Iterator<Item = Loaded<serde_json::Value>> {
    WalkDir::new(path).into_iter().filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.extension()?.to_str()? == "json" {
            let file = File::open(path).ok()?;
            let reader = BufReader::new(file);
            let file_name = path.file_name()?.to_str()?.to_string();
            let file_name_without_extension = path.file_stem()?.to_str()?.to_string();
            Some(Loaded::<serde_json::Value> {
                value: serde_json::from_reader(reader).ok()?,
                file_name,
                type_name: sanitize_type_ident(&file_name_without_extension),
                id: file_name_without_extension,
            })
        } else {
            None
        }
    })
}
fn common_prefix(strings: &[String]) -> String {
    if strings.len() <= 1 {
        return String::new();
    }

    let mut prefix = strings[0].clone();
    for s in strings.iter().skip(1) {
        while !s.starts_with(&prefix) {
            if prefix.is_empty() {
                return String::new();
            }
            prefix.pop();
        }
    }

    prefix
}

fn process_class(
    class: &Loaded<Class>,
    ancestors: Option<&Vec<&Loaded<Class>>>,
    descendants: Option<&Vec<&Loaded<Class>>>,
    all_descendants: &HashMap<String, Vec<&Loaded<Class>>>,
    classes_by_id: &HashMap<String, Loaded<Class>>,
    generated_additional_types: &mut Vec<String>,
) -> proc_macro2::TokenStream {
    let item_name = format_ident!("{}", class.type_name);

    let mut additional_types = Vec::new();

    let is_abstract = class.value.is_abstract.unwrap_or(false);

    let item_tokens = if is_abstract {
        let Some(descendants) = descendants else {
            panic!("Abstract class has no descendants: {}", class.id);
        };

        let variant_names: Vec<String> = descendants.iter().map(|&d| d.type_name.clone()).collect();
        // Check if there is a common prefix for all variant_names:
        let common_prefix = common_prefix(&variant_names);

        println!("Common prefix: {}", common_prefix);

        let variants = descendants.iter().map(|&d| {
            let name = format_ident!("{}", d.type_name);
            let short_name_str = d
                .type_name
                .strip_prefix(&common_prefix)
                .unwrap_or(&d.type_name);
            let short_name = format_ident!("{}", short_name_str);
            let id = &d.id;

            quote! {
                #[serde(rename = #id)]
                #short_name(Box<#name>),
            }
        });

        quote! {
            #[derive(serde::Deserialize)]
            #[serde(tag = "type")]
            pub enum #item_name {
                #(#variants)*
            }
        }
    } else {
        if descendants.is_some() {
            panic!("Non-abstract class has descendants: {}", class.id);
        }

        let mut fields = Vec::new();

        // Order is important here. More distant ancestors are after closer ones.
        let all_property_sources: Vec<&Loaded<Class>> = match ancestors {
            Some(ancestors) => std::iter::once(&class).chain(ancestors).copied().collect(),
            None => std::iter::once(class).collect(),
        };

        let mut seen_properties = HashSet::new();
        let mut all_properties = all_property_sources
            .iter()
            .flat_map(|c| &c.value.properties)
            .filter(|p| seen_properties.insert(p.0.clone()))
            .collect::<Vec<_>>();

        all_properties.sort_by_key(|p| p.0.clone());

        for p in all_properties.iter() {
            let (is_optional, name_str, json_name_str) = match p.0.strip_suffix('?') {
                Some(prefix) => (true, sanitize_field_ident(prefix), prefix),
                None => (false, sanitize_field_ident(p.0), p.0.as_str()),
            };

            let is_optional = is_optional || !p.1.required;
            let has_shorthand = !p.1.shorthands.is_empty();

            let name = format_ident!("{}", name_str);

            if name_str == "target_elements" {
                println!("{:?}", p.1.type_);
            }

            let sanitized_field_type = sanitize_type(
                &p.1.type_,
                generated_additional_types,
                is_optional,
                has_shorthand,
                all_descendants,
                classes_by_id,
            );

            if let Some(additional_type) = sanitized_field_type.additional_type {
                additional_types.push(additional_type);
            }

            let field_type_str = sanitized_field_type.type_name;
            let field_type: Type = parse_str(&field_type_str)
                .unwrap_or_else(|_| panic!("Failed to parse type: {}", field_type_str));

            let additional_attributes =
                if let Some(type_deserializer_name) = sanitized_field_type.type_deserializer_name {
                    if is_optional {
                        quote! {
                            #[serde(default, deserialize_with = #type_deserializer_name)]
                        }
                    } else {
                        quote! {
                            #[serde(deserialize_with = #type_deserializer_name)]
                        }
                    }
                } else {
                    quote! {}
                };

            if is_optional {
                fields.push(quote! {
                    #[serde(rename = #json_name_str)]
                    #additional_attributes
                    pub #name: Option<#field_type>,
                });
            } else {
                fields.push(quote! {
                    #[serde(rename = #json_name_str)]
                    #additional_attributes
                    pub #name: #field_type,
                });
            }
        }

        // Sometimes the JSON unnecessarily includes a "type" property, so add one to every struct.
        if !seen_properties.contains("type") {
            fields.push(quote! {
                #[serde(rename = "type")]
                pub type_: Option<String>,
            });
        }

        quote! {
            #[derive(serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct #item_name {
                #(#fields)*
            }
        }
    };

    let result = quote! {
        #(#additional_types)*

        #item_tokens
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

        let name_pascal_case_str = name_str.to_case(Case::Pascal);
        let alias = if &name_pascal_case_str == name_str {
            name_str.to_case(Case::Camel)
        } else {
            name_pascal_case_str
        };

        let name = format_ident!("{}", name_str.to_case(Case::Pascal));

        quote! {
            #[serde(rename = #name_str, alias = #alias)]
            #name,
        }
    });

    let result = quote! {
        #[derive(serde::Deserialize)]
        pub enum #enum_name {
            #(#variants)*
        }
    };

    println!("{}", result);

    result
}

fn sanitize_field_ident(ident: &str) -> String {
    let result = ident.replace(['.', '$'], "_").to_case(Case::Snake);

    match result.as_str() {
        "type" => "type_".to_string(),
        _ => result,
    }
}

fn sanitize_type_ident(ident: &str) -> String {
    ident.replace(['.', '$'], "_").to_case(Case::Pascal)
}

struct SanitizeTypeResult {
    type_name: String,
    additional_type: Option<proc_macro2::TokenStream>,
    type_deserializer_name: Option<String>,
}

fn get_type_has_shorthand(
    type_names: &[&str],
    all_descendants: &HashMap<String, Vec<&Loaded<Class>>>,
    classes_by_id: &HashMap<String, Loaded<Class>>,
) -> bool {
    for &t in type_names.iter() {
        if let Some(loaded_class) = classes_by_id.get(t) {
            if loaded_class.value.shorthand.is_some() {
                return true;
            }
        }

        if let Some(descendants) = all_descendants.get(t) {
            if descendants.iter().any(|d| d.value.shorthand.is_some()) {
                return true;
            }
        }
    }
    false
}

fn sanitize_type(
    type_name: &str,
    generated_additional_types: &mut Vec<String>,
    _is_optional: bool,
    property_has_shorthand: bool,
    all_descendants: &HashMap<String, Vec<&Loaded<Class>>>,
    classes_by_id: &HashMap<String, Loaded<Class>>,
) -> SanitizeTypeResult {
    let (type_name_without_suffix, type_name_suffix) = type_without_modifiers(type_name);
    let type_names = {
        let mut type_names = type_name_without_suffix.split('|').collect::<Vec<_>>();

        // If the property has a shorthand defined, or any of the types or the descendants of those types
        // have a shorthand defined, add String as a fallback.
        let type_has_shorthand =
            get_type_has_shorthand(&type_names, all_descendants, classes_by_id);

        if property_has_shorthand || type_has_shorthand {
            type_names.push("String");
        }
        type_names
    };

    if type_names.len() == 1 {
        let sanitized = sanitize_type_inner(type_name);
        return SanitizeTypeResult {
            type_name: sanitized,
            additional_type: None,
            type_deserializer_name: None,
        };
    }

    let type_name_vec = type_names
        .iter()
        .map(|&t| sanitize_type_ident(t))
        .collect::<Vec<_>>();

    let type_name_str = type_name_vec.join("Or");

    // let type_deserializer_name_str = format!(
    //     "deserialize_{}{}",
    //     type_name_vec
    //         .iter()
    //         .map(|n| n.to_case(Case::Snake))
    //         .collect::<Vec<_>>()
    //         .join("_or_"),
    //     if is_optional { "_optional" } else { "" }
    // );

    // let type_deserializer_name = format_ident!("{}", type_deserializer_name_str);
    let type_name = format_ident!("{}", type_name_str);

    let use_box = type_names.iter().any(|&t| t == FALLBACK_OPTION);

    if generated_additional_types.contains(&type_name_str) {
        return SanitizeTypeResult {
            type_name: type_name_str,
            additional_type: None,
            type_deserializer_name: None, // Some(type_deserializer_name_str),
        };
    }

    generated_additional_types.push(type_name_str.clone());

    let (variants, from_impls): (Vec<_>, Vec<_>) = type_names
        .iter()
        .map(|&t| {
            let name = format_ident!("{}", sanitize_type_ident(t));
            let inner_type: Type = parse_str(&sanitize_type_inner(t))
                .unwrap_or_else(|_| panic!("Failed to parse type: {}", t));

            if use_box && t != FALLBACK_OPTION {
                return (
                    quote! {
                        #name(Box<#inner_type>),
                    },
                    quote! {
                        impl From<#inner_type> for #type_name {
                            fn from(value: #inner_type) -> Self {
                                #type_name::#name(Box::new(value))
                            }
                        }
                    },
                );
            }

            (
                quote! {
                    #name(#inner_type),
                },
                quote! {
                    impl From<#inner_type> for #type_name {
                        fn from(value: #inner_type) -> Self {
                            #type_name::#name(value)
                        }
                    }
                },
            )
        })
        .unzip();

    let additional_type = quote! {

        // use super::utils::#type_deserializer_name;

        pub enum #type_name {
            #(#variants)*
        }

        #(#from_impls)*
    };

    SanitizeTypeResult {
        type_name: sanitize_type_inner(&format!("{}{}", type_name_str, type_name_suffix)),
        additional_type: Some(additional_type),
        type_deserializer_name: None, // Some(type_deserializer_name_str),
    }
}

fn type_without_modifiers(type_name: &str) -> (&str, &str) {
    if type_name.strip_suffix("[]?").is_some() {
        return (type_name.strip_suffix("[]?").unwrap(), "[]?");
    }
    if type_name.strip_suffix('?').is_some() {
        return (type_name.strip_suffix('?').unwrap(), "?");
    }
    if type_name.strip_suffix("[]").is_some() {
        return (type_name.strip_suffix("[]").unwrap(), "[]");
    }
    (type_name, "")
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
        _ => sanitize_type_ident(type_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_generate_tokens() {
        let tokens = generate_inner(PathBuf::from("../../adaptive-cards-data/schema/typed-schema-1.6")).unwrap();

        assert!(!tokens.is_empty());
    }
}
