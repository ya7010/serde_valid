use serde_json::json;
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
    #[allow(clippy::box_collection)]
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

    let string_errors = json!({
        "errors": [
            "The length of the value must be `>= 2`.",
            "The value must match the pattern of \"^[a-z]+$\".",
            "The value must be in [allowed]."
        ]
    });
    let object_errors = json!({
        "errors": ["The size of the properties must be `>= 1`."]
    });

    assert_eq!(
        errors,
        json!({
            "errors": [],
            "properties": {
                "boxed_string": string_errors,
                "rc_string": string_errors,
                "arc_string": string_errors,
                "boxed_object": object_errors,
                "rc_object": object_errors,
                "arc_object": object_errors,
            }
        })
    );
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
    assert_eq!(
        serde_json::to_value(
            CustomScalarConstraint {
                value: CustomNumber(1),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": ["The number must be `> 1`."]
                }
            }
        })
    );
}
