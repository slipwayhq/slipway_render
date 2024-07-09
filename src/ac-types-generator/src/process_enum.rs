use convert_case::{Case, Casing};
use quote::{format_ident, quote};

use crate::typed_schema_types::{Enum, EnumValue};

pub(super) fn process_enum(type_name: String, input: Enum) -> proc_macro2::TokenStream {
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
