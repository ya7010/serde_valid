use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use quote::quote;

type Lits<'a> = syn::punctuated::Punctuated<syn::Lit, syn::token::Comma>;

pub fn extract_generic_enum_validator_from_name_value(
    field: &impl Field,
    name_value: &syn::MetaNameValue,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let lits = get_enum_from_name_value(name_value)?;
    inner_extract_generic_enum_validator(field, &lits, message_format, rename_map)
}

fn inner_extract_generic_enum_validator(
    field: &impl Field,
    lits: &Lits,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();

    Ok(quote!(
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedEnum::validate_composited_enum(
            #field_ident,
            &[#lits],
        ) {
            use ::serde_valid::validation::IntoError;

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#message_format));
        }
    ))
}

fn get_enum_from_name_value(name_value: &syn::MetaNameValue) -> Result<Lits<'_>, crate::Errors> {
    if let syn::Expr::Array(array) = &name_value.value {
        let mut items = Lits::new();
        for item in &array.elems {
            match item {
                syn::Expr::Lit(lit) => items.push(lit.lit.clone()),
                _ => return Err(vec![crate::Error::literal_only(item)]),
            }
        }
        Ok(items)
    } else {
        Err(vec![crate::Error::validate_enum_need_array(
            &name_value.value,
        )])
    }
}
