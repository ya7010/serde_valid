use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::field_validate::array::extract_array_unique_items_validator;
use crate::attribute::{MetaPathFieldValidation, Validator};
use crate::serde::rename::RenameMap;
use crate::types::Field;

pub fn extract_field_validator_from_meta_path(
    field: &impl Field,
    validation_type: MetaPathFieldValidation,
    _validation: &syn::Path,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaPathFieldValidation::UniqueItems => Ok(extract_array_unique_items_validator(
            field,
            message_format,
            rename_map,
        )),
    }
}
