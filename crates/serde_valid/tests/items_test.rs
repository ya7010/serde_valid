use serde_json::json;
use serde_valid::Validate;

#[test]
fn items_vec_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 4)]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_array_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 4)]
        val: [i32; 4],
    }

    let s = TestStruct { val: [1, 2, 3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_min_items_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 3)]
        #[validate(max_items = 5)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_min_items_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 5)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": { "errors": ["The length of the items must be `>= 4`."] }
            }
        })
    );
}

#[test]
fn items_max_items_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 0)]
        #[validate(max_items = 4)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_max_items_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 1)]
        #[validate(max_items = 2)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": { "errors": ["The length of the items must be `<= 2`."] }
            }
        })
    );
}

#[test]
fn items_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 2)]
        #[validate(max_items = 2)]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![], vec![1, 2, 3]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 4)]
        val: Option<Vec<i32>>,
    }

    let s = TestStruct {
        val: Some(vec![1, 2, 3, 4]),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_nested_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 3)]
        #[validate(max_items = 3)]
        val: Option<Option<Vec<i32>>>,
    }

    let s = TestStruct {
        val: Some(Some(vec![1, 2, 3])),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_vec_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 3)]
        #[validate(max_items = 3)]
        val: Vec<Option<Vec<i32>>>,
    }

    let s = TestStruct {
        val: vec![Some(vec![1, 2, 3, 4]), Some(vec![1, 2, 3, 4]), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 4)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["The length of the items must be `>= 4`."]
                }
            }
        })
    );
}

#[test]
fn items_custom_err_message_fn() {
    #[inline]
    fn min_error_message(_params: &serde_valid::MinItemsError) -> String {
        "this is min custom message.".to_string()
    }
    #[inline]
    fn max_error_message(_params: &serde_valid::MaxItemsError) -> String {
        "this is max custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 4, message_fn = min_error_message)]
        #[validate(max_items = 2, message_fn = max_error_message)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
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
    );
}

#[test]
fn items_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 4, message = "this is min custom message.")]
        #[validate(max_items = 2, message = "this is max custom message.")]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
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
    );
}
