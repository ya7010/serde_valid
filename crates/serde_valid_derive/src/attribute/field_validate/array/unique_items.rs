use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_array_unique_items_validator(
    field: &impl Field,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Validator {
    inner_extract_array_unique_items_validator(field, message_format, rename_map)
}

fn inner_extract_array_unique_items_validator(
    field: &impl Field,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> TokenStream {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();

    quote!(
        if let Err(error_params) = ::serde_valid::ValidateUniqueItems::validate_unique_items(
            #field_ident
        ) {
            #errors
                .entry(#rename)
                .or_default()
                .push(::serde_valid::validation::Error::UniqueItems(
                    ::serde_valid::validation::error::Message::new(
                        error_params,
                        #message_format,
                    )
                ));
        }
    )
}
