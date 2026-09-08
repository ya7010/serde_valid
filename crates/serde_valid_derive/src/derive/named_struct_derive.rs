use crate::attribute::field_validate::{extract_field_validator, FieldValidators};
use crate::attribute::struct_validate::collect_struct_custom_from_named_struct;
use crate::error::object_errors_tokens;
use crate::serde::rename::{collect_serde_rename_map, RenameMap};
use crate::types::{Field, NamedField};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::iter::FromIterator;

pub fn expand_named_struct_derive(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let rename_map = collect_serde_rename_map(fields)?;

    let mut errors = vec![];

    let struct_validations = match collect_struct_custom_from_named_struct(&input.attrs) {
        Ok(validations) => TokenStream::from_iter(validations),
        Err(rule_errors) => {
            errors.extend(rule_errors);
            quote!()
        }
    };

    let field_validates = match collect_named_fields_validators_list(fields, &rename_map) {
        Ok(field_validators) => TokenStream::from_iter(field_validators.iter().map(|validator| {
            if validator.is_empty() {
                quote!()
            } else {
                validator.generate_tokens()
            }
        })),
        Err(validation_errors) => {
            errors.extend(validation_errors);
            quote!()
        }
    };

    let fields_errors = object_errors_tokens();
    let rule_vec_errors = crate::types::rule_vec_errors_ident();
    let property_vec_errors_map = crate::types::property_vec_errors_map_ident();

    if errors.is_empty() {
        Ok(quote!(
            impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                fn validate(&self) -> ::std::result::Result<(), ::serde_valid::validation::Errors> {
                    let mut #rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut #property_vec_errors_map = ::serde_valid::validation::PropertyVecErrorsMap::new();

                    #field_validates
                    #struct_validations

                    if #rule_vec_errors.is_empty() && #property_vec_errors_map.is_empty() {
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Err(#fields_errors)
                    }
                }
            }
        ))
    } else {
        Err(errors)
    }
}

pub fn collect_named_fields_validators_list<'a>(
    fields: &'a syn::FieldsNamed,
    rename_map: &RenameMap,
) -> Result<Vec<FieldValidators<'a, NamedField<'a>>>, crate::Errors> {
    let mut errors = vec![];

    let validators = fields
        .named
        .iter()
        .filter_map(
            |field| match collect_named_field_validators(field, rename_map) {
                Ok(validators) => Some(validators),
                Err(ref mut error) => {
                    errors.append(error);
                    None
                }
            },
        )
        .collect();

    if errors.is_empty() {
        Ok(validators)
    } else {
        Err(errors)
    }
}

fn collect_named_field_validators<'a>(
    field: &'a syn::Field,
    rename_map: &RenameMap,
) -> Result<FieldValidators<'a, NamedField<'a>>, crate::Errors> {
    let mut errors = vec![];

    let named_field = NamedField::new(field).map_err(|error| vec![error])?;
    let validators = named_field
        .attrs()
        .iter()
        .filter_map(|attribute| {
            if attribute.path().is_ident("validate") {
                match extract_field_validator(&named_field, attribute, rename_map) {
                    Ok(validator) => Some(validator),
                    Err(validator_error) => {
                        errors.extend(validator_error);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(FieldValidators::new(
        Cow::Owned(named_field.clone()),
        validators,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deprecated_enumerate_attribute_is_rejected() {
        let input: syn::DeriveInput = syn::parse_quote! {
            struct Input {
                #[validate(enumerate = ["a"])]
                value: String,
            }
        };
        let syn::Data::Struct(data) = &input.data else {
            unreachable!();
        };
        let syn::Fields::Named(fields) = &data.fields else {
            unreachable!();
        };

        let errors = expand_named_struct_derive(&input, fields).unwrap_err();
        let message = crate::error::to_compile_errors(errors).to_string();

        assert!(message.contains("`enumerate` is unknown validation type"));
        assert!(message.contains("r#enum"));
    }
}
