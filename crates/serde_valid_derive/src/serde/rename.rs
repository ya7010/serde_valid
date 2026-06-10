use std::collections::HashMap;

use crate::types::{CommaSeparatedMetas, Field, NamedField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub type RenameMap = HashMap<String, TokenStream>;

pub fn collect_serde_rename_map(fields: &syn::FieldsNamed) -> Result<RenameMap, crate::Errors> {
    let mut renames = RenameMap::new();
    for field in fields.named.iter() {
        let named_field = NamedField::new(field).map_err(|error| vec![error])?;
        for attribute in named_field.attrs() {
            if attribute.path().is_ident("serde") {
                if let Some(rename) = find_rename_from_serde_attributes(attribute) {
                    renames.insert(
                        field.ident.to_token_stream().to_string(),
                        quote!(std::borrow::Cow::from(#rename)),
                    );
                }
            }
        }
    }
    Ok(renames)
}

fn find_rename_from_serde_attributes(attribute: &syn::Attribute) -> Option<TokenStream> {
    if let syn::Meta::List(serde_list) = &attribute.meta {
        if let Ok(serde_nested_meta) =
            serde_list.parse_args_with(CommaSeparatedMetas::parse_terminated)
        {
            for serde_meta in serde_nested_meta {
                if let Some(rename) = find_rename_from_serde_rename_attributes(&serde_meta) {
                    return Some(rename);
                }
            }
        }
    }
    None
}

fn find_rename_from_serde_rename_attributes(serde_meta: &syn::Meta) -> Option<TokenStream> {
    match serde_meta {
        syn::Meta::NameValue(rename_name_value) => {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &rename_name_value.value
            {
                Some(lit_str.to_token_stream())
            } else {
                None
            }
        }
        syn::Meta::List(rename_list) => {
            if let Ok(nested) = rename_list.parse_args_with(CommaSeparatedMetas::parse_terminated) {
                for rename_meta in nested {
                    if !rename_meta.path().is_ident("deserialize") {
                        continue;
                    }
                    if let syn::Meta::NameValue(deserialize_name_value) = rename_meta {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = &deserialize_name_value.value
                        {
                            return Some(lit_str.to_token_stream());
                        }
                    }
                }
            }

            None
        }
        _ => None,
    }
}
