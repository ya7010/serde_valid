use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use quote::quote;

pub fn extract_generic_validate_validator(
    field: &impl Field,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let field_ident = field.ident();
    let field_name = field.name();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let validate = quote_validation!(
        zero field_ident,
        Validate, validate
    );
    let inner_errors = crate::types::generated_ident("__serde_valid_inner_errors");
    let object_errors = crate::types::generated_ident("__serde_valid_object_errors");
    let array_errors = crate::types::generated_ident("__serde_valid_array_errors");
    let new_type_errors = crate::types::generated_ident("__serde_valid_new_type_errors");

    Ok(quote!(
        if let ::std::result::Result::Err(#inner_errors) = #validate {
            match #inner_errors {
                ::serde_valid::validation::Errors::Object(#object_errors) => {
                    #errors.entry(#rename).or_default().push(
                        ::serde_valid::validation::Error::Properties(#object_errors)
                    );
                }
                ::serde_valid::validation::Errors::Array(#array_errors) => {
                    #errors.entry(#rename).or_default().push(
                        ::serde_valid::validation::Error::Items(#array_errors)
                    );
                }
                ::serde_valid::validation::Errors::NewType(#new_type_errors) => {
                    #errors.entry(#rename).or_default().extend(#new_type_errors);
                }
            }
        }
    ))
}
