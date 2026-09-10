use serde_json::json;
use serde_valid::Validate;
use std::collections::HashMap;

#[test]
fn optional_forwards_scalar_string_numeric_enum_and_properties() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(min_length = 1)]
        optional_str: Option<&'a str>,
        #[validate(minimum = 1)]
        optional_num: Option<i32>,
        #[validate(r#enum = [1, 2, 3])]
        optional_enum: Option<i32>,
        #[validate(min_properties = 1)]
        optional_map_size: Option<HashMap<&'a str, u8>>,
    }

    assert!(Cases {
        optional_str: None,
        optional_num: None,
        optional_enum: None,
        optional_map_size: None,
    }
    .validate()
    .is_ok());

    assert_eq!(
        serde_json::to_value(
            Cases {
                optional_str: Some(""),
                optional_num: Some(0),
                optional_enum: Some(9),
                optional_map_size: Some(HashMap::new()),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "optional_str": {
                    "errors": ["The length of the value must be `>= 1`."]
                },
                "optional_num": {
                    "errors": ["The number must be `>= 1`."]
                },
                "optional_enum": {
                    "errors": ["The value must be in [1, 2, 3]."]
                },
                "optional_map_size": {
                    "errors": ["The size of the properties must be `>= 1`."]
                }
            }
        })
    );
}

#[test]
fn optional_preserves_inner_sequence_and_map_paths() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(min_length = 1)]
        optional_vec: Option<Vec<&'a str>>,
        #[validate(min_length = 1)]
        optional_slice: Option<&'a [&'a str]>,
        #[validate(minimum = 1)]
        optional_map_values: Option<HashMap<&'a str, i32>>,
    }

    assert!(Cases {
        optional_vec: None,
        optional_slice: None,
        optional_map_values: None,
    }
    .validate()
    .is_ok());

    assert_eq!(
        serde_json::to_value(
            Cases {
                optional_vec: Some(vec![""]),
                optional_slice: Some(&[""]),
                optional_map_values: Some(HashMap::from([("k", 0)])),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "optional_vec": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                },
                "optional_slice": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                },
                "optional_map_values": {
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

#[test]
#[allow(clippy::box_collection)]
fn optional_composes_with_sequence_wrappers() {
    #[derive(Validate)]
    struct Cases {
        #[validate(minimum = 1)]
        optional_boxed_vec: Option<Box<Vec<i32>>>,
        #[validate(minimum = 1)]
        boxed_optional_vec: Box<Option<Vec<i32>>>,
    }

    assert!(Cases {
        optional_boxed_vec: None,
        boxed_optional_vec: Box::new(None),
    }
    .validate()
    .is_ok());

    assert_eq!(
        serde_json::to_value(
            Cases {
                optional_boxed_vec: Some(Box::new(vec![0])),
                boxed_optional_vec: Box::new(Some(vec![0])),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "optional_boxed_vec": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                },
                "boxed_optional_vec": {
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
fn nested_options_compose_on_scalar_and_sequence_paths() {
    #[derive(Validate)]
    struct Cases {
        #[validate(minimum = 1)]
        nested_option: Option<Option<i32>>,
        #[validate(minimum = 1)]
        option_of_vec_of_option: Option<Vec<Option<i32>>>,
    }

    assert!(Cases {
        nested_option: None,
        option_of_vec_of_option: None,
    }
    .validate()
    .is_ok());

    assert_eq!(
        serde_json::to_value(
            Cases {
                nested_option: Some(Some(0)),
                option_of_vec_of_option: Some(vec![Some(0), None]),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "nested_option": {
                    "errors": ["The number must be `>= 1`."]
                },
                "option_of_vec_of_option": {
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
fn optional_covers_pattern_array_and_cow_borrow_shapes() {
    use std::borrow::Cow;

    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(pattern = r"^\d+$")]
        optional_pattern: Option<&'a str>,
        #[validate(minimum = 1)]
        optional_array: Option<[i32; 1]>,
        #[validate(min_length = 1)]
        optional_cow: Option<Cow<'a, str>>,
        #[validate(minimum = 1)]
        optional_cow_slice: Option<Cow<'a, [i32]>>,
        #[validate(min_length = 1)]
        optional_boxed_slice: Option<Box<[&'a str]>>,
    }

    assert!(Cases {
        optional_pattern: None,
        optional_array: None,
        optional_cow: None,
        optional_cow_slice: None,
        optional_boxed_slice: None,
    }
    .validate()
    .is_ok());

    assert_eq!(
        serde_json::to_value(
            Cases {
                optional_pattern: Some("x"),
                optional_array: Some([0]),
                optional_cow: Some(Cow::Borrowed("")),
                optional_cow_slice: Some(Cow::Borrowed(&[0])),
                optional_boxed_slice: Some(Box::new([""])),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "optional_pattern": {
                    "errors": ["The value must match the pattern of \"^\\d+$\"."]
                },
                "optional_array": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                },
                "optional_cow": {
                    "errors": ["The length of the value must be `>= 1`."]
                },
                "optional_cow_slice": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The number must be `>= 1`."]
                        }
                    }
                },
                "optional_boxed_slice": {
                    "errors": [],
                    "items": {
                        "0": {
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                }
            }
        })
    );
}
