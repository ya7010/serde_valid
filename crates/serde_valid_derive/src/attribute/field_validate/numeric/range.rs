use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
macro_rules! extract_numeric_range_validator{
    (
        $extract_validator:ident,
        $inner_extract_validator:ident,
        $ValidateCompositedTrait:ident,
        $validate_composited_method:ident
    ) => {
        pub fn $extract_validator(
            field: &impl Field,
            validation_value: &syn::Expr,
            message_format: MessageFormat,
            rename_map: &RenameMap,
        ) -> Result<Validator, crate::Errors> {
            $inner_extract_validator(field, validation_value, message_format, rename_map)
        }

        fn $inner_extract_validator(
            field: &impl Field,
            validation_value: &syn::Expr,
            message_format: MessageFormat,
            rename_map: &RenameMap,
        ) -> Result<TokenStream, crate::Errors> {
            let field_name = field.name();
            let field_ident = field.ident();
            let field_key = field.key();
            let rename = rename_map.get(field_name).unwrap_or(&field_key);
            let errors = field.errors_variable();
            let validate = quote_composited_validation!(
                field_ident, validation_value,
                $ValidateCompositedTrait, $validate_composited_method
            );
            let error_params =
                crate::types::generated_ident("__serde_valid_composited_error_params");

            Ok(quote!(
                if let ::std::result::Result::Err(#error_params) = #validate {
                    use ::serde_valid::validation::IntoError;

                    #errors
                        .entry(#rename)
                        .or_default()
                        .push(#error_params.into_error_by(#message_format));
                }
            ))
        }
    }
}

extract_numeric_range_validator!(
    extract_numeric_maximum_validator,
    inner_extract_numeric_maximum_validator,
    ValidateCompositedMaximum,
    validate_composited_maximum
);
extract_numeric_range_validator!(
    extract_numeric_minimum_validator,
    inner_extract_numeric_minimum_validator,
    ValidateCompositedMinimum,
    validate_composited_minimum
);
extract_numeric_range_validator!(
    extract_numeric_exclusive_maximum_validator,
    inner_extract_numeric_exclusive_maximum_validator,
    ValidateCompositedExclusiveMaximum,
    validate_composited_exclusive_maximum
);
extract_numeric_range_validator!(
    extract_numeric_exclusive_minimum_validator,
    inner_extract_numeric_exclusive_minimum_validator,
    ValidateCompositedExclusiveMinimum,
    validate_composited_exclusive_minimum
);
