use serde_json::json;
use serde_valid::Validate;

#[test]
fn scalar_string_numeric_enum_and_properties() {
    #[derive(Validate)]
    struct Cases<'a> {
        #[validate(min_length = 1)]
        text: &'a str,
        #[validate(minimum = 1)]
        number: i32,
        #[validate(r#enum = [1, 2, 3])]
        choice: i32,
        #[validate(min_properties = 1)]
        object: std::collections::HashMap<&'a str, u8>,
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                text: "",
                number: 0,
                choice: 9,
                object: std::collections::HashMap::new(),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "text": {
                    "errors": ["The length of the value must be `>= 1`."]
                },
                "number": {
                    "errors": ["The number must be `>= 1`."]
                },
                "choice": {
                    "errors": ["The value must be in [1, 2, 3]."]
                },
                "object": {
                    "errors": ["The size of the properties must be `>= 1`."]
                }
            }
        })
    );
}

#[test]
fn scalar_capability_and_transparent_cover_boxed_length_and_numeric() {
    use serde_valid::traits::Numeric;

    struct Count(i32);

    impl Numeric for Count {
        type Value = i32;

        fn numeric(&self) -> Self::Value {
            self.0
        }
    }

    #[derive(Validate)]
    struct Cases {
        #[validate(min_length = 1)]
        boxed_str: Box<str>,
        #[validate(minimum = 1)]
        boxed_count: Box<Count>,
        #[validate(minimum = 1)]
        rc_count: std::rc::Rc<Count>,
    }

    assert_eq!(
        serde_json::to_value(
            Cases {
                boxed_str: "".into(),
                boxed_count: Box::new(Count(0)),
                rc_count: std::rc::Rc::new(Count(0)),
            }
            .validate()
            .unwrap_err()
        )
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "boxed_str": {
                    "errors": ["The length of the value must be `>= 1`."]
                },
                "boxed_count": {
                    "errors": ["The number must be `>= 1`."]
                },
                "rc_count": {
                    "errors": ["The number must be `>= 1`."]
                }
            }
        })
    );
}

#[test]
fn scalar_custom_validate_minimum_works_without_a_wrapper() {
    use serde_valid::{MinimumError, ValidateMinimum};

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

    #[derive(Validate)]
    struct Cases {
        #[validate(minimum = 1)]
        value: CustomNumber,
    }

    assert_eq!(
        serde_json::to_value(Cases { value: CustomNumber(0) }.validate().unwrap_err()).unwrap(),
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
