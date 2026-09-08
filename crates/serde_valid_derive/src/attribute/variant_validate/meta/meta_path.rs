use crate::attribute::{
    common::message_format::MessageFormat, MetaPathStructValidation, Validator,
};

#[inline]
pub fn extract_variant_validator_from_meta_path(
    validation_type: MetaPathStructValidation,
    _validation: &syn::Path,
    _message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}
