mod field;
mod nested_meta;
mod single_ident_path;

pub use field::{Field, NamedField, UnnamedField};
pub use nested_meta::NestedMeta;
use proc_macro2::TokenStream;
pub use single_ident_path::SingleIdentPath;

pub fn generated_ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, proc_macro2::Span::mixed_site())
}

pub fn rule_vec_errors_ident() -> syn::Ident {
    generated_ident("__rule_vec_errors")
}

pub fn property_vec_errors_map_ident() -> syn::Ident {
    generated_ident("__property_vec_errors_map")
}

pub fn item_vec_errors_map_ident() -> syn::Ident {
    generated_ident("__item_vec_errors_map")
}

pub type CommaSeparatedTokenStreams = syn::punctuated::Punctuated<TokenStream, syn::token::Comma>;
pub type CommaSeparatedNestedMetas = syn::punctuated::Punctuated<NestedMeta, syn::token::Comma>;
pub type CommaSeparatedMetas = syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>;
