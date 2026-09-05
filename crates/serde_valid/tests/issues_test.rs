use serde_valid::Validate;

mod issue54 {
    use super::*;

    #[test]
    fn test_enum_valians_works() {
        #[derive(Validate)]
        enum Works {
            VariantB(),
            VariantA,
        }

        assert!(Works::VariantA.validate().is_ok());
        assert!(Works::VariantB().validate().is_ok());
    }

    #[test]
    fn test_enum_valiant_fied_case() {
        #[derive(Validate)]
        enum Fails {
            VariantA,
            VariantB(),
        }

        assert!(Fails::VariantA.validate().is_ok());
        assert!(Fails::VariantB().validate().is_ok());
    }
}

mod issue107 {
    use serde::{Deserialize, Serialize};
    use serde_valid::Validate;
    use std::collections::HashSet;

    // Test case 1: Named fields with some fields having validation and others not
    #[allow(unused_variables)]
    #[derive(Debug, PartialEq, Deserialize, Serialize, Validate)]
    #[serde(untagged)]
    pub enum WhiteList {
        List {
            #[validate(min_length = 1)]
            white_type: String,
            #[allow(unused_variables)]
            list: HashSet<String>,
        },
    }

    #[test]
    fn test_issue_107_named_fields() {
        let white_list = WhiteList::List {
            white_type: "ip".to_string(),
            list: vec!["127.0.0.1".to_string(), "192.168.1.1".to_string()]
                .into_iter()
                .collect(),
        };

        assert!(white_list.validate().is_ok());
    }

    #[test]
    fn test_issue_107_named_fields_validation_error() {
        let white_list = WhiteList::List {
            white_type: "".to_string(),
            list: vec!["127.0.0.1".to_string()].into_iter().collect(),
        };

        assert!(white_list.validate().is_err());
    }

    // Test case 2: Unnamed fields with some fields having validation and others not
    #[derive(Debug, PartialEq, Deserialize, Serialize, Validate)]
    #[serde(untagged)]
    pub enum DataEnum {
        Unnamed(#[validate(minimum = 0)] i32, String),
    }

    #[test]
    fn test_issue_107_unnamed_fields() {
        let data = DataEnum::Unnamed(5, "test".to_string());
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_issue_107_unnamed_fields_validation_error() {
        let data = DataEnum::Unnamed(-1, "test".to_string());
        assert!(data.validate().is_err());
    }

    // Test case 3: Multiple variants with mixed validation scenarios
    #[derive(Debug, PartialEq, Deserialize, Serialize, Validate)]
    pub enum MultiVariant {
        Variant1 {
            #[validate(maximum = 100)]
            validated_field: i32,
            unvalidated_field: String,
        },
        Variant2 {
            field1: String,
            #[validate(minimum = 0)]
            field2: i32,
            field3: bool,
        },
        Variant3(#[validate(min_length = 1)] String, i32, bool),
    }

    #[test]
    fn test_issue_107_multi_variant() {
        let v1 = MultiVariant::Variant1 {
            validated_field: 50,
            unvalidated_field: "test".to_string(),
        };
        assert!(v1.validate().is_ok());

        let v2 = MultiVariant::Variant2 {
            field1: "test".to_string(),
            field2: 10,
            field3: true,
        };
        assert!(v2.validate().is_ok());

        let v3 = MultiVariant::Variant3("test".to_string(), 10, false);
        assert!(v3.validate().is_ok());
    }

    #[test]
    fn test_issue_107_multi_variant_validation_error() {
        let v1 = MultiVariant::Variant1 {
            validated_field: 150,
            unvalidated_field: "test".to_string(),
        };
        assert!(v1.validate().is_err());

        let v2 = MultiVariant::Variant2 {
            field1: "test".to_string(),
            field2: -10,
            field3: true,
        };
        assert!(v2.validate().is_err());

        let v3 = MultiVariant::Variant3("".to_string(), 10, false);
        assert!(v3.validate().is_err());
    }
}

mod issue125 {
    use serde::Deserialize;
    use serde_valid::Validate;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, Validate)]
    struct Child {
        #[validate(minimum = 1)]
        value: i32,
    }

    #[derive(Debug, Deserialize, Validate)]
    struct BorrowedNames<'a> {
        #[validate(min_items = 4)]
        #[validate(max_items = 2)]
        #[validate(min_length = 1)]
        #[validate(unique_items)]
        #[serde(borrow)]
        names: Box<[&'a str]>,
    }

    #[test]
    fn boxed_slice_supports_array_and_composited_validators() {
        let value = BorrowedNames {
            names: vec!["", "duplicate", "duplicate"].into_boxed_slice(),
        };

        let errors = value.validate().unwrap_err().to_string();
        assert!(errors.contains("The length of the items must be `>= 4`."));
        assert!(errors.contains("The length of the items must be `<= 2`."));
        assert!(errors.contains("The length of the value must be `>= 1`."));
        assert!(errors.contains("The items must be unique."));
    }

    #[test]
    fn boxed_slice_deserializes_borrowed_strings() {
        let json = r#"{"names":["borrowed"]}"#;
        let value: BorrowedNames<'_> = serde_json::from_str(json).unwrap();
        assert_eq!(value.names.as_ref(), ["borrowed"]);
    }

    #[derive(Debug, Deserialize, Validate)]
    struct BorrowedSlice<'a> {
        #[validate(min_items = 2)]
        #[validate(max_items = 2)]
        #[validate(unique_items)]
        #[serde(borrow)]
        data: &'a [u8],
    }

    #[test]
    fn borrowed_slice_supports_array_validators() {
        assert!(BorrowedSlice { data: &[1, 2] }.validate().is_ok());
        assert!(BorrowedSlice { data: &[1] }.validate().is_err());
        assert!(BorrowedSlice { data: &[1, 2, 3] }.validate().is_err());
        assert!(BorrowedSlice { data: &[1, 1] }.validate().is_err());
    }

    #[derive(Debug, Validate)]
    struct BoxedString {
        #[validate(min_length = 1)]
        #[validate(max_length = 2)]
        value: Box<str>,
    }

    #[test]
    fn boxed_string_supports_length_validators() {
        assert!(BoxedString { value: "x".into() }.validate().is_ok());
        assert!(BoxedString { value: "".into() }.validate().is_err());
        assert!(BoxedString {
            value: "too long".into(),
        }
        .validate()
        .is_err());
    }

    #[derive(Debug, Validate)]
    struct BoxedChildren {
        #[validate]
        children: Box<[Child]>,
    }

    #[derive(Debug, Validate)]
    struct BorrowedChildren<'a> {
        #[validate]
        children: &'a [Child],
    }

    #[test]
    fn boxed_and_borrowed_slices_support_nested_validation() {
        let children = vec![Child { value: 0 }].into_boxed_slice();
        assert!(BoxedChildren { children }.validate().is_err());

        let children = [Child { value: 0 }];
        assert!(BorrowedChildren {
            children: &children,
        }
        .validate()
        .is_err());
    }

    #[derive(Debug, Validate)]
    struct BoxedNumbers {
        #[validate(minimum = 1)]
        numbers: Box<[i32]>,
    }

    #[derive(Debug, Validate)]
    struct BorrowedNumbers<'a> {
        #[validate(minimum = 1)]
        numbers: &'a [i32],
    }

    #[test]
    fn boxed_slice_supports_numeric_composited_validation() {
        assert!(BoxedNumbers {
            numbers: vec![1, 2].into_boxed_slice(),
        }
        .validate()
        .is_ok());
        assert!(BoxedNumbers {
            numbers: vec![0, 2].into_boxed_slice(),
        }
        .validate()
        .is_err());

        assert!(BorrowedNumbers { numbers: &[1, 2] }.validate().is_ok());
        assert!(BorrowedNumbers { numbers: &[0, 2] }.validate().is_err());
    }

    #[test]
    fn existing_map_key_contract_remains_supported() {
        #[derive(Eq, Hash, PartialEq)]
        struct CustomKey(String);

        impl From<&CustomKey> for String {
            fn from(key: &CustomKey) -> Self {
                key.0.clone()
            }
        }

        let string_keys = HashMap::from([("string".to_owned(), Child { value: 0 })]);
        assert!(string_keys.validate().is_err());

        let custom_keys = HashMap::from([(CustomKey("custom".to_owned()), Child { value: 0 })]);
        assert!(custom_keys.validate().is_err());
    }
}
