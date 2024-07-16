use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syn::{parse_str, Type};

use crate::relationships::Relationships;

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
}

pub(super) fn sanitize_type(
    type_name: &str,
    generated_additional_types: &mut Vec<String>,
    property_has_shorthand: bool,
    relationships: &Relationships,
) -> SanitizeTypeResult {
    // The type may end with a suffix like [] or ?, so extract it.
    let (type_name_without_suffix, type_name_suffix) = type_without_modifiers(type_name);

    // If the type is a pipe-separated list of types, we'll need to generate an enum.
    let type_names = {
        let mut type_names = type_name_without_suffix.split('|').collect::<Vec<_>>();

        // If the property has a shorthand defined, or any its descendants
        // has a shorthand defined, add String as a fallback.
        let type_has_shorthand = get_type_has_shorthand(&type_names, relationships);

        if property_has_shorthand || type_has_shorthand {
            type_names.push("String");
        }
        type_names
    };

    // Only one type, so just return the sanitized version of it.
    if type_names.len() == 1 {
        let sanitized = sanitize_type_inner(type_name);
        return SanitizeTypeResult {
            type_name: sanitized,
            additional_type: None,
        };
    }

    // Multiple types, so generate a selector enum.
    let enum_name_str = type_names
        .iter()
        .map(|&t| sanitize_type_ident(t))
        .collect::<Vec<_>>()
        .join("Or");

    if generated_additional_types.contains(&enum_name_str) {
        return SanitizeTypeResult {
            type_name: enum_name_str,
            additional_type: None,
        };
    }
    generated_additional_types.push(enum_name_str.clone());

    let additional_type = generate_enum_type_selector(&enum_name_str, type_names);

    // If the type originally had a suffix, add it back on to the enum type.
    SanitizeTypeResult {
        type_name: sanitize_type_inner(&format!("{}{}", enum_name_str, type_name_suffix)),
        additional_type: Some(additional_type),
    }
}

fn generate_enum_type_selector(
    enum_name_str: &String,
    type_names: Vec<&str>,
) -> proc_macro2::TokenStream {
    let type_name = format_ident!("{}", enum_name_str);

    // If any of the types are a fallback option, then one of the other types is likely the same type
    // as the parent struct and so we'll need to box it.
    let use_box = type_names.iter().any(|&t| t == FALLBACK_OPTION);

    let (variants, from_impls): (Vec<_>, Vec<_>) = type_names
        .iter()
        .map(|&t| {
            let name = format_ident!("{}", sanitize_type_ident(t));
            let inner_type: Type = parse_str(&sanitize_type_inner(t))
                .unwrap_or_else(|_| panic!("Failed to parse type: {}", t));

            if use_box && t != FALLBACK_OPTION {
                // This is likely the same type as the parent struct so we need to box it.
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

            // No need to box.
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
        pub enum #type_name {
            #(#variants)*
        }

        #(#from_impls)*
    };
    additional_type
}

fn get_type_has_shorthand(type_names: &[&str], relationships: &Relationships) -> bool {
    for &t in type_names.iter() {
        // If the class has a shorthand defined, return true.
        if let Some(loaded_class) = relationships.classes.get(t) {
            if loaded_class.value.shorthand.is_some() {
                return true;
            }
        }

        // If any of the class's descendants have a shorthand defined, return true.
        if let Some(descendants) = relationships.descendants.get(t) {
            if descendants.iter().any(|d| d.value.shorthand.is_some()) {
                return true;
            }
        }
    }
    false
}

fn type_without_modifiers(type_name: &str) -> (&str, &str) {
    const SUFFIXES: [&str; 3] = ["[]?", "?", "[]"];

    for suffix in SUFFIXES.iter() {
        if let Some(stripped) = type_name.strip_suffix(suffix) {
            return (stripped, suffix);
        }
    }

    (type_name, "")
}

fn sanitize_type_inner(type_name: &str) -> String {
    // If we have a question mark suffix, the type is optional.
    if let Some(prefix) = type_name.strip_suffix('?') {
        // We don't wrap in an Option here because we do that later when writing the property
        // based on both the suffix and whether the property has a required field.
        return sanitize_type_inner(prefix);
    }

    // If we have an array suffix, the type should be Vec.
    if let Some(prefix) = type_name.strip_suffix("[]") {
        return format!("Vec<{}>", sanitize_type_inner(prefix));
    }

    // Map other Typescript types to Rust types.
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
