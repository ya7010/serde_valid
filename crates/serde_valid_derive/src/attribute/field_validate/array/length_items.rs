use crate::attribute::common::lit::get_numeric;
use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! extract_array_length_validator{
    ($ErrorType:ident) => {
        pastey::paste! {
            pub fn [<extract_array_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                message_format: MessageFormat,
                rename_map: &RenameMap,
            ) -> Result<Validator, crate::Errors> {
                [<inner_extract_array_ $ErrorType:snake _validator>](field, validation_value, message_format, rename_map)
            }

            fn [<inner_extract_array_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                message_format: MessageFormat,
                rename_map: &RenameMap,
            ) -> Result<TokenStream, crate::Errors> {
                let field_name = field.name();
                let field_ident = field.ident();
                let field_key = field.key();
                let rename = rename_map.get(field_name).unwrap_or(&field_key);
                let [<$ErrorType:snake>] = get_numeric(validation_value)?;
                let errors = field.errors_variable();

                Ok(quote!(
                    if let Err(error_params) = ::serde_valid::[<Validate $ErrorType>]::[<validate_ $ErrorType:snake>](
                        #field_ident,
                        #[<$ErrorType:snake>],
                    ) {
                        use ::serde_valid::validation::error::FormatDefault;

                        #errors
                            .entry(#rename)
                            .or_default()
                            .push(::serde_valid::validation::Error::$ErrorType(
                                ::serde_valid::validation::error::Message::new(
                                    error_params,
                                    #message_format,
                                )
                            ));
                    }
                ))
            }
        }
    }
}

extract_array_length_validator!(MaxItems);
extract_array_length_validator!(MinItems);
