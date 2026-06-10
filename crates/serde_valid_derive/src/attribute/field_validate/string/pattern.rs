use crate::attribute::common::lit::get_str;
use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_string_pattern_validator(field, validation_value, message_format, rename_map)
}

fn inner_extract_string_pattern_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let pattern = get_str(validation_value)?;
    let pattern_ident = syn::Ident::new(
        &format!("{}_PATTERN", &field_ident).to_uppercase(),
        field_ident.span(),
    );

    Ok(quote!(
        static #pattern_ident : ::serde_valid::export::once_cell::sync::OnceCell<::serde_valid::export::regex::Regex> = ::serde_valid::export::once_cell::sync::OnceCell::new();
        let __pattern = #pattern_ident.get_or_init(|| ::serde_valid::export::regex::Regex::new(#pattern).unwrap());
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedPattern::validate_composited_pattern(
            #field_ident,
            __pattern,
        ) {
            use ::serde_valid::validation::IntoError;

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#message_format));
        }
    ))
}
