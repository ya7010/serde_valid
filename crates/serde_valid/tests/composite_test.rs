use std::collections::HashMap;

use serde_json::json;
use serde_valid::Validate;

#[test]
fn composite_box_type() {
    #[derive(Validate)]
    struct BoxStruct<'a> {
        #[validate(min_length = 1)]
        val: Box<[&'a str]>
    }

    let s = BoxStruct {
        val: Box::new([""])
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
                            "errors": ["The length of the value must be `>= 1`."]
                        }
                    }
                }
            }
        })
    );
}

#[test]
fn composite_option_type() {
    #[derive(Validate)]
    struct OptionStruct<'a> {
        #[validate(min_length = 1)]
        val: Option<&'a str>
    }

    #[derive(Validate)]
    struct OptionSliceStruct<'a> {
        #[validate(min_length = 1)]
        val: Option<&'a [&'a str]>
    }

    #[derive(Validate)]
    struct OptionMapStruct<'a> {
        #[validate(min_properties = 1)]
        val: Option<HashMap<&'a str, u8>>
    }

    let s = OptionStruct {
        val: Some("")
    };

    let s1 = OptionSliceStruct {
        val: Some(&[""])
    };

    let s2 = OptionMapStruct {
        val: Some(HashMap::new())
    };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["The length of the value must be `>= 1`."]
                }
            }
        })
    );

    assert_eq!(
        serde_json::to_value(s1.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
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

    assert_eq!(
        serde_json::to_value(s2.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["The size of the properties must be `>= 1`."]
                }
            }
        })
    );
}
