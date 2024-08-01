use convert_case::{Case, Casing};
use quote::{format_ident, quote};

use crate::{
    load::Loaded,
    typed_schema_types::{Enum, EnumValue},
};

pub(super) fn process_enum(loaded_enum: &Loaded<Enum>) -> proc_macro2::TokenStream {
    let type_name = &loaded_enum.type_name;
    let enum_ = &loaded_enum.value;

    let enum_name = format_ident!("{}", type_name);
    let variants = enum_.values.iter().map(|e| {
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

        quote! {
            #[serde(rename = #name_str, alias = #alias)]
            #name,
        }
    });

    let result = quote! {
        #[derive(serde::Deserialize, Copy, Clone)]
        pub enum #enum_name {
            #(#variants)*
        }
    };

    println!("{}", result);

    result
}
