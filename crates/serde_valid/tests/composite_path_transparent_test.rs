use serde_json::json;
use serde_valid::{MinimumError, Validate, ValidateMinimum};

struct CustomNumber(i32);

impl ValidateMinimum<i32> for CustomNumber {
    fn validate_minimum(&self, minimum: i32) -> Result<(), MinimumError> {
        if self.0 >= minimum {
            Ok(())
        } else {
            Err(MinimumError::new(minimum))
        }
    }
}

#[test]
fn custom_minimum_scalar_works_without_a_wrapper() {
    #[derive(Validate)]
    struct Cases {
        #[validate(minimum = 1)]
        value: CustomNumber,
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                value: CustomNumber(0)
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": ["The number must be `>= 1`."]
                }
            }
        })
    );
}

#[test]
fn transparent_forwards_box_and_rc_for_direct_validate_minimum() {
    #[derive(Validate)]
    struct Cases {
        #[validate(minimum = 1)]
        boxed_custom: Box<CustomNumber>,
        #[validate(minimum = 1)]
        rc_custom: std::rc::Rc<CustomNumber>,
        #[validate(minimum = 1)]
        boxed_builtin: Box<i32>,
        #[validate(minimum = 1)]
        rc_builtin: std::rc::Rc<i32>,
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                boxed_custom: Box::new(CustomNumber(0)),
                rc_custom: std::rc::Rc::new(CustomNumber(0)),
                boxed_builtin: Box::new(0),
                rc_builtin: std::rc::Rc::new(0),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "boxed_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "rc_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "boxed_builtin": {
                    "errors": ["The number must be `>= 1`."]
                },
                "rc_builtin": {
                    "errors": ["The number must be `>= 1`."]
                }
            }
        })
    );

    assert!(Cases {
        boxed_custom: Box::new(CustomNumber(1)),
        rc_custom: std::rc::Rc::new(CustomNumber(1)),
        boxed_builtin: Box::new(1),
        rc_builtin: std::rc::Rc::new(1),
    }
    .validate()
    .is_ok());
}

#[test]
fn transparent_composes_with_optional_and_sequence() {
    #[derive(Validate)]
    #[allow(clippy::box_collection)]
    struct Cases {
        #[validate(minimum = 1)]
        boxed_optional_custom: Box<Option<CustomNumber>>,
        #[validate(minimum = 1)]
        optional_boxed_vec: Option<Box<Vec<i32>>>,
    }

    assert!(Cases {
        boxed_optional_custom: Box::new(None),
        optional_boxed_vec: None,
    }
    .validate()
    .is_ok());

    assert_eq!(
        serde_json::to_value(
            Cases {
                boxed_optional_custom: Box::new(Some(CustomNumber(0))),
                optional_boxed_vec: Some(Box::new(vec![0])),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "boxed_optional_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "optional_boxed_vec": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                }
            }
        })
    );
}
