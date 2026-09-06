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
    use indexmap::IndexMap;
    use serde::Deserialize;
    use serde_json::json;
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
        #[validate(pattern = "^(alpha|gamma)$")]
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
        let pattern_errors = BoxedStringConstraints {
            value: "beta".into(),
        }
        .validate()
        .unwrap_err()
        .to_string();
        assert!(pattern_errors.contains("must match the pattern"));
        assert!(!pattern_errors.contains("must be in"));

        let enum_errors = BoxedStringConstraints {
            value: "gamma".into(),
        }
        .validate()
        .unwrap_err()
        .to_string();
        assert!(!enum_errors.contains("must match the pattern"));
        assert!(enum_errors.contains("must be in"));
    }

    #[derive(Debug, Validate)]
    struct BoxedIntegerConstraint {
        #[validate(r#enum = [1, 2])]
        value: Box<i32>,
    }

    #[derive(Clone, Debug)]
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

    #[derive(Debug, Validate)]
    struct CowOsStrConstraint<'a> {
        #[validate(r#enum = ["alpha", "beta"])]
        value: std::borrow::Cow<'a, std::ffi::OsStr>,
    }

    #[derive(Debug, Validate)]
    struct CowPathConstraint<'a> {
        #[validate(r#enum = ["alpha", "beta"])]
        value: std::borrow::Cow<'a, std::path::Path>,
    }

    #[derive(Debug, Validate)]
    struct CowCustomEnumConstraint<'a> {
        #[validate(r#enum = ["alpha", "beta"])]
        value: std::borrow::Cow<'a, CustomEnumValue>,
    }

    #[derive(Debug, Validate)]
    struct CowIntegerConstraint<'a> {
        #[validate(r#enum = [1, 2])]
        value: std::borrow::Cow<'a, i32>,
    }

    #[derive(Debug, Validate)]
    struct CowStringConstraint<'a> {
        #[validate(r#enum = ["alpha", "beta"])]
        value: std::borrow::Cow<'a, str>,
    }

    #[derive(Debug, Validate)]
    struct BorrowedCustomEnumConstraint<'a> {
        #[validate(r#enum = ["alpha", "beta"])]
        value: &'a CustomEnumValue,
    }

    #[derive(Debug, Validate)]
    struct BorrowedIntegerConstraint<'a> {
        #[validate(r#enum = [1, 2])]
        value: &'a i32,
    }

    #[derive(Debug, Validate)]
    #[allow(clippy::box_collection)]
    struct BoxedOuterContainerConstraints {
        #[validate(r#enum = [1, 2])]
        optional: Box<Option<i32>>,
        #[validate(minimum = 1)]
        values: Box<Vec<i32>>,
    }

    #[test]
    fn boxed_and_cow_values_delegate_enum_validation_to_their_inner_type() {
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
        assert!(CowStringConstraint {
            value: std::borrow::Cow::Borrowed("alpha"),
        }
        .validate()
        .is_ok());
        assert!(CowStringConstraint {
            value: std::borrow::Cow::Owned("gamma".to_owned()),
        }
        .validate()
        .is_err());
        assert!(CowOsStrConstraint {
            value: std::borrow::Cow::Borrowed(std::ffi::OsStr::new("alpha")),
        }
        .validate()
        .is_ok());
        assert!(CowOsStrConstraint {
            value: std::borrow::Cow::Borrowed(std::ffi::OsStr::new("gamma")),
        }
        .validate()
        .is_err());
        assert!(CowOsStrConstraint {
            value: std::borrow::Cow::Owned(std::ffi::OsString::from("alpha")),
        }
        .validate()
        .is_ok());
        assert!(CowPathConstraint {
            value: std::borrow::Cow::Borrowed(std::path::Path::new("alpha")),
        }
        .validate()
        .is_ok());
        assert!(CowPathConstraint {
            value: std::borrow::Cow::Borrowed(std::path::Path::new("gamma")),
        }
        .validate()
        .is_err());
        assert!(CowPathConstraint {
            value: std::borrow::Cow::Owned(std::path::PathBuf::from("alpha")),
        }
        .validate()
        .is_ok());
        assert!(CowCustomEnumConstraint {
            value: std::borrow::Cow::Owned(CustomEnumValue("alpha".to_owned())),
        }
        .validate()
        .is_ok());

        let custom = CustomEnumValue("alpha".to_owned());
        assert!(CowCustomEnumConstraint {
            value: std::borrow::Cow::Borrowed(&custom),
        }
        .validate()
        .is_ok());

        let errors = CowCustomEnumConstraint {
            value: std::borrow::Cow::Owned(CustomEnumValue("gamma".to_owned())),
        }
        .validate()
        .unwrap_err();
        assert_eq!(
            serde_json::to_value(errors).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": {
                        "errors": ["The value must be in [alpha, beta]."]
                    }
                }
            })
        );

        let one = 1;
        assert!(CowIntegerConstraint {
            value: std::borrow::Cow::Borrowed(&one),
        }
        .validate()
        .is_ok());
        assert!(CowIntegerConstraint {
            value: std::borrow::Cow::Owned(3),
        }
        .validate()
        .is_err());

        let custom = CustomEnumValue("alpha".to_owned());
        assert!(BorrowedCustomEnumConstraint { value: &custom }
            .validate()
            .is_ok());
        let integer = 3;
        assert!(BorrowedIntegerConstraint { value: &integer }
            .validate()
            .is_err());

        assert!(BoxedOuterContainerConstraints {
            optional: Box::new(Some(1)),
            values: Box::new(vec![1, 2]),
        }
        .validate()
        .is_ok());

        let errors = serde_json::to_value(
            BoxedOuterContainerConstraints {
                optional: Box::new(Some(3)),
                values: Box::new(vec![1, 2]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();
        assert!(errors["properties"].get("optional").is_some());
        assert!(errors["properties"].get("values").is_none());

        let errors = serde_json::to_value(
            BoxedOuterContainerConstraints {
                optional: Box::new(Some(1)),
                values: Box::new(vec![0, 2]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();
        assert!(errors["properties"].get("optional").is_none());
        assert!(errors["properties"].get("values").is_some());

        assert!(BoxedOuterContainerConstraints {
            optional: Box::new(Some(3)),
            values: Box::new(vec![0, 2]),
        }
        .validate()
        .is_err());
    }

    #[derive(Validate)]
    #[allow(clippy::box_collection)]
    struct StandardCompositedWrappers<'a> {
        #[validate(minimum = 1)]
        boxed_hash_map: Box<HashMap<String, i32>>,
        #[validate(min_length = 2)]
        boxed_index_map: Box<IndexMap<String, String>>,
        #[validate(minimum = 1)]
        borrowed_vec: &'a Vec<i32>,
        #[validate(minimum = 1)]
        borrowed_option: &'a Option<i32>,
        #[validate(r#enum = [1, 2])]
        cow_slice: std::borrow::Cow<'a, [i32]>,
    }

    #[test]
    fn standard_composited_wrappers_delegate_to_their_containers() {
        let valid_vec = vec![1, 2];
        let valid_option = Some(1);
        assert!(StandardCompositedWrappers {
            boxed_hash_map: Box::new(HashMap::from([("number".to_owned(), 1)])),
            boxed_index_map: Box::new(IndexMap::from([("string".to_owned(), "ok".to_owned(),)])),
            borrowed_vec: &valid_vec,
            borrowed_option: &valid_option,
            cow_slice: std::borrow::Cow::Borrowed(&[1, 2]),
        }
        .validate()
        .is_ok());

        let invalid_vec = vec![0];
        let invalid_option = Some(0);
        let errors = serde_json::to_value(
            StandardCompositedWrappers {
                boxed_hash_map: Box::new(HashMap::from([("number".to_owned(), 0)])),
                boxed_index_map: Box::new(IndexMap::from([("string".to_owned(), "x".to_owned())])),
                borrowed_vec: &invalid_vec,
                borrowed_option: &invalid_option,
                cow_slice: std::borrow::Cow::Owned(vec![3]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        for field in [
            "boxed_hash_map",
            "boxed_index_map",
            "borrowed_vec",
            "borrowed_option",
            "cow_slice",
        ] {
            assert!(errors["properties"].get(field).is_some(), "missing {field}");
        }
    }

    #[allow(deprecated)]
    mod deprecated_enumerate {
        use super::*;
        use serde_valid::validation::ValidateCompositedEnumerate;
        use serde_valid::ValidateEnumerate;

        #[derive(Debug, Validate)]
        struct DeprecatedBoxedStringConstraint {
            #[validate(enumerate = ["alpha", "beta"])]
            value: Box<str>,
        }

        #[derive(Clone, Debug)]
        struct DeprecatedCustomEnumValue(String);

        impl ValidateEnumerate<&'static str> for DeprecatedCustomEnumValue {
            fn validate_enumerate(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
                self.0.validate_enumerate(candidates)
            }
        }

        fn validate_enumerate<T, C>(value: &T, candidates: &[C]) -> Result<(), EnumError>
        where
            T: ValidateEnumerate<C>,
        {
            value.validate_enumerate(candidates)
        }

        fn validate_composited_enumerate<'a, T>(
            value: &T,
            candidates: &'a [i32],
        ) -> Result<(), serde_valid::validation::Composited<EnumError>>
        where
            T: ValidateCompositedEnumerate<&'a [i32]>,
        {
            value.validate_composited_enumerate(candidates)
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
            let custom = Box::new(DeprecatedCustomEnumValue("alpha".to_owned()));
            assert!(custom.validate_enumerate(&["alpha", "beta"]).is_ok());
            let custom: std::borrow::Cow<'_, DeprecatedCustomEnumValue> =
                std::borrow::Cow::Owned(DeprecatedCustomEnumValue("alpha".to_owned()));
            assert!(validate_enumerate(&custom, &["alpha", "beta"]).is_ok());
            let custom: std::borrow::Cow<'_, DeprecatedCustomEnumValue> =
                std::borrow::Cow::Owned(DeprecatedCustomEnumValue("gamma".to_owned()));
            assert!(validate_enumerate(&custom, &["alpha", "beta"]).is_err());
            let custom = DeprecatedCustomEnumValue("alpha".to_owned());
            let custom = std::borrow::Cow::Borrowed(&custom);
            assert!(validate_enumerate(&custom, &["alpha", "beta"]).is_ok());
            let custom = DeprecatedCustomEnumValue("gamma".to_owned());
            let custom = std::borrow::Cow::Borrowed(&custom);
            assert!(validate_enumerate(&custom, &["alpha", "beta"]).is_err());

            let integer: std::borrow::Cow<'_, i32> = std::borrow::Cow::Owned(1);
            assert!(validate_enumerate(&integer, &[1, 2]).is_ok());
            let integer: std::borrow::Cow<'_, i32> = std::borrow::Cow::Owned(3);
            assert!(validate_enumerate(&integer, &[1, 2]).is_err());

            let values = IndexMap::from([("value".to_owned(), 3)]);
            assert!(validate_composited_enumerate(&values, &[1, 2]).is_err());
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
        assert_eq!(
            serde_json::to_value(value.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "boxed_slice": {
                        "errors": [],
                        "items": {
                            "0": {
                                "errors": [],
                                "properties": {
                                    "value": { "errors": ["The number must be `>= 1`."] }
                                }
                            }
                        }
                    },
                    "borrowed_slice": {
                        "errors": [],
                        "items": {
                            "0": {
                                "errors": [],
                                "properties": {
                                    "value": { "errors": ["The number must be `>= 1`."] }
                                }
                            }
                        }
                    },
                    "boxed_array": {
                        "errors": [],
                        "items": {
                            "0": {
                                "errors": [],
                                "properties": {
                                    "value": { "errors": ["The number must be `>= 1`."] }
                                }
                            }
                        }
                    },
                    "borrowed_array": {
                        "errors": [],
                        "items": {
                            "0": {
                                "errors": [],
                                "properties": {
                                    "value": { "errors": ["The number must be `>= 1`."] }
                                }
                            }
                        }
                    }
                }
            })
        );

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

    #[derive(Debug, Validate)]
    struct IndexMapCompositedConstraints {
        #[validate(minimum = 1)]
        #[validate(maximum = 2)]
        numbers: IndexMap<String, i32>,
        #[validate(min_length = 2)]
        strings: IndexMap<String, String>,
        #[validate(r#enum = [1, 2])]
        enumerated: IndexMap<String, i32>,
        #[validate(minimum = 2)]
        #[validate(r#enum = [2])]
        merged: IndexMap<String, i32>,
    }

    #[test]
    fn index_map_supports_composited_validators() {
        assert!(IndexMapCompositedConstraints {
            numbers: IndexMap::from([("number".to_owned(), 1)]),
            strings: IndexMap::from([("string".to_owned(), "ok".to_owned())]),
            enumerated: IndexMap::from([("enumerated".to_owned(), 1)]),
            merged: IndexMap::from([("merged".to_owned(), 2)]),
        }
        .validate()
        .is_ok());

        let errors = serde_json::to_value(
            IndexMapCompositedConstraints {
                numbers: IndexMap::from([("low".to_owned(), 0), ("high".to_owned(), 3)]),
                strings: IndexMap::from([("string".to_owned(), "x".to_owned())]),
                enumerated: IndexMap::from([("enumerated".to_owned(), 3)]),
                merged: IndexMap::from([("merged".to_owned(), 1)]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();
        assert_eq!(
            errors,
            json!({
                "errors": [],
                "properties": {
                    "numbers": {
                        "errors": [],
                        "properties": {
                            "low": { "errors": ["The number must be `>= 1`."] },
                            "high": { "errors": ["The number must be `<= 2`."] }
                        }
                    },
                    "strings": {
                        "errors": [],
                        "properties": {
                            "string": { "errors": ["The length of the value must be `>= 2`."] }
                        }
                    },
                    "enumerated": {
                        "errors": [],
                        "properties": {
                            "enumerated": { "errors": ["The value must be in [1, 2]."] }
                        }
                    },
                    "merged": {
                        "errors": [],
                        "properties": {
                            "merged": {
                                "errors": [
                                    "The number must be `>= 2`.",
                                    "The value must be in [2]."
                                ]
                            }
                        }
                    }
                }
            })
        );
    }

    #[test]
    fn hash_map_composited_errors_preserve_keys_and_nested_items() {
        #[derive(Eq, Hash, PartialEq)]
        struct AliasedKey(u8);

        impl AsRef<str> for AliasedKey {
            fn as_ref(&self) -> &str {
                "same"
            }
        }

        #[derive(Validate)]
        struct MapConstraints {
            #[validate(minimum = 1)]
            numbers: HashMap<String, i32>,
            #[validate(minimum = 1)]
            nested: HashMap<String, Vec<i32>>,
            #[validate(minimum = 1)]
            nested_maps: HashMap<String, HashMap<String, i32>>,
            #[validate(minimum = 1)]
            maps_in_sequence: Vec<HashMap<String, i32>>,
            #[validate(minimum = 1)]
            aliased: HashMap<AliasedKey, i32>,
        }

        let errors = serde_json::to_value(
            MapConstraints {
                numbers: HashMap::from([("invalid".to_owned(), 0), ("valid".to_owned(), 1)]),
                nested: HashMap::from([("group".to_owned(), vec![0, 1])]),
                nested_maps: HashMap::from([(
                    "outer".to_owned(),
                    HashMap::from([("inner".to_owned(), 0)]),
                )]),
                maps_in_sequence: vec![HashMap::from([("inner".to_owned(), 0)])],
                aliased: HashMap::from([(AliasedKey(1), 0), (AliasedKey(2), 0)]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        assert_eq!(
            errors["properties"]["numbers"],
            json!({
                "errors": [],
                "properties": {
                    "invalid": { "errors": ["The number must be `>= 1`."] }
                }
            })
        );
        assert_eq!(
            errors["properties"]["nested"],
            json!({
                "errors": [],
                "properties": {
                    "group": {
                        "errors": [],
                        "items": {
                            "0": { "errors": ["The number must be `>= 1`."] }
                        }
                    }
                }
            })
        );
        assert_eq!(
            errors["properties"]["aliased"]["properties"]["same"]["errors"],
            json!(["The number must be `>= 1`.", "The number must be `>= 1`."])
        );
        assert_eq!(
            errors["properties"]["nested_maps"],
            json!({
                "errors": [],
                "properties": {
                    "outer": {
                        "errors": [],
                        "properties": {
                            "inner": { "errors": ["The number must be `>= 1`."] }
                        }
                    }
                }
            })
        );
        assert_eq!(
            errors["properties"]["maps_in_sequence"],
            json!({
                "errors": [],
                "items": {
                    "0": {
                        "errors": [],
                        "properties": {
                            "inner": { "errors": ["The number must be `>= 1`."] }
                        }
                    }
                }
            })
        );

        #[derive(Validate)]
        struct BorrowedMap<'a> {
            #[validate(minimum = 1)]
            numbers: HashMap<&'a str, i32>,
        }

        let key = String::from("borrowed");
        let errors = serde_json::to_value(
            BorrowedMap {
                numbers: HashMap::from([(key.as_str(), 0)]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();
        assert_eq!(
            errors["properties"]["numbers"]["properties"]["borrowed"]["errors"],
            json!(["The number must be `>= 1`."])
        );

        #[derive(Validate)]
        struct MapTuple(#[validate(minimum = 1)] HashMap<String, i32>);

        let errors = serde_json::to_value(
            MapTuple(HashMap::from([("tuple".to_owned(), 0)]))
                .validate()
                .unwrap_err(),
        )
        .unwrap();
        assert_eq!(
            errors["errors"][0]["properties"]["tuple"]["errors"],
            json!(["The number must be `>= 1`."])
        );
    }

    #[derive(Debug, Deserialize, Validate)]
    struct BorrowedMapKeys<'a> {
        #[validate]
        #[serde(borrow)]
        hash_map: HashMap<&'a str, Child>,
        #[validate]
        #[serde(borrow)]
        index_map: IndexMap<&'a str, Child>,
    }

    #[test]
    fn maps_deserialize_and_validate_borrowed_string_keys() {
        let value: BorrowedMapKeys<'_> = serde_json::from_str(
            r#"{
                "hash_map": { "hash": { "value": 0 } },
                "index_map": { "index": { "value": 0 } }
            }"#,
        )
        .unwrap();

        let errors = serde_json::to_value(value.validate().unwrap_err()).unwrap();
        assert_eq!(
            errors["properties"]["hash_map"]["properties"]["hash"]["properties"]["value"]["errors"],
            json!(["The number must be `>= 1`."])
        );
        assert_eq!(
            errors["properties"]["index_map"]["properties"]["index"]["properties"]["value"]
                ["errors"],
            json!(["The number must be `>= 1`."])
        );

        let json = String::from(
            r#"{
                "hash_map": { "hash": { "value": 1 } },
                "index_map": { "index": { "value": 1 } }
            }"#,
        );
        let value: BorrowedMapKeys<'_> = serde_json::from_str(&json).unwrap();
        assert!(value.validate().is_ok());
    }

    #[test]
    fn string_like_map_key_contract_supports_owned_and_borrowed_keys() {
        #[derive(Eq, Hash, PartialEq)]
        struct CustomKey(String);

        impl AsRef<str> for CustomKey {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        let string_keys = HashMap::from([("string".to_owned(), Child { value: 0 })]);
        assert!(string_keys.validate().is_err());

        let custom_keys = HashMap::from([(CustomKey("custom".to_owned()), Child { value: 0 })]);
        assert!(custom_keys.validate().is_err());

        let borrowed_keys = HashMap::from([("borrowed", Child { value: 0 })]);
        assert_eq!(
            serde_json::to_value(borrowed_keys.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "borrowed": {
                        "errors": [],
                        "properties": {
                            "value": { "errors": ["The number must be `>= 1`."] }
                        }
                    }
                }
            })
        );

        let borrowed_keys = IndexMap::from([("borrowed", Child { value: 0 })]);
        assert!(borrowed_keys.validate().is_err());
    }
}
