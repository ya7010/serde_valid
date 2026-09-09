use serde_valid::Validate;

use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[test]
fn properties_hash_map_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_properties = 3)]
        #[validate(max_properties = 3)]
        val: HashMap<String, String>,
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    map.insert("key3".to_string(), "value3".to_string());

    let s = TestStruct { val: map };
    assert!(s.validate().is_ok());
}

#[test]
fn properties_btree_map_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_properties = 3)]
        #[validate(max_properties = 3)]
        val: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    map.insert("key3".to_string(), "value3".to_string());

    let s = TestStruct { val: map };
    assert!(s.validate().is_ok());
}

#[test]
fn properties_json_value_map_type() {
    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[validate(min_properties = 3)]
        #[validate(max_properties = 3)]
        val: serde_json::Map<String, serde_json::Value>,
    }

    let s: TestStruct = serde_json::from_value(json!({
        "val": {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        }
    }))
    .unwrap();
    assert!(s.validate().is_ok());
}

#[test]
fn properties_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_properties = 3)]
        #[validate(max_properties = 3)]
        val: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let s = TestStruct { val: map };
    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": { "errors": ["The size of the properties must be `>= 3`."] }
            }
        })
    );
}

#[test]
fn properties_hash_map_type_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_properties = 3)]
        #[validate(max_properties = 3)]
        val: HashMap<String, String>,
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());

    let s = TestStruct { val: map };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "The size of the properties must be `>= 3`."
                    ]
                }
            }
        })
    );
}

#[test]
fn properties_btree_map_type_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_properties = 3)]
        #[validate(max_properties = 3)]
        val: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), "value1".to_string());

    let s = TestStruct { val: map };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "The size of the properties must be `>= 3`."
                    ]
                }
            }
        })
    );
}

#[test]
fn properties_json_map_type_err_message() {
    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[validate(min_properties = 3)]
        #[validate(max_properties = 3)]
        val: serde_json::Map<String, serde_json::Value>,
    }

    let s: TestStruct = serde_json::from_value(json!({
        "val": {
            "key1": "value1",
        }
    }))
    .unwrap();

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "The size of the properties must be `>= 3`.",
                    ]
                }
            }
        })
    );
}

#[test]
fn properties_custom_err_message_fn() {
    fn min_custom_error_message(_params: &serde_valid::MinPropertiesError) -> String {
        "this is min custom message.".to_string()
    }

    fn max_custom_error_message(_params: &serde_valid::MaxPropertiesError) -> String {
        "this is max custom message.".to_string()
    }

    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[validate(min_properties = 3, message_fn = min_custom_error_message)]
        #[validate(max_properties = 1, message_fn = max_custom_error_message)]
        val: serde_json::Map<String, serde_json::Value>,
    }

    let s: TestStruct = serde_json::from_value(json!({
        "val": {
            "key1": "value1",
            "key2": "value2",
        }
    }))
    .unwrap();

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
fn properties_custom_err_message() {
    #[derive(Deserialize, Validate)]
    struct TestStruct {
        #[validate(min_properties = 3, message = "this is min custom message.")]
        #[validate(max_properties = 1, message = "this is max custom message.")]
        val: serde_json::Map<String, serde_json::Value>,
    }

    let s: TestStruct = serde_json::from_value(json!({
        "val": {
            "key1": "value1",
            "key2": "value2",
        }
    }))
    .unwrap();

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
