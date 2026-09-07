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
    let field_ident_name = field_ident.to_string();
    let field_ident_name = field_ident_name
        .strip_prefix("r#")
        .unwrap_or(&field_ident_name);
    let pattern_ident = syn::Ident::new(
        &format!("{field_ident_name}_PATTERN").to_uppercase(),
        field_ident.span(),
    );
    let pattern_variable = quote!(
        #pattern_ident.get_or_init(|| ::serde_valid::export::regex::Regex::new(#pattern).unwrap())
    );
    let validate = quote_composited_validation!(
        field_ident,
        pattern_variable,
        ValidateCompositedPattern,
        validate_composited_pattern
    );
    let error_params = crate::types::generated_ident("__serde_valid_composited_error_params");

    Ok(quote!(
        {
            static #pattern_ident : ::serde_valid::export::once_cell::sync::OnceCell<::serde_valid::export::regex::Regex> = ::serde_valid::export::once_cell::sync::OnceCell::new();
            if let ::std::result::Result::Err(#error_params) = #validate {
                use ::serde_valid::validation::IntoError;

                #errors
                    .entry(#rename)
                    .or_default()
                    .push(#error_params.into_error_by(#message_format));
            }
        }
    ))
}
