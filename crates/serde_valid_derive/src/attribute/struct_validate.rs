pub mod generic;
mod meta;

use crate::attribute::Validator;

use self::meta::extract_struct_validator;

pub fn collect_struct_custom_from_named_struct(
    attributes: &[syn::Attribute],
) -> Result<Vec<Validator>, crate::Errors> {
    let mut errors = vec![];

    let validations = attributes
        .iter()
        .filter_map(|attribute| {
            if attribute.path().is_ident("validate") {
                match extract_struct_validator(attribute) {
                    Ok(validator) => Some(validator),
                    Err(validator_error) => {
                        errors.extend(validator_error);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(validations)
    } else {
        Err(errors)
    }
}
