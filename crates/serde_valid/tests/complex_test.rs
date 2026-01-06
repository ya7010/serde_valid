use serde_valid::Validate;

fn sample_rule(_val: i32) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

#[derive(Debug, Validate)]
#[validate(custom = |s| sample_rule(s.int_value))]
struct TestStruct<'a> {
    // Generic validator
    #[validate(r#enum = [5, 10, 15])]
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 5)]
    #[validate(exclusive_minimum = 4)]
    #[validate(exclusive_maximum = 6)]
    int_value: i32,

    // Generic validator
    #[validate(r#enum = [5.0, 10.0, 15.0])]
    // Numeric validator
    #[validate(multiple_of = 5.0)]
    #[validate(minimum = 5.0)]
    #[validate(maximum = 5.0)]
    #[validate(exclusive_minimum = 4.0)]
    #[validate(exclusive_maximum = 6.0)]
    float_value: f32,

    // Generic validator
    #[validate(r#enum = ["12345", "67890"])]
    // String validator
    #[validate(min_length = 5)]
    #[validate(max_length = 5)]
    #[validate(pattern = r"^\d{5}$")]
    string_value: String,

    // Generic validator
    #[validate(r#enum = ["12345", "67890"])]
    // String validator
    #[validate(min_length = 5)]
    #[validate(max_length = 5)]
    #[validate(pattern = r"^\d{5}$")]
    str_value: &'a str,

    // Generic validator
    #[validate(r#enum = [5, 10, 15])]
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 5)]
    optional_value: Option<i32>,

    // Generic validator
    #[validate(r#enum = [5, 10, 15])]
    // Array validator
    #[validate(unique_items)]
    #[validate(min_items = 3)]
    #[validate(max_items = 3)]
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 15)]
    vec_value: Vec<i32>,

    // Nested validator
    #[validate]
    nested_struct: TestInnerStruct<'a>,

    // Nested vec validator
    #[validate]
    // Array validator
    #[validate(min_items = 1)]
    #[validate(max_items = 1)]
    nested_vec_struct: Vec<TestInnerStruct<'a>>,
}

#[derive(Debug, Validate)]
struct TestInnerStruct<'a> {
    // Generic validator
    #[validate(r#enum = [5, 10, 15])]
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 5)]
    #[validate(exclusive_minimum = 4)]
    #[validate(exclusive_maximum = 6)]
    inner_int_value: i32,

    // Generic validator
    #[validate(r#enum = [5.0, 10.0, 15.0])]
    // Numeric validator
    #[validate(multiple_of = 5.0)]
    #[validate(minimum = 5.0)]
    #[validate(maximum = 5.0)]
    #[validate(exclusive_minimum = 4.0)]
    #[validate(exclusive_maximum = 6.0)]
    inner_float_value: f32,

    // Generic validator
    #[validate(r#enum = ["12345", "67890"])]
    // String validator
    #[validate(min_length = 5)]
    #[validate(max_length = 5)]
    #[validate(pattern = r"^\d{5}$")]
    inner_string_value: String,

    // Generic validator
    #[validate(r#enum = ["12345", "67890"])]
    // String validator
    #[validate(min_length = 5)]
    #[validate(max_length = 5)]
    #[validate(pattern = r"^\d{5}$")]
    inner_str_value: &'a str,

    // Generic validator
    #[validate(r#enum = [5, 10, 15])]
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 5)]
    inner_optional_value: Option<i32>,

    // Generic validator
    #[validate(r#enum = [5, 10, 15])]
    // Array validator
    #[validate(unique_items)]
    #[validate(min_items = 3)]
    #[validate(max_items = 3)]
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 15)]
    inner_vec_value: Vec<i32>,
}

#[test]
fn complex_validation() {
    let s = TestStruct {
        int_value: 5,
        float_value: 5.0,
        string_value: "12345".to_string(),
        str_value: "12345",
        optional_value: Some(5),
        vec_value: vec![5, 10, 15],
        nested_struct: TestInnerStruct {
            inner_int_value: 5,
            inner_float_value: 5.0,
            inner_string_value: "12345".to_string(),
            inner_str_value: "12345",
            inner_optional_value: Some(5),
            inner_vec_value: vec![5, 10, 15],
        },
        nested_vec_struct: vec![TestInnerStruct {
            inner_int_value: 5,
            inner_float_value: 5.0,
            inner_string_value: "12345".to_string(),
            inner_str_value: "12345",
            inner_optional_value: Some(5),
            inner_vec_value: vec![5, 10, 15],
        }],
    };
    assert!(s.validate().is_ok());
}
