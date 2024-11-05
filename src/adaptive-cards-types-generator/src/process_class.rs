use std::collections::HashSet;

use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_str, Ident, Type};

use crate::{
    common_prefix::common_prefix,
    layout_data::{
        create_layoutable_tokens, LAYOUT_DATA_GENERIC_PARAMETER_STR, LAYOUT_DATA_TURBOFISH_STR,
    },
    load::Loaded,
    relationships::Relationships,
    sanitize::{sanitize_field_ident, sanitize_type},
    typed_schema_types::Class,
    GeneratedAdditionalTypes,
};

// const UNIMPLEMENTED_LAYOUTABLE_TYPES: [&str; 15] = [
//     "ActionSet",
//     "ColumnSet",
//     "FactSet",
//     "Image",
//     "ImageSet",
//     "InputChoiceSet",
//     "InputDate",
//     "InputNumber",
//     "InputText",
//     "InputTime",
//     "InputToggle",
//     "Input",
//     "Media",
//     "RichTextBlock",
//     "Table",
// ];

pub(super) struct ProcessClassResult {
    pub class_tokens: TokenStream,
    pub impl_as_trait_macro_tokens: TokenStream,
}

pub(super) fn process_class(
    class: &Loaded<Class>,
    relationships: &Relationships,
    generated_additional_types: &mut GeneratedAdditionalTypes,
) -> ProcessClassResult {
    let class_id = &class.id;
    let class_data = relationships.get_class_data(class_id);

    let ancestors = class_data.ancestors;
    let descendants = class_data.descendants;
    let metadata = class_data.metadata;

    let is_layoutable = relationships.is_layoutable(class_id);

    let (generic_parameter, _turbofish, where_clause) = create_layoutable_tokens(is_layoutable);

    let struct_name_str = &class.type_name;
    let struct_name = format_ident!("{}", struct_name_str);
    let struct_turbofish_str = if is_layoutable {
        LAYOUT_DATA_TURBOFISH_STR
    } else {
        ""
    };

    // Additional code to be generated before the struct.
    let mut pre_struct_tokens = Vec::new();

    // Additional code to be generated after the struct.
    let mut post_struct_tokens = Vec::new();

    // Tokens which will be used as part of the impl_as_trait macro.
    let mut impl_as_trait_macro_tokens = Vec::new();

    // If we're an abstract type, we will generate an enum instead of a struct.
    let struct_tokens = if metadata.is_abstract {
        // If we're an abstract type, we should have descendants, or there is nothing to generate.
        if descendants.is_empty() {
            panic!("Abstract class has no descendants: {}", class_id);
        };

        // Sort the descendants so we always generate the entries in the same order.
        let descendants = descendants
            .iter()
            .sorted_by_key(|d| &d.file_name)
            .collect::<Vec<_>>();

        // Check if there is a common prefix for all variant_names, which we can omit in the generated code.
        let common_prefix = common_prefix(
            &descendants
                .iter()
                .map(|&d| d.type_name.clone())
                .collect::<Vec<_>>(),
        );
        println!("Common prefix: {}", common_prefix);

        struct VariantInfo {
            ident: Ident,
            is_abstract: bool,
        }

        // Collect the variant tokens and metadata about each variant.
        let (variants, variant_infos): (Vec<TokenStream>, Vec<VariantInfo>) =
            descendants
                .iter()
                .map(|&d| {
                    let name = format_ident!("{}", d.type_name);
                    let short_name_str = d
                        .type_name
                        .strip_prefix(&common_prefix)
                        .unwrap_or(&d.type_name);
                    let short_name = format_ident!("{}", short_name_str);
                    let id = &d.id;
                    let descendant_is_layoutable = relationships.is_layoutable(&d.type_name);
                    let (
                        descendant_generic_parameter,
                        _descendant_turbofish,
                        _descendant_where_clause,
                    ) = create_layoutable_tokens(descendant_is_layoutable);

                    (
                        quote! {
                            #[serde(rename = #id)]
                            #short_name(Box<#name #descendant_generic_parameter>),
                        },
                        VariantInfo {
                            ident: short_name,
                            is_abstract: d.value.is_abstract.unwrap_or(false),
                        },
                    )
                })
                .unzip();

        // If this type needs to support layout...
        if is_layoutable {
            // Generate the standard generic as_* methods.
            let mut generate_methods = Vec::new();
            generate_methods.push((
                format_ident!("as_has_layout_data"),
                quote! { &dyn crate::HasLayoutData #generic_parameter },
            ));
            if metadata.is_element {
                generate_methods.push((
                    format_ident!("as_element"),
                    quote! { &dyn crate::LayoutableElement },
                ));
            }

            for (as_name, as_type) in generate_methods {
                // ... we need to generate a method to get the trait for the inner variant type.
                let match_tokens = variant_infos.iter().map(|v| {
                    let variant_ident = &v.ident;

                    if v.is_abstract {
                        // If the inner variant is abstract it is an enum, so call as_layoutable.
                        quote! {
                            #struct_name::#variant_ident(inner) => inner.#as_name(),
                        }
                    } else {
                        // Otherwise it will be a boxed struct, so we can just return it.
                        quote! {
                            #struct_name::#variant_ident(inner) => inner,
                        }
                    }
                });

                // Generate the as_layoutable method.
                post_struct_tokens.push(quote! {
                    impl #generic_parameter #struct_name #generic_parameter
                        #where_clause {
                        pub fn #as_name(&self) -> #as_type {
                            match self {
                                #(#match_tokens)*
                            }
                        }
                    }
                });
            }

            // Build the macro to allow users to generate their own `as_*` implementations.
            let match_tokens = variant_infos.iter().map(|v| {
                let variant_ident = &v.ident;

                if v.is_abstract {
                    // If the inner variant is abstract it is an enum, so call as_layoutable.
                    quote! {
                        adaptive_cards::#struct_name::#variant_ident(inner) => inner.$method_name(),
                    }
                } else {
                    // Otherwise it will be a boxed struct, so we can just return it.
                    quote! {
                        adaptive_cards::#struct_name::#variant_ident(inner) => inner,
                    }
                }
            });

            // Generate the as_layoutable method.
            impl_as_trait_macro_tokens.push(quote! {
                impl $as_trait_name for adaptive_cards::#struct_name<$layout_data> {
                    fn $method_name(&self) -> &dyn $trait_name {
                        match self {
                            #(#match_tokens)*
                        }
                    }
                }
            });
        }

        // Generate the enum.
        quote! {
            #[derive(serde::Deserialize, serde::Serialize, Clone)]
            #[serde(tag = "type")]
            pub enum #struct_name #generic_parameter
                #where_clause {
                #(#variants)*
            }
        }
    } else {
        // We don't support non-abstract types with descendants, but this is fine as it doesn't
        // happen in Adaptive Cards
        if !descendants.is_empty() {
            panic!("Non-abstract class has descendants: {}", class.id);
        }

        let mut fields = Vec::new();

        // Order is important here. More distant ancestors are after closer ones.
        let all_field_sources: Vec<&Loaded<Class>> =
            std::iter::once(class).chain(ancestors).collect();

        // Ancestors can have the same field name, so we need to deduplicate them.
        let mut seen_fields = HashSet::new();

        // Get all unique fields, sorted so we always generate them in the same order.
        let all_fields = all_field_sources
            .iter()
            .flat_map(|c| &c.value.properties)
            .filter(|p| seen_fields.insert(p.0.clone()))
            .sorted_by_key(|p| p.0.clone())
            .collect::<Vec<_>>();

        // For each field...
        for f in all_fields.iter() {
            // Extract whether the field is optional, the generated field name, and the original JSON name.
            let (is_optional, name_str, json_name_str) = match f.0.strip_suffix('?') {
                Some(prefix) => (true, sanitize_field_ident(prefix), prefix),
                None => (false, sanitize_field_ident(f.0), f.0.as_str()),
            };

            // Does the field have a default value if it isn't specified in the Adaptive Cards JSON.
            let default_value = f.1.default.clone();
            let has_default_value = default_value.is_some();

            // The generated field is optional if the original name was suffixed with a "?" (see above) or if the
            // `required` property is false, and if it doesn't have a default value.
            let is_optional = (is_optional || !f.1.required) && !has_default_value;

            // A shorthand means the entire field type could be represented as a string in the Adaptive Cards JSON.
            let has_shorthand = !f.1.shorthands.is_empty();

            let name = format_ident!("{}", name_str);

            // Get the rust type we should use for the field (this will also generate additional types if necessary).
            let sanitized_field_type = sanitize_type(
                &f.1.type_,
                generated_additional_types,
                has_shorthand,
                relationships,
            );

            // If additional types were generated during sanitization, add them to the pre-struct tokens.
            if let Some(additional_type) = sanitized_field_type.additional_type {
                pre_struct_tokens.push(additional_type.token_stream.clone());
            }

            let field_type_str = sanitized_field_type.type_name;

            let is_vec = field_type_str.starts_with("Vec<");

            let field_type_str = if !is_vec {
                let field_is_layoutable = relationships.is_layoutable(&field_type_str)
                    || generated_additional_types.is_layoutable(&field_type_str);

                if field_is_layoutable {
                    format!("{}{}", field_type_str, LAYOUT_DATA_GENERIC_PARAMETER_STR)
                } else {
                    field_type_str.to_string()
                }
            } else {
                let inner_type = field_type_str
                    .strip_prefix("Vec<")
                    .unwrap()
                    .strip_suffix('>')
                    .unwrap();

                let field_is_layoutable = relationships.is_layoutable(inner_type)
                    || generated_additional_types.is_layoutable(inner_type);

                if field_is_layoutable {
                    format!("Vec<{}{}>", inner_type, LAYOUT_DATA_GENERIC_PARAMETER_STR)
                } else {
                    field_type_str.to_string()
                }
            };

            let field_type: Type = parse_str(&field_type_str)
                .unwrap_or_else(|_| panic!("Failed to parse type: {}", field_type_str));

            if is_optional {
                // If the field is optional, generate an Option<> field.
                fields.push(quote! {
                    #[serde(rename = #json_name_str, skip_serializing_if = "Option::is_none")]
                    pub #name: Option<#field_type>,
                });
            } else if let Some(default_value) = default_value {
                // If the field has a default value, generate a function to get the default value.
                let default_value_function = format!("default_value_for_{}", name);
                let default_value_function_qualified = format!(
                    "{}{}::{}",
                    struct_name, struct_turbofish_str, default_value_function
                );
                let default_value_function_ident = format_ident!("{}", default_value_function);
                let is_default_value_function_str = format!("is_{}", default_value_function);
                let is_default_value_function_fully_qualified_str = format!(
                    "{}{}::{}",
                    struct_name_str, struct_turbofish_str, is_default_value_function_str
                );
                let is_default_value_function_ident =
                    format_ident!("{}", is_default_value_function_str);
                let (default_value_impl, is_default_value_impl) =
                    get_default_value_impl(field_type_str, default_value, &field_type);

                let default_value_function_tokens = quote! {
                    impl #generic_parameter #struct_name #generic_parameter
                        #where_clause {
                        fn #default_value_function_ident() -> #field_type {
                            #default_value_impl
                        }

                        fn #is_default_value_function_ident(value: &#field_type) -> bool {
                            #is_default_value_impl
                        }
                    }
                };

                post_struct_tokens.push(default_value_function_tokens);

                // And generate the field specifying the generated function to use to get the default value.
                fields.push(quote! {
                    #[serde(rename = #json_name_str, default = #default_value_function_qualified, skip_serializing_if = #is_default_value_function_fully_qualified_str)]
                    pub #name: #field_type,
                });
            } else {
                // Otherwise generate a regular field.
                fields.push(quote! {
                    #[serde(rename = #json_name_str)]
                    pub #name: #field_type,
                });
            }
        }

        // Sometimes the JSON unnecessarily includes a "type" field, so add one to every struct.
        if !seen_fields.contains("type") {
            fields.push(quote! {
                #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
                pub type_: Option<String>,
            });
        }

        // If this type needs to support layout...
        if is_layoutable {
            // Generate a layout_data field to store the layout metadata.
            fields.push(quote! {
                #[serde(rename = ".layout", skip_deserializing)]
                pub layout_data: core::cell::RefCell #generic_parameter,
            });

            // Implement the HasLayoutData trait for the struct.
            post_struct_tokens.push(quote! {
                impl #generic_parameter crate::HasLayoutData #generic_parameter for #struct_name #generic_parameter
                    #where_clause {
                    fn layout_data(&self) -> &core::cell::RefCell #generic_parameter {
                        &self.layout_data
                    }
                }
            });

            // And if we haven't yet implemented the Layoutable trait for this type, generate an empty impl.
            // let is_unimplemented_layoutable =
            //     UNIMPLEMENTED_LAYOUTABLE_TYPES.contains(&class.type_name.as_str());
            // if is_unimplemented_layoutable {
            //     post_struct_tokens.push(quote! {
            //         impl crate::layoutable::Layoutable for #struct_name {
            //         }
            //     });
            // }

            // If we're layoutable and not an adaptive card, implement the
            // LayoutableElement trait for this type.
            if metadata.is_element {
                // If we're an image, we need to handle the height field differently
                // as it is overridden and a different type to other elements.
                let height_impl = if metadata.is_image {
                    quote! { self.height.clone() }
                } else {
                    quote! {
                        self.height
                            .map(StringOrBlockElementHeight::BlockElementHeight)
                            .unwrap_or(StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto))
                    }
                };

                post_struct_tokens.push(quote! {
                    impl #generic_parameter crate::LayoutableElement for #struct_name #generic_parameter
                        #where_clause {
                        fn get_height(&self) -> StringOrBlockElementHeight {
                            #height_impl
                        }
                        fn get_separator(&self) -> bool {
                            self.separator.unwrap_or(false)
                        }
                        fn get_spacing(&self) -> Spacing {
                            self.spacing.unwrap_or(Spacing::Default)
                        }
                        fn get_is_visible(&self) -> bool {
                            self.is_visible
                        }
                    }

                })
            }
        }

        // Generate the struct.
        quote! {
            #[derive(serde::Deserialize, serde::Serialize, Clone)]
            #[serde(deny_unknown_fields)]
            pub struct #struct_name #generic_parameter
                #where_clause {
                #(#fields)*
            }
        }
    };

    // The final tokens are the concatenation of the pre-struct tokens, the struct,
    // and the post-struct tokens.
    let result = quote! {
        #(#pre_struct_tokens)*
        #struct_tokens
        #(#post_struct_tokens)*
    };

    println!("{}", result);

    ProcessClassResult {
        class_tokens: result,
        impl_as_trait_macro_tokens: quote! { #(#impl_as_trait_macro_tokens)* },
    }
}

/// Generate the default value for a field.
fn get_default_value_impl(
    field_type_str: String,
    default_value: serde_json::Value,
    field_type: &Type,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match field_type_str.as_str() {
        "String" => {
            let default_value = default_value
                .as_str()
                .unwrap_or_else(|| panic!("Expected string default value: {:?}", default_value));
            (
                quote! { String::from(#default_value) },
                quote! { *value == #default_value },
            )
        }
        "bool" => {
            let default_value = default_value.as_bool().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            (
                quote! { #default_value },
                quote! { *value == #default_value },
            )
        }
        "u32" | "u64" => {
            let default_value = default_value.as_u64().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            (
                quote! { #default_value },
                quote! { *value == #default_value },
            )
        }
        "f32" | "f64" => {
            let default_value = default_value.as_f64().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            (
                quote! { #default_value },
                quote! { *value == #default_value },
            )
        }
        "StringOrNumber" => {
            if let Some(default_value) = default_value.as_f64() {
                (
                    quote! { StringOrNumber::Number(#default_value) },
                    quote! { matches!(*value, StringOrNumber::Number(#default_value)) },
                )
            } else {
                let default_value = default_value.as_str().unwrap_or_else(|| {
                    panic!(
                        "Expected {field_type_str} default value: {:?}",
                        default_value
                    )
                });
                (
                    quote! { StringOrNumber::String(String::from(#default_value)) },
                    quote! { matches!(*value, StringOrNumber::String(String::from(#default_value))) },
                )
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
            (
                quote! { StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::#default_value_ident) },
                quote! { matches!(*value, StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::#default_value_ident)) },
            )
        }
        _ => {
            let default_value = default_value.as_str().unwrap_or_else(|| {
                panic!(
                    "Expected {field_type_str} default value: {:?}",
                    default_value
                )
            });
            let default_value_ident = format_ident!("{}", default_value.to_case(Case::Pascal));
            (
                quote! { #field_type::#default_value_ident },
                quote! { matches!(*value, #field_type::#default_value_ident) },
            )
        }
    }
}
