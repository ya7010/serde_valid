use crate::{
    attribute::{
        common::message_format::MessageFormat,
        struct_validate::generic::extract_generic_struct_custom_validator_from_meta_name_value,
        MetaNameValueStructValidation, Validator,
    },
    warning::WithWarnings,
};

#[inline]
pub fn extract_struct_validator_from_meta_name_value(
    validation_type: MetaNameValueStructValidation,
    validation: &syn::MetaNameValue,
    message_format: MessageFormat,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    match validation_type {
        MetaNameValueStructValidation::Custom => {
            extract_generic_struct_custom_validator_from_meta_name_value(validation, message_format)
        }
    }
    .map(WithWarnings::new)
}
