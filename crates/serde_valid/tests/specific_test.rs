use serde_json::json;
use serde_valid::Validate;

#[test]
fn test_raw_type_field() {
    #[derive(Validate)]
    #[validate(custom = |s| sample_rule(s.r#type))]
    struct MyStruct {
        #[validate(maximum = 10)]
        pub r#type: i32,
    }

    fn sample_rule(_type: i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    let my_struct = MyStruct { r#type: 1 };
    assert!(my_struct.validate().is_ok());

    let my_struct = MyStruct { r#type: 11 };
    assert_eq!(
        serde_json::to_value(my_struct.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "type": { "errors": ["The number must be `<= 10`."] }
            }
        })
    );
}
