use std::time::Duration;

use serde_json::json;
use serde_valid::utils::{duration_maximum, duration_minimum};
use serde_valid::Validate;

#[test]
fn duration_maximum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = duration_maximum(Duration::from_micros(5)))]
        val: Duration,
    }

    let s = TestStruct {
        val: Duration::from_micros(5),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn duration_minimum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = duration_minimum(Duration::from_micros(5)))]
        val: Duration,
    }

    let s = TestStruct {
        val: Duration::from_secs(5),
    };

    assert!(s.validate().is_ok());
}

#[test]
fn duration_maximum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = duration_maximum(Duration::from_micros(5)))]
        val: Duration,
    }

    let s = TestStruct {
        val: Duration::from_micros(10),
    };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "Duration 10µs is greater than maximum 5µs."
                    ]
                }
            }
        })
    );
}

#[test]
fn duration_minimum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = duration_minimum(Duration::from_micros(5)))]
        val: Duration,
    }

    let s = TestStruct {
        val: Duration::from_micros(1),
    };

    assert_eq!(
        serde_json::to_value(s.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "Duration 1µs is less than minimum 5µs."
                    ]
                }
            }
        })
    );
}
