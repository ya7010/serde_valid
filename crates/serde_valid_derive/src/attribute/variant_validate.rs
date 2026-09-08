mod meta;

use crate::attribute::Validator;

use self::meta::extract_variant_validator;

pub fn collect_variant_custom_from_variant(
    attributes: &[syn::Attribute],
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];

    let validations = attributes
        .iter()
        .filter_map(|attribute| {
            if attribute.path().is_ident("validate") {
                match extract_variant_validator(attribute) {
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
        Ok(Validator::from_iter(validations))
    } else {
        Err(errors)
    }
}
