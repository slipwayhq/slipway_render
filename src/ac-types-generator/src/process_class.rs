use std::collections::HashSet;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_str, Ident, Type};

use crate::{
    common_prefix::common_prefix,
    load::Loaded,
    relationships::Relationships,
    sanitize::{sanitize_field_ident, sanitize_type},
    typed_schema_types::Class,
};

const LAYOUTABLE_TYPES: [&str; 2] = ["Element", "ActiveCard"];

pub(super) fn process_class(
    class: &Loaded<Class>,
    relationships: &Relationships,
    generated_additional_types: &mut Vec<String>,
) -> proc_macro2::TokenStream {
    let ancestors = relationships
        .ancestors
        .get(&class.id)
        .unwrap_or_else(|| panic!("No ancestors for {}", class.id));
    let descendants = relationships
        .descendants
        .get(&class.id)
        .unwrap_or_else(|| panic!("No descendants for {}", class.id));

    let struct_name = format_ident!("{}", class.type_name);

    let mut pre_struct_tokens = Vec::new();
    let mut post_struct_tokens = Vec::new();

    let is_abstract = class.value.is_abstract.unwrap_or(false);

    let is_layoutable = LAYOUTABLE_TYPES.contains(&class.type_name.as_str())
        || ancestors
            .iter()
            .any(|a| LAYOUTABLE_TYPES.contains(&a.type_name.as_str()));

    let struct_tokens = if is_abstract {
        if descendants.is_empty() {
            panic!("Abstract class has no descendants: {}", class.id);
        };

        let mut descendants = descendants.iter().collect::<Vec<_>>();
        descendants.sort_by_key(|d| &d.file_name);

        let variant_names: Vec<String> = descendants.iter().map(|&d| d.type_name.clone()).collect();

        // Check if there is a common prefix for all variant_names:
        let common_prefix = common_prefix(&variant_names);

        println!("Common prefix: {}", common_prefix);

        struct VariantInfo {
            ident: Ident,
            is_abstract: bool,
        }

        let (variants, variant_infos): (Vec<TokenStream>, Vec<VariantInfo>) = descendants
            .iter()
            .map(|&d| {
                let name = format_ident!("{}", d.type_name);
                let short_name_str = d
                    .type_name
                    .strip_prefix(&common_prefix)
                    .unwrap_or(&d.type_name);
                let short_name = format_ident!("{}", short_name_str);
                let id = &d.id;

                (
                    quote! {
                        #[serde(rename = #id)]
                        #short_name(Box<#name>),
                    },
                    VariantInfo {
                        ident: short_name,
                        is_abstract: d.value.is_abstract.unwrap_or(false),
                    },
                )
            })
            .unzip();

        if is_layoutable {
            let match_tokens = variant_infos.iter().map(|v| {
                let variant_ident = &v.ident;

                if v.is_abstract {
                    quote! {
                        #struct_name::#variant_ident(inner) => inner.as_layoutable(),
                    }
                } else {
                    quote! {
                        #struct_name::#variant_ident(inner) => inner,
                    }
                }
            });

            post_struct_tokens.push(quote! {
                impl #struct_name {
                    pub fn as_layoutable(&self) -> &dyn crate::Layoutable {
                        match self {
                            #(#match_tokens)*
                        }
                    }
                }
            });
        }

        quote! {
            #[derive(serde::Deserialize)]
            #[serde(tag = "type")]
            pub enum #struct_name {
                #(#variants)*
            }
        }
    } else {
        if !descendants.is_empty() {
            panic!("Non-abstract class has descendants: {}", class.id);
        }

        let mut fields = Vec::new();

        // Order is important here. More distant ancestors are after closer ones.
        let all_property_sources: Vec<&Loaded<Class>> =
            std::iter::once(&class).chain(ancestors).copied().collect();

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

            let default_value = p.1.default.clone();
            let has_default_value = default_value.is_some();
            let is_optional = (is_optional || !p.1.required) && !has_default_value;
            let has_shorthand = !p.1.shorthands.is_empty();

            let name = format_ident!("{}", name_str);

            if name_str == "target_elements" {
                println!("{:?}", p.1.type_);
            }

            let sanitized_field_type = sanitize_type(
                &p.1.type_,
                generated_additional_types,
                has_shorthand,
                relationships,
            );

            if let Some(additional_type) = sanitized_field_type.additional_type {
                pre_struct_tokens.push(additional_type);
            }

            let field_type_str = sanitized_field_type.type_name;
            let field_type: Type = parse_str(&field_type_str)
                .unwrap_or_else(|_| panic!("Failed to parse type: {}", field_type_str));

            if is_optional {
                fields.push(quote! {
                    #[serde(rename = #json_name_str)]
                    pub #name: Option<#field_type>,
                });
            } else if let Some(default_value) = default_value {
                let default_value_function = format!("default_value_for_{}", name);
                let default_value_function_qualified =
                    format!("{}::{}", struct_name, default_value_function);
                let default_value_function_ident = format_ident!("{}", default_value_function);
                let default_value_impl =
                    get_default_value_impl(field_type_str, default_value, &field_type);

                let default_value_function_tokens = quote! {
                    impl #struct_name {
                        fn #default_value_function_ident() -> #field_type {
                            #default_value_impl
                        }
                    }
                };

                post_struct_tokens.push(default_value_function_tokens);

                fields.push(quote! {
                    #[serde(rename = #json_name_str, default = #default_value_function_qualified)]
                    pub #name: #field_type,
                });
            } else {
                fields.push(quote! {
                    #[serde(rename = #json_name_str)]
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

        if is_layoutable {
            fields.push(quote! {
                #[serde(skip)]
                pub layout_data: core::cell::RefCell<crate::LayoutData>,
            });

            post_struct_tokens.push(quote! {
                impl crate::Layoutable for #struct_name {
                    fn layout_data(&self) -> &core::cell::RefCell<crate::LayoutData> {
                        &self.layout_data
                    }
                }
            });
        }

        quote! {
            #[derive(serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            pub struct #struct_name {
                #(#fields)*
            }
        }
    };

    let result = quote! {
        #(#pre_struct_tokens)*
        #struct_tokens
        #(#post_struct_tokens)*
    };

    println!("{}", result);

    result
}

fn get_default_value_impl(
    field_type_str: String,
    default_value: serde_json::Value,
    field_type: &Type,
) -> proc_macro2::TokenStream {
    match field_type_str.as_str() {
        "String" => {
            let default_value = default_value
                .as_str()
                .unwrap_or_else(|| panic!("Expected string default value: {:?}", default_value));
            quote! { String::from(#default_value) }
        }
        "bool" => {
            let default_value = default_value.as_bool().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            quote! { #default_value }
        }
        "u32" | "u64" => {
            let default_value = default_value.as_u64().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            quote! { #default_value }
        }
        "f32" | "f64" => {
            let default_value = default_value.as_f64().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            quote! { #default_value }
        }
        "StringOrNumber" => {
            if let Some(default_value) = default_value.as_f64() {
                quote! { StringOrNumber::Number(#default_value) }
            } else {
                let default_value = default_value.as_str().unwrap_or_else(|| {
                    panic!(
                        "Expected {field_type_str} default value: {:?}",
                        default_value
                    )
                });
                quote! { StringOrNumber::String(String::from(#default_value)) }
            }
        }
        "StringOrBlockElementHeight" => {
            let default_value = default_value.as_str().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            let default_value_ident = format_ident!("{}", default_value.to_case(Case::Pascal));
            quote! { StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::#default_value_ident) }
        }
        _ => {
            let default_value = default_value.as_str().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            let default_value_ident = format_ident!("{}", default_value.to_case(Case::Pascal));
            quote! { #field_type::#default_value_ident }
        }
    }
}
