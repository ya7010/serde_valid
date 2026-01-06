#![allow(deprecated)]

use serde_json::json;
use serde_valid::{Validate, ValidateEnum};

#[test]
fn enumerate_integer_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [1, 2, 3])]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_float_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [0.3, 1.2, 1.5])]
        val: f32,
    }

    let s = TestStruct { val: 0.3 };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_str_type() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(enumerate = ["a", "b"])]
        val: &'a str,
    }

    let s = TestStruct { val: "a" };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = ["a", "b"])]
        val: String,
    }

    let s = TestStruct {
        val: "a".to_string(),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_vec_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [1, 2, 3, 4, 5])]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_vec_str_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = ["1", "2", "3", "4", "5"])]
        val: Vec<&'static str>,
    }

    let s = TestStruct {
        val: vec!["3", "4"],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_vec_string_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = ["1", "2", "3", "4", "5"])]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec!["3".to_owned(), "4".to_owned()],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_option_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [1, 2, 3])]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(3) };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_vec_option_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [3])]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct { val: vec![Some(3)] };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [0.3, 1.2, 1.5])]
        val: f32,
    }

    let s = TestStruct { val: 0.1 };
    assert!(s.validate().is_err());
}

#[test]
fn enumerate_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [1, 2, 3])]
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
                        "The value must be in [1, 2, 3]."
                    ]
                }
            }
        })
        .to_string()
    );
}

#[test]
fn enumerate_custom_err_message_fn() {
    fn error_message(_params: &serde_valid::error::EnumError) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [1, 2, 3], message_fn = error_message)]
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
                    "this is custom message."
                    ]
                }
            }
        })
        .to_string()
    );
}

#[test]
fn enumerate_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [1, 2, 3], message = "this is custom message.")]
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
                    "this is custom message."
                    ]
                }
            }
        })
        .to_string()
    );
}

#[test]
fn enumerate_numeric_trait() {
    struct MyType(i32);

    impl PartialEq<i32> for MyType {
        fn eq(&self, other: &i32) -> bool {
            self.0.eq(other)
        }
    }

    impl ValidateEnum<i32> for MyType {
        fn validate_enum(&self, items: &[i32]) -> Result<(), serde_valid::EnumError> {
            self.0.validate_enum(items)
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate = [1, 2, 3], message = "this is custom message.")]
        val: MyType,
    }

    let s = TestStruct { val: MyType(4) };

    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                    "this is custom message."
                    ]
                }
            }
        })
        .to_string()
    );
}
