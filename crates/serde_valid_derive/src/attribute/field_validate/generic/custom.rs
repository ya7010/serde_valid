use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::{CommaSeparatedNestedMetas, Field, SingleIdentPath};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_generic_custom_validator_from_meta_list(
    field: &impl Field,
    meta_list: &syn::MetaList,
    _message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let path = &meta_list.path;
    let path_ident = SingleIdentPath::new(path)
        .map_err(|error| vec![error])?
        .ident();
    let field_name = field.name();
    let field_key = field.key();
    let nested = meta_list
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::custom_message_parse_error(path_ident, &error)])?;

    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let custom_fn_name = match nested.len() {
        0 => Err(vec![
            crate::Error::validate_custom_meta_list_need_function_or_closure(path),
        ]),
        1 => extract_custom_fn_name(&nested[0]),
        _ => Err(nested
            .iter()
            .skip(1)
            .map(crate::Error::validate_custom_tail_error)
            .collect()),
    }?;
    inner_extract_generic_custom_validator(
        field_ident,
        rename,
        &custom_fn_name,
        &field.errors_variable(),
    )
}

pub fn extract_generic_custom_validator_from_meta_name_value(
    field: &impl Field,
    meta_name_value: &syn::MetaNameValue,
    _message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let field_name = field.name();
    let field_key = field.key();

    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let fn_name = match &meta_name_value.value {
        syn::Expr::Path(path) => Ok(quote!(#path)),
        syn::Expr::Call(func_call) => Ok(quote!(#func_call)),
        syn::Expr::Closure(closure) => Ok(quote!((#closure))),
        _ => Err(vec![
            crate::Error::validate_custom_meta_name_value_need_function_or_closure(meta_name_value),
        ]),
    }?;

    inner_extract_generic_custom_validator(field_ident, rename, &fn_name, &field.errors_variable())
}

fn inner_extract_generic_custom_validator(
    field_ident: &syn::Ident,
    rename: &TokenStream,
    custom_fn_name: &TokenStream,
    errors: &TokenStream,
) -> Result<Validator, crate::Errors> {
    Ok(quote!(
        if let Err(__errors) = serde_valid::validation::custom::wrap_into_vec_errors(#custom_fn_name(#field_ident)) {
            #errors
                .entry(#rename)
                .or_default()
                .extend(__errors);
        };
    ))
}

fn extract_custom_fn_name(
    nested_meta: &crate::types::NestedMeta,
) -> Result<TokenStream, crate::Errors> {
    match nested_meta {
        crate::types::NestedMeta::Meta(syn::Meta::Path(fn_name)) => Ok(quote!(#fn_name)),
        crate::types::NestedMeta::Meta(syn::Meta::List(closure)) => Ok(quote!(#closure)),
        crate::types::NestedMeta::Closure(closure) => Ok(quote!((#closure))),
        _ => Err(vec![
            crate::Error::validate_custom_meta_list_need_function_or_closure(nested_meta),
        ]),
    }
}
