use crate::attribute::field_validate::{extract_field_validator, FieldValidators};
use crate::attribute::struct_validate::collect_struct_custom_from_named_struct;
use crate::attribute::Validator;
use crate::error::{array_errors_tokens, new_type_errors_tokens};
use crate::types::{Field, UnnamedField};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn expand_unnamed_struct_derive(
    input: &syn::DeriveInput,
    fields: &syn::FieldsUnnamed,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut warnings = vec![];
    let mut errors = vec![];

    let struct_validations = match collect_struct_custom_from_named_struct(&input.attrs) {
        Ok(validations) => {
            warnings.extend(validations.warnings);
            Validator::from_iter(validations.data)
        }
        Err(rule_errors) => {
            errors.extend(rule_errors);
            quote!()
        }
    };

    let field_validates: TokenStream = match collect_unnamed_fields_validators_list(fields) {
        Ok(field_validators) => TokenStream::from_iter(field_validators.iter().map(|validator| {
            warnings.extend(validator.warnings.clone());
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

    let fields_errors = if fields.unnamed.len() != 1 {
        array_errors_tokens()
    } else {
        new_type_errors_tokens()
    };

    let warnings = warnings
        .into_iter()
        .enumerate()
        .map(|(index, warning)| warning.add_index(index))
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(quote!(
            #(#warnings)*
            impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
                    let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut __item_vec_errors_map = ::serde_valid::validation::ItemVecErrorsMap::new();

                    #field_validates
                    #struct_validations

                    if __rule_vec_errors.is_empty() && __item_vec_errors_map.is_empty() {
                        Ok(())
                    } else {
                        Err(#fields_errors)
                    }
                }
            }
        ))
    } else {
        Err(errors)
    }
}

pub fn collect_unnamed_fields_validators_list(
    fields: &syn::FieldsUnnamed,
) -> Result<Vec<FieldValidators<'_, UnnamedField<'_>>>, crate::Errors> {
    let mut errors = vec![];

    let validators = fields
        .unnamed
        .iter()
        .enumerate()
        .filter_map(|field| match collect_unnamed_field_validators(field) {
            Ok(validators) => Some(validators),
            Err(ref mut error) => {
                errors.append(error);
                None
            }
        })
        .collect();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(validators)
}

fn collect_unnamed_field_validators(
    (index, field): (usize, &syn::Field),
) -> Result<FieldValidators<'_, UnnamedField<'_>>, crate::Errors> {
    let mut errors = vec![];

    let unnamed_field = UnnamedField::new(index, field);

    let validators = unnamed_field
        .attrs()
        .iter()
        .filter_map(|attribute| {
            if attribute.path().is_ident("validate") {
                match extract_field_validator(&unnamed_field, attribute, &HashMap::new()) {
                    Ok(validator) => Some(validator),
                    Err(validator_errors) => {
                        errors.extend(validator_errors);
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

    Ok(FieldValidators::new(Cow::Owned(unnamed_field), validators))
}
