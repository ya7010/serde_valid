use serde::Deserialize;
use serde_json::json;
use serde_valid::json::FromJsonValue;
use serde_valid::Validate;

#[test]
fn serde_rename_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 100)]
        #[serde(rename = "value")]
        val: i32,
    }

    let s = TestStruct::from_json_value(json!({ "value": 123 }));

    assert!(s.is_ok())
}

#[test]
fn serde_rename_is_err() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(maximum = 100)]
        #[serde(rename = "value")]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "value": 123 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": [
                        "The number must be `<= 100`."
                    ]
                }
            }
        })
    );
}

#[test]
fn serde_rename_deserialize_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 100)]
        #[serde(rename(deserialize = "value"))]
        val: i32,
    }

    let s = TestStruct::from_json_value(json!({ "value": 123 }));

    assert!(s.is_ok())
}

#[test]
fn serde_rename_deserialize_is_err() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(maximum = 100)]
        #[serde(rename(deserialize = "value"))]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "value": 123 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": ["The number must be `<= 100`."]
                }
            }
        })
    );
}

#[test]
fn serde_rename_enume_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    enum TestEnum {
        Struct {
            #[validate(minimum = 100)]
            #[serde(rename = "value")]
            val: i32,
        },
    }

    let s = TestEnum::from_json_value(json!({ "Struct": { "value": 123 } }));

    assert!(s.is_ok())
}

#[test]
fn serde_rename_enume_is_err() {
    #[derive(Debug, Validate, Deserialize)]
    enum TestEnum {
        Struct {
            #[validate(maximum = 100)]
            #[serde(rename = "value")]
            val: i32,
        },
    }

    let err = TestEnum::from_json_value(json!({ "Struct": { "value": 123 } })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": ["The number must be `<= 100`."]
                }
            }
        })
    );
}

#[test]
fn serde_with_does_not_rename() {
    mod seconds {
        use serde::{Deserialize, Deserializer};

        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
            u64::deserialize(d)
        }
    }

    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[serde(with = "seconds")]
        #[validate(maximum = 10)]
        first: u64,

        #[serde(with = "seconds")]
        #[validate(maximum = 10)]
        second: u64,
    }

    let err = TestStruct::from_json_value(json!({ "first": 11, "second": 12 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "first": {
                    "errors": ["The number must be `<= 10`."]
                },
                "second": {
                    "errors": ["The number must be `<= 10`."]
                }
            }
        })
    );
}

#[test]
fn serde_deserialize_with_does_not_rename() {
    fn deserialize_u64<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        serde::Deserialize::deserialize(d)
    }

    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[serde(deserialize_with = "deserialize_u64")]
        #[validate(maximum = 10)]
        val: u64,
    }

    let err = TestStruct::from_json_value(json!({ "val": 11 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["The number must be `<= 10`."]
                }
            }
        })
    );
}

#[test]
fn serde_default_does_not_rename() {
    fn default_count() -> u64 {
        0
    }

    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[serde(default = "default_count")]
        #[validate(maximum = 10)]
        count: u64,
    }

    let err = TestStruct::from_json_value(json!({ "count": 11 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "count": {
                    "errors": ["The number must be `<= 10`."]
                }
            }
        })
    );
}

#[test]
fn serde_rename_with_other_attributes_is_applied() {
    fn default_count() -> u64 {
        0
    }

    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[serde(default = "default_count", rename = "value")]
        #[validate(maximum = 10)]
        count: u64,
    }

    let err = TestStruct::from_json_value(json!({ "value": 11 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": ["The number must be `<= 10`."]
                }
            }
        })
    );
}
