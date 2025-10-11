use crate::attribute::common::lit::get_lit;
use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::field_validate::array::{
    extract_array_max_items_validator, extract_array_min_items_validator,
};
use crate::attribute::field_validate::generic::{
    extract_generic_custom_validator_from_meta_name_value,
    extract_generic_enumerate_validator_from_name_value,
};
use crate::attribute::field_validate::numeric::{
    extract_numeric_exclusive_maximum_validator, extract_numeric_exclusive_minimum_validator,
    extract_numeric_maximum_validator, extract_numeric_minimum_validator,
    extract_numeric_multiple_of_validator,
};
use crate::attribute::field_validate::object::{
    extract_object_max_properties_validator, extract_object_min_properties_validator,
};
use crate::attribute::field_validate::string::{
    extract_string_max_length_validator, extract_string_min_length_validator,
    extract_string_pattern_validator,
};
use crate::attribute::{MetaNameValueFieldValidation, Validator};
use crate::serde::rename::RenameMap;
use crate::types::Field;

pub fn extract_field_validator_from_meta_name_value(
    field: &impl Field,
    validation_type: MetaNameValueFieldValidation,
    validation: &syn::MetaNameValue,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaNameValueFieldValidation::Minimum => {
            extract_numeric_minimum_validator(field, &validation.value, message_format, rename_map)
        }
        MetaNameValueFieldValidation::Maximum => {
            extract_numeric_maximum_validator(field, &validation.value, message_format, rename_map)
        }
        MetaNameValueFieldValidation::ExclusiveMinimum => {
            extract_numeric_exclusive_minimum_validator(
                field,
                &validation.value,
                message_format,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::ExclusiveMaximum => {
            extract_numeric_exclusive_maximum_validator(
                field,
                &validation.value,
                message_format,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::MinLength => {
            let validation_value = get_lit(&validation.value)?;
            extract_string_min_length_validator(field, validation_value, message_format, rename_map)
        }
        MetaNameValueFieldValidation::MaxLength => {
            let validation_value = get_lit(&validation.value)?;
            extract_string_max_length_validator(field, validation_value, message_format, rename_map)
        }
        MetaNameValueFieldValidation::MinItems => {
            let validation_value = get_lit(&validation.value)?;
            extract_array_min_items_validator(field, validation_value, message_format, rename_map)
        }
        MetaNameValueFieldValidation::MaxItems => {
            let validation_value = get_lit(&validation.value)?;
            extract_array_max_items_validator(field, validation_value, message_format, rename_map)
        }
        MetaNameValueFieldValidation::MinProperties => {
            let validation_value = get_lit(&validation.value)?;
            extract_object_min_properties_validator(
                field,
                validation_value,
                message_format,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::MaxProperties => {
            let validation_value = get_lit(&validation.value)?;
            extract_object_max_properties_validator(
                field,
                validation_value,
                message_format,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::MultipleOf => {
            let validation_value = get_lit(&validation.value)?;
            extract_numeric_multiple_of_validator(
                field,
                validation_value,
                message_format,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::Pattern => {
            let validation_value = get_lit(&validation.value)?;
            extract_string_pattern_validator(field, validation_value, message_format, rename_map)
        }
        MetaNameValueFieldValidation::Enumerate => {
            extract_generic_enumerate_validator_from_name_value(
                field,
                validation,
                message_format,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::Custom => {
            extract_generic_custom_validator_from_meta_name_value(
                field,
                validation,
                message_format,
                rename_map,
            )
        }
    }
}
