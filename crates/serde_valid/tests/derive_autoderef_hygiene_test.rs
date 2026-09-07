use std::ops::Deref;

use serde_valid::Validate;

struct StringWrapper(String);

#[allow(non_snake_case)]
impl StringWrapper {
    fn __serde_valid_autoderef_validate_composited_min_length(
        &self,
        _min_length: usize,
    ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinLengthError>> {
        Ok(())
    }

    fn __serde_valid_autoderef_validate_composited_min_length___serde_valid_value(
        &self,
        _min_length: usize,
    ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinLengthError>> {
        Ok(())
    }
}

impl Deref for StringWrapper {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde_valid::validation::ValidateCompositedMinLength for StringWrapper {
    fn validate_composited_min_length(
        &self,
        min_length: usize,
    ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinLengthError>> {
        serde_valid::validation::ValidateCompositedMinLength::validate_composited_min_length(
            self.0.as_str(),
            min_length,
        )
    }
}

#[derive(Validate)]
struct Input {
    #[validate(min_length = 1)]
    value: StringWrapper,
}

#[test]
fn inherent_methods_cannot_override_trait_qualified_dispatch() {
    assert!(Input {
        value: StringWrapper(String::new()),
    }
    .validate()
    .is_err());
}
