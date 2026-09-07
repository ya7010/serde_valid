#![allow(deprecated)]

use serde_valid::{Validate, ValidateExclusiveMinimum};
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn every_standard_pin_pointer_family_has_scalar_forwarding() {
    fn assert_string<T>()
    where
        T: serde_valid::ValidateMinLength
            + serde_valid::ValidatePattern
            + serde_valid::ValidateEnum<&'static str>
            + serde_valid::ValidateEnumerate<&'static str>,
    {
    }

    fn assert_object<T: serde_valid::ValidateMinProperties>() {}

    assert_string::<Pin<Box<str>>>();
    assert_string::<Pin<Rc<str>>>();
    assert_string::<Pin<Arc<str>>>();
    assert_object::<Pin<Box<HashMap<String, String>>>>();
    assert_object::<Pin<Rc<HashMap<String, String>>>>();
    assert_object::<Pin<Arc<HashMap<String, String>>>>();
}

#[derive(Validate)]
struct PinnedPointerConstraints {
    #[validate(min_length = 2)]
    #[validate(pattern = "^[a-z]+$")]
    #[validate(r#enum = ["allowed"])]
    #[validate(enumerate = ["allowed"])]
    string: Pin<Rc<str>>,
    #[validate(min_properties = 1)]
    object: Pin<Arc<HashMap<String, String>>>,
}

#[test]
fn pin_forwards_every_scalar_validator_through_its_pointer_target() {
    let errors = serde_json::to_value(
        PinnedPointerConstraints {
            string: Pin::new(Rc::from("1")),
            object: Pin::new(Arc::new(HashMap::new())),
        }
        .validate()
        .unwrap_err(),
    )
    .unwrap();

    assert!(errors["properties"].get("string").is_some(), "{errors}");
    assert!(errors["properties"].get("object").is_some(), "{errors}");
}

struct CustomNumber(i32);

impl ValidateExclusiveMinimum<i32> for CustomNumber {
    fn validate_exclusive_minimum(
        &self,
        exclusive_minimum: i32,
    ) -> Result<(), serde_valid::ExclusiveMinimumError> {
        self.0.validate_exclusive_minimum(exclusive_minimum)
    }
}

#[derive(Validate)]
struct CustomScalarConstraint {
    #[validate(exclusive_minimum = 1)]
    value: CustomNumber,
}

#[test]
fn public_validator_impl_automatically_provides_composited_validation() {
    fn assert_composited<T: serde_valid::validation::ValidateCompositedExclusiveMinimum<i32>>() {}

    assert_composited::<CustomNumber>();
    assert!(CustomScalarConstraint {
        value: CustomNumber(1),
    }
    .validate()
    .is_err());
}
