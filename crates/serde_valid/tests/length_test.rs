use serde_json::json;
use serde_valid::Validate;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};

#[test]
fn length_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: &'a str,
    }

    let s = TestStruct { val: "test" };
    assert!(s.validate().is_ok());
}

#[test]
fn length_cow_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: Cow<'a, str>,
    }

    let s = TestStruct {
        val: Cow::from("test"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_os_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: &'a OsStr,
    }

    let s = TestStruct {
        val: OsStr::new("fo�o"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_os_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 4)]
        #[validate(max_length = 4)]
        val: OsString,
    }

    let s = TestStruct {
        val: OsString::from("fo�o"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_path_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(min_length = 13)]
        #[validate(max_length = 13)]
        val: &'a std::path::Path,
    }

    let s = TestStruct {
        val: std::path::Path::new("./foo/bar.txt"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_path_buf_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 13)]
        #[validate(max_length = 13)]
        val: std::path::PathBuf,
    }

    let s = TestStruct {
        val: std::path::PathBuf::from("./foo/bar.txt"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_min_length_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 10)]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    assert!(s.validate().is_ok());
}

#[test]
fn length_min_length_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 1)]
        #[validate(max_length = 10)]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": { "errors": ["The length of the value must be `>= 1`."] }
            }
        })
    );
}

#[test]
fn length_max_length_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 5)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("abcde"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_max_length_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 1)]
        #[validate(max_length = 3)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("abcd"),
    };
    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": { "errors": ["The length of the value must be `<= 3`."] }
            }
        })
    );
}

#[test]
fn length_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(min_length = 0)]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![String::from("abcd"), String::from("efg")],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_nested_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 0)]
        #[validate(max_length = 3)]
        val: Vec<Vec<String>>,
    }

    let s = TestStruct {
        val: vec![
            vec![String::from(""), String::from("1")],
            vec![String::from("12"), String::from("123")],
        ],
    };
    assert!(s.validate().is_ok());
}

#[derive(Validate)]
struct LengthValue {
    #[validate(min_length = 0)]
    #[validate(max_length = 5)]
    value: String,
}

#[test]
fn length_validation_runs_through_optional_nested_validation() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: Option<LengthValue>,
    }

    assert!(TestStruct {
        val: Some(LengthValue {
            value: "abcd".to_owned(),
        }),
    }
    .validate()
    .is_ok());
    assert_eq!(
        serde_json::to_value(
            TestStruct {
                val: Some(LengthValue {
                    value: "abcdefg".to_owned(),
                }),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [],
                    "properties": {
                        "value": { "errors": ["The length of the value must be `<= 5`."] }
                    }
                }
            }
        })
    );
    assert!(TestStruct { val: None }.validate().is_ok());
}

#[test]
fn length_validation_runs_through_nested_optional_validation() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: Option<Option<LengthValue>>,
    }

    assert!(TestStruct {
        val: Some(Some(LengthValue {
            value: "abcd".to_owned(),
        })),
    }
    .validate()
    .is_ok());
}

#[test]
fn length_validation_runs_through_sequence_nested_validation() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: Vec<Option<LengthValue>>,
    }

    assert!(TestStruct {
        val: vec![
            Some(LengthValue {
                value: "abc".to_owned(),
            }),
            Some(LengthValue {
                value: "abcde".to_owned(),
            }),
            None,
        ],
    }
    .validate()
    .is_ok());
}

#[test]
fn length_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 1)]
        #[validate(max_length = 3)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "The length of the value must be `<= 3`."
                    ]
                }
            }
        })
    );
}

#[test]
fn length_custom_err_message_fn() {
    fn custom_min_error_message(_params: &serde_valid::MinLengthError) -> String {
        "this is min custom message.".to_string()
    }

    fn custom_max_error_message(_params: &serde_valid::MaxLengthError) -> String {
        "this is max custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 5, message_fn = custom_min_error_message)]
        #[validate(max_length = 3, message_fn = custom_max_error_message)]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };

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
fn length_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 5, message = "this is min custom message.")]
        #[validate(max_length = 3, message = "this is max custom message.")]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };

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
fn length_vec_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_length = 5, message = "this is min custom message.")]
        #[validate(max_length = 3, message = "this is max custom message.")]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![String::from("test")],
    };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": [
                                "this is min custom message.",
                                "this is max custom message."
                            ]
                        }
                    }
                }
            }
        })
    );
}

#[test]
fn multi_items_error() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 2)]
        #[validate(max_items = 5)]
        #[validate(min_length = 1)]
        #[validate(max_length = 5)]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![
            "user1",
            "",
            "user3",
            "too_long",
            "user5",
            "bbb",
            "last_error",
        ]
        .into_iter()
        .map(Into::into)
        .collect(),
    };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!(
            {
                "errors": [],
                "properties": {
                    "val": {
                        "errors": ["The length of the items must be `<= 5`."],
                        "items": {
                            "1": { "errors": ["The length of the value must be `>= 1`."] },
                            "3": { "errors": ["The length of the value must be `<= 5`."] },
                            "6": { "errors": ["The length of the value must be `<= 5`."] }
                        }
                    }
                }
            }
        )
    );
}
