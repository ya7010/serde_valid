use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::field_validate::generic::extract_generic_custom_validator_from_meta_list;
use crate::attribute::{MetaListFieldValidation, Validator};
use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::warning::WithWarnings;

pub fn extract_field_validator_from_meta_list(
    field: &impl Field,
    validation_type: MetaListFieldValidation,
    validation: &syn::MetaList,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    match validation_type {
        MetaListFieldValidation::Custom => extract_generic_custom_validator_from_meta_list(
            field,
            validation,
            message_format,
            rename_map,
        )
        .map(WithWarnings::new),
    }
}
