use crate::attribute::{
    common::message_format::MessageFormat, MetaListStructValidation, Validator,
};

pub fn extract_variant_validator_from_meta_list(
    validation_type: MetaListStructValidation,
    _validation: &syn::MetaList,
    _message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}
