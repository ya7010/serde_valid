mod meta;

use crate::{attribute::Validator, warning::WithWarnings};

use self::meta::extract_variant_validator;

pub fn collect_variant_custom_from_variant(
    attributes: &[syn::Attribute],
) -> Result<WithWarnings<Validator>, crate::Errors> {
    let mut errors = vec![];
    let mut warnings = vec![];

    let validations = attributes
        .iter()
        .filter_map(|attribute| {
            if attribute.path().is_ident("validate") {
                match extract_variant_validator(attribute) {
                    Ok(validator) => {
                        warnings.extend(validator.warnings);
                        Some(validator.data)
                    }
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
        Ok(WithWarnings::new_with_warnings(
            Validator::from_iter(validations),
            warnings,
        ))
    } else {
        Err(errors)
    }
}
