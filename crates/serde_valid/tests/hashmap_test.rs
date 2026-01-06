use regex::Regex;
use serde_json::json;
use serde_valid::Validate;
use std::collections::HashMap;

#[derive(Debug, Validate)]
struct TestStruct {
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 20)]
    #[validate(exclusive_minimum = 4)]
    #[validate(exclusive_maximum = 21)]
    // Checks hashmap not number
    #[validate(max_properties = 2)]
    #[validate(min_properties = 2)]
    hashmap_of_hashmap: HashMap<&'static str, HashMap<String, i32>>,

    // Generic validator
    #[validate(r#enum = [5, 10, 15])]
    // Numeric validator
    #[validate(multiple_of = 5)]
    #[validate(minimum = 5)]
    #[validate(maximum = 10)]
    #[validate(exclusive_minimum = 4)]
    #[validate(exclusive_maximum = 11)]
    // Checks hashmap not number
    #[validate(max_properties = 2)]
    #[validate(min_properties = 2)]
    hashmap_of_numbers: HashMap<String, i32>,

    #[validate(pattern = "d.*")]
    #[validate(max_length = 5)]
    #[validate(min_length = 5)]
    // Checks hashmap not string
    #[validate(max_properties = 2)]
    #[validate(min_properties = 2)]
    hashmap_of_strings: HashMap<String, String>,
}

#[derive(Debug, Validate)]
struct DeriveValidateHashmap {
    #[validate]
    hashmap_of_object: HashMap<String, Object>,
}

#[derive(Debug, Validate)]
struct Object {
    #[validate(maximum = 5)]
    number: i32,
}

#[test]
fn hashmap_validation() {
    // Create basic valid hashmaps.
    let mut hashmap_of_numbers = HashMap::from([("five".to_string(), 5), ("ten".to_string(), 10)]);
    let mut hashmap_of_strings = HashMap::from([
        ("one".to_string(), "ddddd".to_string()),
        ("two".to_string(), "ddddd".to_string()),
    ]);
    let mut hashmap_of_numbers_2 =
        HashMap::from([("five".to_string(), 5), ("ten".to_string(), 10)]);
    let mut hashmap_of_hashmap = HashMap::from([
        ("H_One", hashmap_of_numbers.clone()),
        ("H_two", hashmap_of_numbers_2.clone()),
    ]);

    // Test valid set
    let test_struct = TestStruct {
        hashmap_of_hashmap: hashmap_of_hashmap.clone(),
        hashmap_of_numbers: hashmap_of_numbers.clone(),
        hashmap_of_strings: hashmap_of_strings.clone(),
    };
    assert!(test_struct.validate().is_ok());

    // Test invalid set
    hashmap_of_numbers.insert("twenty".to_string(), 20);
    hashmap_of_strings.insert("three".to_string(), "ff".to_string());
    hashmap_of_numbers_2.insert("nineteen".to_string(), 19);
    hashmap_of_hashmap.insert("H_three", hashmap_of_numbers_2);
    let test_struct2 = TestStruct {
        hashmap_of_hashmap,
        hashmap_of_numbers,
        hashmap_of_strings,
    };

    // Because hashmap indexing is non-deterministic,
    // we have to check for the individual errors returned.
    let errors = test_struct2.validate().unwrap_err().to_string();
    assert_eq!(
        // This should appear for all 3 hashmaps.
        Regex::new(r"The size of the properties must be `<= 2`\.")
            .unwrap()
            .find_iter(&errors)
            .count(),
        3
    );
    assert!(errors.contains("The value must be multiple of `5`."));
    assert!(errors.contains(
        &json!({"errors":[
            r#"The value must match the pattern of "d.*"."#,
            "The length of the value must be `>= 5`."
        ]})
        .to_string()
    ));
    assert!(errors.contains(
        &json!({"errors":[
            "The value must be in [5, 10, 15].",
            "The number must be `<= 10`.",
            "The number must be `< 11`."
        ]})
        .to_string()
    ));
}

#[test]
fn hashmap_object_validation() {
    let object = Object { number: 5 };
    let hashmap_of_object = HashMap::from([("object".to_string(), object)]);
    let test_struct = DeriveValidateHashmap { hashmap_of_object };
    assert!(test_struct.validate().is_ok());

    let object2 = Object { number: 6 };
    let hashmap_of_object2 = HashMap::from([("object".to_string(), object2)]);
    let test_struct2 = DeriveValidateHashmap {
        hashmap_of_object: hashmap_of_object2,
    };
    assert_eq!(
        test_struct2.validate().unwrap_err().to_string(),
        json!({"errors":[],
        "properties":{
            "hashmap_of_object":{
                "errors":[],
                "properties":{
                    "object":{
                        "errors":[],
                        "properties":{
                            "number":{
                                "errors":
                                ["The number must be `<= 5`."]
                                }
                            }
                        }
                    }
                }
            }
        })
        .to_string()
    );
}
