mod meta_list;
mod meta_name_value;
mod meta_path;

use crate::attribute::common::message_format::{
    default_message_format, extract_custom_message_format,
};
use crate::attribute::{
    MetaListFieldValidation, MetaNameValueFieldValidation, MetaPathFieldValidation, Validator,
};
use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::types::SingleIdentPath;
use crate::warning::WithWarnings;
use meta_list::extract_field_validator_from_meta_list;
use meta_name_value::extract_field_validator_from_meta_name_value;
use meta_path::extract_field_validator_from_meta_path;
use std::str::FromStr;

use super::generic::extract_generic_validate_validator;

pub fn extract_field_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    rename_map: &RenameMap,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    match &attribute.meta {
        syn::Meta::List(list) => inner_extract_field_validator(field, attribute, list, rename_map),
        syn::Meta::Path(_) => extract_generic_validate_validator(field, rename_map),
        syn::Meta::NameValue(name_value) => {
            Err(vec![crate::Error::validate_meta_name_value_not_supported(
                name_value,
            )])
        }
    }
}

fn inner_extract_field_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
    rename_map: &RenameMap,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    let mut errors = vec![];
    let nested = meta_list
        .parse_args_with(crate::types::CommaSeparatedMetas::parse_terminated)
        .map_err(|error| {
            vec![crate::Error::validate_attribute_parse_error(
                attribute, &error,
            )]
        })?;

    let WithWarnings {
        data: message_format,
        mut warnings,
    } = match nested.len() {
        0 => Err(vec![crate::Error::field_validation_type_required(
            attribute,
        )])?,
        1 => None,
        2 => match extract_custom_message_format(&nested[1]) {
            Ok(custom_message) => {
                if nested[0].path().is_ident("custom") {
                    errors.push(
                        crate::Error::validate_custom_does_not_support_custom_message(&nested[1]),
                    );
                    None
                } else {
                    Some(custom_message)
                }
            }
            Err(message_fn_errors) => {
                errors.extend(message_fn_errors);
                None
            }
        },
        _ => {
            for meta in nested.iter().skip(2) {
                errors.push(crate::Error::too_many_list_items(meta));
            }
            None
        }
    }
    .unwrap_or_else(|| WithWarnings::new(default_message_format()));

    let meta = &nested[0];

    let validation_path = match meta {
        syn::Meta::Path(path) => path,
        syn::Meta::List(list) => &list.path,
        syn::Meta::NameValue(name_value) => &name_value.path,
    };

    let validation_name = SingleIdentPath::new(validation_path).ident().to_string();

    let validator = match (
        MetaPathFieldValidation::from_str(&validation_name),
        MetaListFieldValidation::from_str(&validation_name),
        MetaNameValueFieldValidation::from_str(&validation_name),
        meta,
    ) {
        (Ok(validation_type), _, _, syn::Meta::Path(validation)) => {
            extract_field_validator_from_meta_path(
                field,
                validation_type,
                validation,
                message_format,
                rename_map,
            )
        }

        (_, Ok(validation_type), _, syn::Meta::List(validation)) => {
            extract_field_validator_from_meta_list(
                field,
                validation_type,
                validation,
                message_format,
                rename_map,
            )
        }

        (_, _, Ok(validation_type), syn::Meta::NameValue(validation)) => {
            extract_field_validator_from_meta_name_value(
                field,
                validation_type,
                validation,
                message_format,
                rename_map,
            )
        }

        (Ok(_), _, _, _) => Err(vec![crate::Error::meta_path_validation_need_value(
            validation_path,
            &validation_name,
        )]),

        (_, Ok(_), _, _) => Err(vec![crate::Error::meta_list_validation_need_value(
            validation_path,
            &validation_name,
        )]),

        (_, _, Ok(_), _) => Err(vec![crate::Error::meta_name_value_validation_need_value(
            validation_path,
            &validation_name,
        )]),

        _ => Err(vec![crate::Error::field_validation_type_unknown(
            validation_path,
            &validation_name,
        )]),
    }
    .map(|validator| {
        warnings.extend(validator.warnings);
        validator.data
    });

    match validator {
        Ok(validator) => {
            if errors.is_empty() {
                Ok(WithWarnings {
                    data: validator,
                    warnings,
                })
            } else {
                Err(errors)
            }
        }
        Err(validator_errors) => {
            errors.extend(validator_errors);
            Err(errors)
        }
    }
}
