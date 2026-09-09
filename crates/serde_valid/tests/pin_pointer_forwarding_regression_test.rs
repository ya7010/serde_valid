use serde_valid::{Validate, ValidateExclusiveMinimum};
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Validate)]
struct PinnedPointerConstraints {
    #[validate(min_length = 2)]
    #[validate(pattern = "^[a-z]+$")]
    #[validate(r#enum = ["allowed"])]
    boxed_string: Pin<Box<str>>,
    #[validate(min_length = 2)]
    #[validate(pattern = "^[a-z]+$")]
    #[validate(r#enum = ["allowed"])]
    rc_string: Pin<Rc<str>>,
    #[validate(min_length = 2)]
    #[validate(pattern = "^[a-z]+$")]
    #[validate(r#enum = ["allowed"])]
    arc_string: Pin<Arc<str>>,
    #[validate(min_properties = 1)]
    boxed_object: Pin<Box<HashMap<String, String>>>,
    #[validate(min_properties = 1)]
    rc_object: Pin<Rc<HashMap<String, String>>>,
    #[validate(min_properties = 1)]
    arc_object: Pin<Arc<HashMap<String, String>>>,
}

#[test]
fn pin_forwards_every_scalar_validator_through_its_pointer_target() {
    let errors = serde_json::to_value(
        PinnedPointerConstraints {
            boxed_string: Pin::new(Box::<str>::from("1")),
            rc_string: Pin::new(Rc::from("1")),
            arc_string: Pin::new(Arc::from("1")),
            boxed_object: Pin::new(Box::new(HashMap::new())),
            rc_object: Pin::new(Rc::new(HashMap::new())),
            arc_object: Pin::new(Arc::new(HashMap::new())),
        }
        .validate()
        .unwrap_err(),
    )
    .unwrap();

    for field in [
        "boxed_string",
        "rc_string",
        "arc_string",
        "boxed_object",
        "rc_object",
        "arc_object",
    ] {
        assert!(
            errors["properties"].get(field).is_some(),
            "missing {field}: {errors}"
        );
    }
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
    assert!(CustomScalarConstraint {
        value: CustomNumber(1),
    }
    .validate()
    .is_err());
}
