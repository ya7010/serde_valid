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
    use serde_valid::{EnumError, Validate, ValidateEnum};
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
    struct BoxedStringConstraints {
        #[validate(pattern = "^[a-z]+$")]
        #[validate(r#enum = ["alpha", "beta"])]
        value: Box<str>,
    }

    #[test]
    fn boxed_string_supports_pattern_and_enum_validators() {
        assert!(BoxedStringConstraints {
            value: "alpha".into(),
        }
        .validate()
        .is_ok());
        assert!(BoxedStringConstraints {
            value: "123".into(),
        }
        .validate()
        .is_err());
        assert!(BoxedStringConstraints {
            value: "gamma".into(),
        }
        .validate()
        .is_err());
    }

    #[derive(Debug, Validate)]
    struct BoxedIntegerConstraint {
        #[validate(r#enum = [1, 2])]
        value: Box<i32>,
    }

    #[derive(Debug)]
    struct CustomEnumValue(String);

    impl ValidateEnum<&'static str> for CustomEnumValue {
        fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
            self.0.validate_enum(candidates)
        }
    }

    #[derive(Debug, Validate)]
    struct BoxedCustomEnumConstraint {
        #[validate(r#enum = ["alpha", "beta"])]
        value: Box<CustomEnumValue>,
    }

    #[derive(Debug, Validate)]
    struct BoxedOsStrConstraint {
        #[validate(r#enum = ["alpha", "beta"])]
        value: Box<std::ffi::OsStr>,
    }

    #[derive(Debug, Validate)]
    struct BoxedPathConstraint {
        #[validate(r#enum = ["alpha", "beta"])]
        value: Box<std::path::Path>,
    }

    #[test]
    fn boxed_values_delegate_enum_validation_to_their_inner_type() {
        assert!(BoxedIntegerConstraint { value: Box::new(1) }
            .validate()
            .is_ok());
        assert!(BoxedIntegerConstraint { value: Box::new(3) }
            .validate()
            .is_err());

        assert!(BoxedCustomEnumConstraint {
            value: Box::new(CustomEnumValue("alpha".to_owned())),
        }
        .validate()
        .is_ok());
        assert!(BoxedCustomEnumConstraint {
            value: Box::new(CustomEnumValue("gamma".to_owned())),
        }
        .validate()
        .is_err());

        assert!(BoxedOsStrConstraint {
            value: std::ffi::OsString::from("alpha").into_boxed_os_str(),
        }
        .validate()
        .is_ok());
        assert!(BoxedPathConstraint {
            value: std::path::PathBuf::from("alpha").into_boxed_path(),
        }
        .validate()
        .is_ok());
    }

    #[allow(deprecated)]
    mod deprecated_enumerate {
        use super::*;

        #[derive(Debug, Validate)]
        struct DeprecatedBoxedStringConstraint {
            #[validate(enumerate = ["alpha", "beta"])]
            value: Box<str>,
        }

        #[test]
        fn boxed_string_remains_supported_by_deprecated_enumerate_validator() {
            assert!(DeprecatedBoxedStringConstraint {
                value: "alpha".into(),
            }
            .validate()
            .is_ok());
            assert!(DeprecatedBoxedStringConstraint {
                value: "gamma".into(),
            }
            .validate()
            .is_err());
        }
    }

    #[derive(Debug, Validate)]
    struct BoxedArray {
        #[validate(min_items = 4)]
        #[validate(max_items = 2)]
        #[validate(unique_items)]
        #[validate(minimum = 1)]
        values: Box<[i32; 3]>,
    }

    #[derive(Debug, Validate)]
    struct BorrowedArray<'a> {
        #[validate(min_items = 4)]
        #[validate(max_items = 2)]
        #[validate(unique_items)]
        #[validate(minimum = 1)]
        values: &'a [i32; 3],
    }

    #[test]
    fn boxed_and_borrowed_arrays_support_array_and_composited_validators() {
        let errors = BoxedArray {
            values: Box::new([0, 1, 1]),
        }
        .validate()
        .unwrap_err()
        .to_string();
        assert!(errors.contains("The length of the items must be `>= 4`."));
        assert!(errors.contains("The length of the items must be `<= 2`."));
        assert!(errors.contains("The items must be unique."));
        assert!(errors.contains("The number must be `>= 1`."));

        let values = [0, 1, 1];
        let errors = BorrowedArray { values: &values }
            .validate()
            .unwrap_err()
            .to_string();
        assert!(errors.contains("The length of the items must be `>= 4`."));
        assert!(errors.contains("The length of the items must be `<= 2`."));
        assert!(errors.contains("The items must be unique."));
        assert!(errors.contains("The number must be `>= 1`."));
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
    struct OptionalChildren<'a> {
        #[validate]
        boxed_slice: Option<Box<[Child]>>,
        #[validate]
        borrowed_slice: Option<&'a [Child]>,
        #[validate]
        boxed_array: Option<Box<[Child; 1]>>,
        #[validate]
        borrowed_array: Option<&'a [Child; 1]>,
    }

    #[test]
    fn optional_boxed_and_borrowed_containers_support_nested_validation() {
        let children = [Child { value: 0 }];
        let value = OptionalChildren {
            boxed_slice: Some(vec![Child { value: 0 }].into_boxed_slice()),
            borrowed_slice: Some(&children),
            boxed_array: Some(Box::new([Child { value: 0 }])),
            borrowed_array: Some(&children),
        };
        let errors = value.validate().unwrap_err().to_string();
        assert!(errors.contains("boxed_slice"));
        assert!(errors.contains("borrowed_slice"));
        assert!(errors.contains("boxed_array"));
        assert!(errors.contains("borrowed_array"));

        assert!(OptionalChildren {
            boxed_slice: None,
            borrowed_slice: None,
            boxed_array: None,
            borrowed_array: None,
        }
        .validate()
        .is_ok());
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
