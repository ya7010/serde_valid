use crate::attribute::common::lit::get_numeric;
use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
macro_rules! extract_object_size_validator {
    (
        $extract_validator:ident,
        $inner_extract_validator:ident,
        $ValidateCompositedTrait:ident,
        $validate_composited_method:ident
    ) => {
        pub fn $extract_validator(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_format: MessageFormat,
            rename_map: &RenameMap,
        ) -> Result<Validator, crate::Errors> {
            $inner_extract_validator(field, validation_value, message_format, rename_map)
        }

        fn $inner_extract_validator(
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
            let limit = get_numeric(validation_value)?;

            Ok(quote!(
                if let Err(__composited_error_params) = ::serde_valid::validation::$ValidateCompositedTrait::$validate_composited_method(
                    #field_ident,
                    #limit
                ) {
                    use ::serde_valid::validation::IntoError;

                    #errors
                        .entry(#rename)
                        .or_default()
                        .push(__composited_error_params.into_error_by(#message_format));
                }
            ))
        }
    }
}

extract_object_size_validator!(
    extract_object_max_properties_validator,
    inner_extract_object_max_properties_validator,
    ValidateCompositedMaxProperties,
    validate_composited_max_properties
);
extract_object_size_validator!(
    extract_object_min_properties_validator,
    inner_extract_object_min_properties_validator,
    ValidateCompositedMinProperties,
    validate_composited_min_properties
);
