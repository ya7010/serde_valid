use std::ops::Deref;

use serde_valid::Validate;

struct StringWrapper(String);

impl StringWrapper {
    fn __serde_valid_autoderef_validate_composited_min_length(
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

#[derive(Validate)]
struct Input {
    #[validate(min_length = 1)]
    value: StringWrapper,
}

#[test]
fn inherent_methods_cannot_override_generated_autoderef_dispatch() {
    assert!(Input {
        value: StringWrapper(String::new()),
    }
    .validate()
    .is_err());
}
