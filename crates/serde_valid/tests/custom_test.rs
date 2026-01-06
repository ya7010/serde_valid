use serde_json::json;
use serde_valid::Validate;

mod parenthesized {
    pub fn meta_path_validation(_val: &[i32]) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }
}

#[test]
fn custom_validation_is_ok() {
    fn user_validation(_val: &i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = user_validation)]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_parenthesized_path_validation_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = parenthesized::meta_path_validation)]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_clouser_validation_is_ok() {
    fn user_validation(_val: &[i32], _maximum: i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = |x| user_validation(x, 10))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_clouser_validation_is_err() {
    fn user_validation(val: &[i32], maximum: i32) -> Result<(), serde_valid::validation::Error> {
        if val.iter().all(|v| v <= &maximum) {
            Ok(())
        } else {
            Err(serde_valid::validation::Error::Custom(
                "this is custom message.".to_string(),
            ))
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = |x| user_validation(x, 10))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 11],
    };
    assert!(s.validate().is_err());
}

#[test]
fn custom_validation_error() {
    fn user_validation(_val: &[i32]) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "this is custom message.".to_string(),
        ))
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom = user_validation)]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
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
fn named_struct_custom_is_ok() {
    fn sample_struct_validation(_val: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom = sample_struct_validation)]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(s.val, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn named_struct_custom_closure_is_ok() {
    fn sample_struct_validation(_val: i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom = |s| sample_struct_validation(s.val))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(s.val, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn unnamed_struct_custom_is_ok() {
    fn sample_struct_validation(_val: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom = sample_struct_validation)]
    struct TestStruct(i32);

    let s = TestStruct(5);
    assert_eq!(s.0, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn unnamed_struct_custom_closure_is_ok() {
    fn sample_struct_validation(_val: i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom = |s| sample_struct_validation(s.0))]
    struct TestStruct(i32);

    let s = TestStruct(5);
    assert_eq!(s.0, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn unnamed_struct_custom_closure_is_err() {
    fn sample_struct_validation(_val: i32) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "Struct Validation Error.".to_owned(),
        ))
    }

    #[derive(Validate)]
    #[validate(custom = |s| sample_struct_validation(s.0))]
    struct TestStruct(i32);

    let s = TestStruct(5);
    assert_eq!(s.0, 5);
    assert!(s.validate().is_err());
}

#[test]
fn named_struct_custom_vec_errors_is_ok() {
    fn validation(_val: &TestStruct) -> Result<(), Vec<serde_valid::validation::Error>> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom = validation)]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(s.val, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn named_struct_custom_vec_errors_is_err() {
    fn validation(_val: &TestStruct) -> Result<(), Vec<serde_valid::validation::Error>> {
        Err(vec![
            serde_valid::validation::Error::Custom("Error 1".to_owned()),
            serde_valid::validation::Error::Custom("Error 2".to_owned()),
        ])
    }

    #[derive(Validate)]
    #[validate(custom = validation)]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };

    assert_eq!(s.val, 5);
    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": ["Error 1", "Error 2"],
            "properties": {}
        })
        .to_string()
    );
}

#[test]
fn named_struct_custom_closure_vec_errors_is_ok() {
    fn sample_struct_validation(_val: i32) -> Result<(), Vec<serde_valid::validation::Error>> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom = |s| sample_struct_validation(s.val))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(s.val, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn named_struct_custom_closure_vec_errors_is_err() {
    fn sample_struct_validation(_val: i32) -> Result<(), Vec<serde_valid::validation::Error>> {
        Err(vec![
            serde_valid::validation::Error::Custom("Error 1".to_owned()),
            serde_valid::validation::Error::Custom("Error 2".to_owned()),
        ])
    }

    #[derive(Validate)]
    #[validate(custom = |s| sample_struct_validation(s.val))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(s.val, 5);
    assert_eq!(
        s.validate().unwrap_err().to_string(),
        json!({
            "errors": ["Error 1", "Error 2"],
            "properties": {}
        })
        .to_string()
    );
}

#[test]
fn filed_custom_validation_using_self() {
    fn food_validation(kind: &str, food: &str) -> Result<(), serde_valid::validation::Error> {
        match kind {
            "Cat" => {
                if food == "CatFood" {
                    Ok(())
                } else {
                    Err(serde_valid::validation::Error::Custom(
                        "Cat should eat CatFood.".to_string(),
                    ))
                }
            }
            "Dog" => {
                if food == "DogFood" {
                    Ok(())
                } else {
                    Err(serde_valid::validation::Error::Custom(
                        "Dog should eat DogFood.".to_string(),
                    ))
                }
            }
            _ => Ok(()),
        }
    }

    #[derive(Validate)]
    struct Pet {
        #[validate(r#enum = ["Cat", "Dog"])]
        kind: String,

        #[validate(custom = |food| food_validation(&self.kind, food))]
        food: String,
    }

    let invalid = Pet {
        kind: "Cat".to_string(),
        food: "DogFood".to_string(),
    };

    assert_eq!(
        invalid.validate().unwrap_err().to_string(),
        json!({
            "errors": [],
            "properties": {
                "food": {
                    "errors": [
                        "Cat should eat CatFood."
                    ]
                }
            }
        })
        .to_string()
    );
}
