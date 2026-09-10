use serde_json::json;
use serde_valid::{MinimumError, Validate, ValidateMinimum};
use std::borrow::Cow;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
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
fn transparent_forwards_all_pointer_wrappers_for_direct_validate_minimum() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(minimum = 1)]
        boxed_custom: Box<CustomNumber>,
        #[validate(minimum = 1)]
        rc_custom: Rc<CustomNumber>,
        #[validate(minimum = 1)]
        arc_custom: Arc<CustomNumber>,
        #[validate(minimum = 1)]
        borrowed_custom: &'a CustomNumber,
        #[validate(minimum = 1)]
        cow_custom: Cow<'a, CustomNumber>,
        #[validate(minimum = 1)]
        pinned_custom: Pin<Box<CustomNumber>>,
        #[validate(minimum = 1)]
        boxed_builtin: Box<i32>,
        #[validate(minimum = 1)]
        rc_builtin: Rc<i32>,
        #[validate(minimum = 1)]
        arc_builtin: Arc<i32>,
        #[validate(min_length = 1)]
        cow_str: Cow<'a, str>,
    }

    let custom = CustomNumber(0);
    let value = Cases {
        boxed_custom: Box::new(CustomNumber(0)),
        rc_custom: Rc::new(CustomNumber(0)),
        arc_custom: Arc::new(CustomNumber(0)),
        borrowed_custom: &custom,
        cow_custom: Cow::Owned(CustomNumber(0)),
        pinned_custom: Pin::new(Box::new(CustomNumber(0))),
        boxed_builtin: Box::new(0),
        rc_builtin: Rc::new(0),
        arc_builtin: Arc::new(0),
        cow_str: Cow::Borrowed(""),
    };

    assert_eq!(
        serde_json::to_value(value.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "boxed_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "rc_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "arc_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "borrowed_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "cow_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "pinned_custom": {
                    "errors": ["The number must be `>= 1`."]
                },
                "boxed_builtin": {
                    "errors": ["The number must be `>= 1`."]
                },
                "rc_builtin": {
                    "errors": ["The number must be `>= 1`."]
                },
                "arc_builtin": {
                    "errors": ["The number must be `>= 1`."]
                },
                "cow_str": {
                    "errors": ["The length of the value must be `>= 1`."]
                }
            }
        })
    );
}

#[test]
fn transparent_nests_pointer_wrappers() {
    #[derive(Validate)]
    #[allow(clippy::box_collection, clippy::redundant_allocation)]
    struct Cases {
        #[validate(minimum = 1)]
        boxed_rc: Box<Rc<CustomNumber>>,
        #[validate(minimum = 1)]
        boxed_boxed_builtin: Box<Box<i32>>,
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                boxed_rc: Box::new(Rc::new(CustomNumber(0))),
                boxed_boxed_builtin: Box::new(Box::new(0)),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "boxed_rc": {
                    "errors": ["The number must be `>= 1`."]
                },
                "boxed_boxed_builtin": {
                    "errors": ["The number must be `>= 1`."]
                }
            }
        })
    );
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

#[test]
fn transparent_forwards_enum_and_pattern() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(r#enum = [1, 2])]
        boxed_enum: Box<i32>,
        #[validate(pattern = r"^\d+$")]
        cow_pattern: Cow<'a, str>,
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                boxed_enum: Box::new(9),
                cow_pattern: Cow::Borrowed("x"),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "boxed_enum": {
                    "errors": ["The value must be in [1, 2]."]
                },
                "cow_pattern": {
                    "errors": ["The value must match the pattern of \"^\\d+$\"."]
                }
            }
        })
    );
}

#[test]
fn serde_borrow_shaped_fields_compose_across_paths() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(min_length = 1)]
        name: &'a str,
        #[validate(min_length = 1)]
        tags: &'a [&'a str],
        #[validate(min_length = 1)]
        maybe_name: Option<&'a str>,
        #[validate(min_length = 1)]
        maybe_tags: Option<&'a [&'a str]>,
        #[validate(min_length = 1)]
        cow_name: Cow<'a, str>,
        #[validate(minimum = 1)]
        cow_nums: Cow<'a, [i32]>,
        #[validate(minimum = 1)]
        maybe_map: Option<std::collections::HashMap<&'a str, i32>>,
    }

    let value = Cases {
        name: "",
        tags: &[""],
        maybe_name: Some(""),
        maybe_tags: Some(&[""]),
        cow_name: Cow::Borrowed(""),
        cow_nums: Cow::Borrowed(&[0]),
        maybe_map: Some(std::collections::HashMap::from([("k", 0)])),
    };

    assert_eq!(
        serde_json::to_value(value.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "name": {
                    "errors": ["The length of the value must be `>= 1`."]
                },
                "tags": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                },
                "maybe_name": {
                    "errors": ["The length of the value must be `>= 1`."]
                },
                "maybe_tags": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                },
                "cow_name": {
                    "errors": ["The length of the value must be `>= 1`."]
                },
                "cow_nums": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                },
                "maybe_map": {
                    "errors": [],
                    "properties": {
                        "k": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                }
            }
        })
    );
}
