use std::collections::HashSet;

use quote::{format_ident, quote};
use syn::{parse_str, Type};

use crate::{
    common_prefix::common_prefix,
    load::Loaded,
    relationships::Relationships,
    sanitize::{sanitize_field_ident, sanitize_type},
    typed_schema_types::Class,
};

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

    let item_name = format_ident!("{}", class.type_name);

    let mut additional_types = Vec::new();

    let is_abstract = class.value.is_abstract.unwrap_or(false);

    let item_tokens = if is_abstract {
        if descendants.is_empty() {
            panic!("Abstract class has no descendants: {}", class.id);
        };

        let mut descendants = descendants.iter().collect::<Vec<_>>();
        descendants.sort_by_key(|d| &d.file_name);

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

            let is_optional = is_optional || !p.1.required;
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
                additional_types.push(additional_type);
            }

            let field_type_str = sanitized_field_type.type_name;
            let field_type: Type = parse_str(&field_type_str)
                .unwrap_or_else(|_| panic!("Failed to parse type: {}", field_type_str));

            if is_optional {
                fields.push(quote! {
                    #[serde(rename = #json_name_str)]
                    pub #name: Option<#field_type>,
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
