use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    load::Loaded,
    typed_schema_types::{Enum, EnumValue},
};

const DEFAULT_VARIANT_NAMES: [&str; 2] = ["Default", "Auto"];

pub(super) fn process_enum(loaded_enum: &Loaded<Enum>) -> proc_macro2::TokenStream {
    let type_name = &loaded_enum.type_name;
    let enum_ = &loaded_enum.value;

    let mut default_variant: Option<Ident> = None;
    let mut variants: Vec<TokenStream> = Vec::new();

    let enum_name = format_ident!("{}", type_name);
    for e in enum_.values.iter() {
        let name_str = match e {
            EnumValue::Variant0(s) => s,
            EnumValue::Variant1 {
                description: _,
                value,
                version: _,
            } => value,
        };

        // We're going to tell serde to serialize/deserialize using name_str.
        // If the name_str is already pascal case, generate a camel case alias.
        // Otherwise generate a pascal case alias.
        let name_pascal_case_str = name_str.to_case(Case::Pascal);
        let alias = if &name_pascal_case_str == name_str {
            name_str.to_case(Case::Camel)
        } else {
            name_pascal_case_str.clone()
        };

        let name = format_ident!("{}", name_pascal_case_str);

        if default_variant.is_none()
            && DEFAULT_VARIANT_NAMES.contains(&name_pascal_case_str.as_str())
        {
            default_variant = Some(name.clone());
        }

        variants.push(quote! {
            #[serde(rename = #name_str, alias = #alias)]
            #name,
        });
    }

    let default_variant_impl = match default_variant {
        None => quote! {},
        Some(name) => {
            quote! {
                impl Default for #enum_name {
                    fn default() -> Self {
                        #enum_name::#name
                    }
                }
            }
        }
    };

    let result = quote! {
        #[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
        pub enum #enum_name {
            #(#variants)*
        }

        #default_variant_impl
    };

    println!("{}", result);

    result
}
