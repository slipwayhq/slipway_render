use std::collections::HashMap;

use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::{parse_str, Type};

use crate::{load::Loaded, typed_schema_types::Class};

const FALLBACK_OPTION: &str = "FallbackOption";

pub(super) fn sanitize_field_ident(ident: &str) -> String {
    let result = ident.replace(['.', '$'], "_").to_case(Case::Snake);

    match result.as_str() {
        "type" => "type_".to_string(),
        _ => result,
    }
}

pub(super) fn sanitize_type_ident(ident: &str) -> String {
    ident.replace(['.', '$'], "_").to_case(Case::Pascal)
}

pub(super) struct SanitizeTypeResult {
    pub type_name: String,
    pub additional_type: Option<proc_macro2::TokenStream>,
    pub type_deserializer_name: Option<String>,
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

pub(super) fn sanitize_type(
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
        // We don't wrap in an Option, because we do that when writing the property
        // both based on the suffix and whether the property has a required field.
        return sanitize_type_inner(prefix);
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
