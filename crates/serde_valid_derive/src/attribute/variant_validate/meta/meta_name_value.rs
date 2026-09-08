use crate::attribute::{
    common::message_format::MessageFormat,
    struct_validate::generic::extract_generic_struct_custom_validator_from_meta_name_value,
    MetaNameValueStructValidation, Validator,
};

#[inline]
pub fn extract_variant_validator_from_meta_name_value(
    validation_type: MetaNameValueStructValidation,
    validation: &syn::MetaNameValue,
    message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaNameValueStructValidation::Custom => {
            extract_generic_struct_custom_validator_from_meta_name_value(validation, message_format)
        }
    }
}
