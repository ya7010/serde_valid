use crate::attribute::common::lit::get_numeric;
use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_numeric_multiple_of_validator(field, validation_value, message_format, rename_map)
}

fn inner_extract_numeric_multiple_of_validator(
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
    let multiple_of = get_numeric(validation_value)?;

    Ok(quote!(
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedMultipleOf::validate_composited_multiple_of(
            #field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::validation::IntoError;

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#message_format));
        }
    ))
}
