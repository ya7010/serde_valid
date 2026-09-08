use serde_json::json;
use serde_valid::Validate;

#[test]
fn range_integer() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    let s = TestStruct { val: 1234 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_float() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0.0)]
        #[validate(maximum = 2000.0)]
        val: f32,
    }

    let s = TestStruct { val: 1234.5678 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_minimum_minus_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = -1)]
        val: i32,
    }

    let s = TestStruct { val: -2 };
    assert!(s.validate().is_err());

    let s = TestStruct { val: -1 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_minimum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_minimum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_maximum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_maximum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 11 };
    assert!(s.validate().is_err());
}

#[test]
fn range_exclusive_maximum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_maximum = 10)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 9 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_maximum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_maximum = 10)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_err());
}

#[test]
fn range_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 20)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![12, 16] };
    assert!(s.validate().is_ok());
}

#[test]
fn range_nested_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 20)]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![4, 8], vec![12, 16]],
    };
    assert!(s.validate().is_ok());
}

#[derive(Validate)]
struct RangeValue {
    #[validate(minimum = 0)]
    #[validate(maximum = 10)]
    value: i32,
}

#[test]
fn range_validation_runs_through_optional_nested_validation() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: Option<Option<RangeValue>>,
    }

    assert!(TestStruct {
        val: Some(Some(RangeValue { value: 5 })),
    }
    .validate()
    .is_ok());
    assert!(TestStruct {
        val: Some(Some(RangeValue { value: 11 })),
    }
    .validate()
    .is_err());
    assert!(TestStruct { val: None }.validate().is_ok());
}

#[test]
fn range_validation_runs_through_sequence_nested_validation() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        vec: Vec<Option<RangeValue>>,
        #[validate]
        array: [Option<RangeValue>; 3],
    }

    assert!(TestStruct {
        vec: vec![Some(RangeValue { value: 4 }), None],
        array: [
            Some(RangeValue { value: 4 }),
            Some(RangeValue { value: 8 }),
            None,
        ],
    }
    .validate()
    .is_ok());
}

#[test]
fn range_inclusive_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };

    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "The number must be `>= 1`."
                    ]
                }
            }
        })
        .to_string()
    );
}

#[test]
fn range_minimum_minus_with_message_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = -1, message = "foo")]
        val: i32,
    }

    let s = TestStruct { val: -2 };
    assert!(s.validate().is_err());

    let s = TestStruct { val: -1 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 1)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };

    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "The number must be `> 1`."
                    ]
                }
            }
        })
        .to_string()
    );
}

#[test]
fn range_custom_err_message_fn() {
    fn custom_min_error_message(_params: &serde_valid::MinimumError) -> String {
        "this is min custom message.".to_string()
    }

    fn custom_max_error_message(_params: &serde_valid::MaximumError) -> String {
        "this is max custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 5, message_fn = custom_min_error_message)]
        #[validate(maximum = 3, message_fn = custom_max_error_message)]
        val: i32,
    }

    let s = TestStruct { val: 4 };

    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "this is min custom message.",
                        "this is max custom message."
                    ]
                }
            }
        })
        .to_string()
    );
}

#[test]
fn range_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 5, message = "this is min custom message.")]
        #[validate(maximum = 3, message = "this is max custom message.")]
        val: i32,
    }

    let s = TestStruct { val: 4 };
    let result = s.validate().unwrap_err();

    assert_eq!(
        serde_json::to_string(&result).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "this is min custom message.",
                        "this is max custom message."
                    ]
                }
            }
        })
        .to_string()
    );
}
