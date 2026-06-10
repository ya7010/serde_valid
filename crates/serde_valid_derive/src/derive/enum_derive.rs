use super::named_struct_derive::collect_named_fields_validators_list;
use super::unnamed_struct_derive::collect_unnamed_fields_validators_list;
use crate::attribute::variant_validate::collect_variant_custom_from_variant;
use crate::attribute::Validator;
use crate::error::{array_errors_tokens, new_type_errors_tokens, object_errors_tokens};
use crate::serde::rename::collect_serde_rename_map;
use crate::types::CommaSeparatedTokenStreams;
use crate::warning::WithWarnings;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;
pub type Variants = syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;

pub fn expand_enum_validate_derive(
    input: &syn::DeriveInput,
    variants: &Variants,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut errors = vec![];

    let validations = variants
        .into_iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(named_fields) => {
                match expand_enum_variant_named_fields_validation(
                    ident,
                    input,
                    variant,
                    named_fields,
                ) {
                    Ok(variant_varidates_and_rules) => variant_varidates_and_rules,
                    Err(variant_errors) => {
                        errors.extend(variant_errors);
                        WithWarnings::new(Validator::new())
                    }
                }
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                match expand_enum_variant_unnamed_fields_varidation(
                    ident,
                    input,
                    variant,
                    unnamed_fields,
                ) {
                    Ok(variant_varidates_and_rules) => variant_varidates_and_rules,
                    Err(variant_errors) => {
                        errors.extend(variant_errors);
                        WithWarnings::new(Validator::new())
                    }
                }
            }
            syn::Fields::Unit => WithWarnings::new(Validator::new()),
        })
        .collect::<Vec<_>>();

    let validations_and_rules = TokenStream::from_iter(
        validations
            .iter()
            .map(|variant| variant.data.clone())
            .collect::<Vec<_>>(),
    );
    let warnings = validations
        .into_iter()
        .flat_map(|variant| variant.warnings)
        .enumerate()
        .map(|(index, warning)| warning.add_index(index))
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(quote!(
            impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
                    #( #warnings )*
                    #validations_and_rules

                    Ok(())
                }
            }
        ))
    } else {
        Err(errors)
    }
}

fn expand_enum_variant_named_fields_validation(
    ident: &syn::Ident,
    input: &syn::DeriveInput,
    variant: &syn::Variant,
    named_fields: &syn::FieldsNamed,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    let mut warnings = vec![];
    let mut errors = vec![];

    let variant_ident = &variant.ident;
    let mut fields_idents = CommaSeparatedTokenStreams::new();
    let rename_map = collect_serde_rename_map(named_fields)?;

    let enum_validates = match collect_variant_custom_from_variant(&input.attrs) {
        Ok(validations) => {
            warnings.extend(validations.warnings);
            TokenStream::from_iter(validations.data)
        }
        Err(rule_errors) => {
            errors.extend(rule_errors);
            quote!()
        }
    };

    let validates = match collect_named_fields_validators_list(named_fields, &rename_map) {
        Ok(field_validators_list) => {
            TokenStream::from_iter(field_validators_list.iter().map(|validators| {
                let field_ident = validators.ident();

                if let Some(token) = validators.get_tokens() {
                    fields_idents.push(quote!(#field_ident));
                    quote!(#token)
                } else {
                    let field_ident_with_underscore =
                        syn::Ident::new(&format!("_{}", field_ident), field_ident.span());
                    fields_idents.push(quote!(#field_ident: #field_ident_with_underscore));
                    quote!()
                }
            }))
        }
        Err(fields_errors) => {
            errors.extend(fields_errors);
            quote!()
        }
    };

    let variant_errors = object_errors_tokens();

    if errors.is_empty() {
        Ok(WithWarnings {
            data: quote!(
                if let #ident::#variant_ident{#fields_idents} = &self {
                    let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut __property_vec_errors_map = ::serde_valid::validation::PropertyVecErrorsMap::new();

                    #validates
                    #enum_validates

                    if !(__rule_vec_errors.is_empty() && __property_vec_errors_map.is_empty()) {
                        Err(#variant_errors)?
                    }
                }
            ),
            warnings,
        })
    } else {
        Err(errors)
    }
}

fn expand_enum_variant_unnamed_fields_varidation(
    ident: &syn::Ident,
    input: &syn::DeriveInput,
    variant: &syn::Variant,
    unnamed_fields: &syn::FieldsUnnamed,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    let mut warnings = vec![];
    let mut errors = vec![];

    let variant_ident = &variant.ident;
    let mut fields_idents = CommaSeparatedTokenStreams::new();

    let enum_validates = match collect_variant_custom_from_variant(&input.attrs) {
        Ok(validations) => {
            warnings.extend(validations.warnings);
            TokenStream::from_iter(validations.data)
        }
        Err(rule_errors) => {
            errors.extend(rule_errors);
            quote!()
        }
    };

    let validates = match collect_unnamed_fields_validators_list(unnamed_fields) {
        Ok(field_validators_list) => {
            TokenStream::from_iter(field_validators_list.iter().map(|validators| {
                let field_ident = validators.ident();

                if let Some(token) = validators.get_tokens() {
                    fields_idents.push(quote!(#field_ident));
                    quote!(#token)
                } else {
                    let field_ident_with_underscore =
                        syn::Ident::new(&format!("_{}", field_ident), field_ident.span());
                    fields_idents.push(quote!(#field_ident_with_underscore));
                    quote!()
                }
            }))
        }
        Err(fields_errors) => {
            errors.extend(fields_errors);
            quote!()
        }
    };

    let variant_errors = if unnamed_fields.unnamed.len() != 1 {
        array_errors_tokens()
    } else {
        new_type_errors_tokens()
    };

    if errors.is_empty() {
        Ok(WithWarnings {
            data: quote!(
                if let #ident::#variant_ident(#fields_idents) = &self {
                    let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut __item_vec_errors_map = ::serde_valid::validation::ItemVecErrorsMap::new();

                    #enum_validates
                    #validates

                    if !(__rule_vec_errors.is_empty() && __item_vec_errors_map.is_empty()) {
                        Err(#variant_errors)?
                    }
                }
            ),
            warnings,
        })
    } else {
        Err(errors)
    }
}
