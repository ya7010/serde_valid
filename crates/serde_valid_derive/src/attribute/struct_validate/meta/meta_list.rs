use crate::{
    attribute::{common::message_format::MessageFormat, MetaListStructValidation, Validator},
    warning::WithWarnings,
};

pub fn extract_struct_validator_from_meta_list(
    validation_type: MetaListStructValidation,
    _validation: &syn::MetaList,
    _message_format: MessageFormat,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    match validation_type {}
}
