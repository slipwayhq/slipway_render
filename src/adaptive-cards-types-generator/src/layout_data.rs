use proc_macro2::TokenStream;
use quote::quote;

pub(super) const LAYOUT_DATA_GENERIC_PARAMETER_STR: &str = "<TLayoutData>";
pub(super) const LAYOUT_DATA_TURBOFISH_STR: &str = "::<TLayoutData>";

pub(super) fn create_generic_parameter(is_layoutable: bool) -> TokenStream {
    if is_layoutable {
        quote! { <TLayoutData> }
    } else {
        quote! {}
    }
}

pub(super) fn create_turbofish(is_layoutable: bool) -> TokenStream {
    if is_layoutable {
        quote! { ::<TLayoutData> }
    } else {
        quote! {}
    }
}

pub(super) fn create_where_clause(is_layoutable: bool) -> TokenStream {
    if is_layoutable {
        quote! { where TLayoutData: Default }
    } else {
        quote! {}
    }
}

pub(super) fn create_layoutable_tokens(
    is_layoutable: bool,
) -> (TokenStream, TokenStream, TokenStream) {
    (
        create_generic_parameter(is_layoutable),
        create_turbofish(is_layoutable),
        create_where_clause(is_layoutable),
    )
}
