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

mod derive_hygiene_edge_cases {
    #![allow(dead_code)]

    // This deliberately shadows the dependency's crate name. Generated paths must
    // start with `::serde_valid` to resolve the external crate.
    mod serde_valid {}

    fn validate_field(_value: &String) -> Result<(), ::serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(::serde_valid::Validate)]
    struct CustomField {
        #[validate(custom = validate_field)]
        value: String,
    }

    fn validate_struct(_value: &CustomStruct) -> Result<(), ::serde_valid::validation::Error> {
        Ok(())
    }

    fn validate_internal_collector_names(
        _value: &InternalCollectorNames,
    ) -> Result<(), ::serde_valid::validation::Error> {
        Err(::serde_valid::validation::Error::Custom(
            "struct error".to_owned(),
        ))
    }

    #[derive(::serde_valid::Validate)]
    #[validate(custom = validate_struct)]
    struct CustomStruct {
        value: String,
    }

    #[derive(::serde_valid::Validate)]
    #[validate(custom = validate_internal_collector_names)]
    struct InternalCollectorNames {
        #[validate(min_length = 1)]
        __property_vec_errors_map: String,
        #[validate(min_length = 1)]
        __rule_vec_errors: String,
    }

    #[derive(::serde_valid::Validate)]
    enum IgnoredNamedFields {
        RawIdentifier {
            r#type: String,
            #[validate(minimum = 0)]
            value: i32,
        },
        BindingCollision {
            foo: String,
            #[validate(min_length = 1)]
            _foo: String,
        },
        CollectorNames {
            #[validate(min_length = 1)]
            __property_vec_errors_map: String,
            #[validate(min_length = 1)]
            __rule_vec_errors: String,
        },
    }

    #[allow(non_camel_case_types)]
    #[derive(::serde::Serialize, ::serde_valid::Validate)]
    struct std {
        #[serde(rename = "renamed")]
        #[validate(minimum = 0)]
        value: i32,
    }

    #[test]
    fn generated_identifiers_do_not_collide_with_user_fields() {
        let value = InternalCollectorNames {
            __property_vec_errors_map: String::new(),
            __rule_vec_errors: String::new(),
        };

        let errors = ::serde_valid::Validate::validate(&value).unwrap_err();
        assert_eq!(
            ::serde_json::to_value(errors).unwrap(),
            ::serde_json::json!({
                "errors": ["struct error"],
                "properties": {
                    "__property_vec_errors_map": {
                        "errors": ["The length of the value must be `>= 1`."]
                    },
                    "__rule_vec_errors": {
                        "errors": ["The length of the value must be `>= 1`."]
                    }
                }
            })
        );
    }

    #[test]
    fn ignored_enum_fields_accept_raw_and_overlapping_identifiers() {
        let raw = IgnoredNamedFields::RawIdentifier {
            r#type: "value".to_owned(),
            value: 0,
        };
        let collision = IgnoredNamedFields::BindingCollision {
            foo: "ignored".to_owned(),
            _foo: "validated".to_owned(),
        };
        let collector_names = IgnoredNamedFields::CollectorNames {
            __property_vec_errors_map: String::new(),
            __rule_vec_errors: String::new(),
        };

        assert!(::serde_valid::Validate::validate(&raw).is_ok());
        assert!(::serde_valid::Validate::validate(&collision).is_ok());
        assert_eq!(
            ::serde_json::to_value(
                ::serde_valid::Validate::validate(&collector_names).unwrap_err()
            )
            .unwrap(),
            ::serde_json::json!({
                "errors": [],
                "properties": {
                    "__property_vec_errors_map": {
                        "errors": ["The length of the value must be `>= 1`."]
                    },
                    "__rule_vec_errors": {
                        "errors": ["The length of the value must be `>= 1`."]
                    }
                }
            })
        );
    }

    #[test]
    fn generated_dependency_and_standard_library_paths_are_absolute() {
        let field = CustomField {
            value: "value".to_owned(),
        };
        let structure = CustomStruct {
            value: "value".to_owned(),
        };
        let standard_library_shadow = std { value: 0 };

        assert!(::serde_valid::Validate::validate(&field).is_ok());
        assert!(::serde_valid::Validate::validate(&structure).is_ok());
        assert!(::serde_valid::Validate::validate(&standard_library_shadow).is_ok());
    }
}

mod prelude_name_hygiene_edge_cases {
    mod result_variants {
        #![allow(dead_code)]

        struct Ok<T>(T);
        struct Err<T>(T);

        #[derive(::serde_valid::Validate)]
        struct Input {
            #[validate(minimum = 0)]
            value: i32,
        }

        #[test]
        fn result_variant_names_do_not_affect_generated_validation() {
            assert!(::serde_valid::Validate::validate(&Input { value: 0 }).is_ok());
        }
    }

    mod vec_type {
        #![allow(dead_code)]

        struct Vec;

        #[derive(::serde_valid::Validate)]
        struct Input {
            #[validate(minimum = 0)]
            value: i32,
        }

        #[test]
        fn vec_type_name_does_not_affect_generated_errors() {
            assert_eq!(
                ::serde_json::to_value(
                    ::serde_valid::Validate::validate(&Input { value: -1 }).unwrap_err()
                )
                .unwrap(),
                ::serde_json::json!({
                    "errors": [],
                    "properties": {
                        "value": { "errors": ["The number must be `>= 0`."] }
                    }
                })
            );
        }
    }

    mod vec_macro {
        #![allow(unused_macros)]

        macro_rules! vec {
            ($($tokens:tt)*) => {
                0usize
            };
        }

        #[derive(::serde_valid::Validate)]
        struct Input {
            #[validate(minimum = 0)]
            value: i32,
        }

        #[test]
        fn vec_macro_name_does_not_affect_generated_errors() {
            assert_eq!(
                ::serde_json::to_value(
                    ::serde_valid::Validate::validate(&Input { value: -1 }).unwrap_err()
                )
                .unwrap(),
                ::serde_json::json!({
                    "errors": [],
                    "properties": {
                        "value": { "errors": ["The number must be `>= 0`."] }
                    }
                })
            );
        }
    }

    mod to_string_trait {
        #![allow(dead_code)]

        trait ToString {
            fn to_string(&self) -> String;
        }

        impl ToString for str {
            fn to_string(&self) -> String {
                String::from("shadowed")
            }
        }

        #[derive(::serde_valid::Validate)]
        struct Input {
            #[validate(minimum = 0, message = "too small")]
            value: i32,
        }

        #[test]
        fn to_string_trait_name_does_not_affect_custom_messages() {
            let errors = ::serde_valid::Validate::validate(&Input { value: -1 }).unwrap_err();
            assert_eq!(
                ::serde_json::to_value(errors).unwrap(),
                ::serde_json::json!({
                    "errors": [],
                    "properties": {
                        "value": { "errors": ["too small"] }
                    }
                })
            );
        }
    }
}

mod issue107 {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
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

        assert_eq!(
            serde_json::to_value(white_list.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "white_type": { "errors": ["The length of the value must be `>= 1`."] }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(data.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "items": {
                    "0": { "errors": ["The number must be `>= 0`."] }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(v1.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "validated_field": { "errors": ["The number must be `<= 100`."] }
                }
            })
        );

        let v2 = MultiVariant::Variant2 {
            field1: "test".to_string(),
            field2: -10,
            field3: true,
        };
        assert_eq!(
            serde_json::to_value(v2.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "field2": { "errors": ["The number must be `>= 0`."] }
                }
            })
        );

        let v3 = MultiVariant::Variant3("".to_string(), 10, false);
        assert_eq!(
            serde_json::to_value(v3.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "items": {
                    "0": { "errors": ["The length of the value must be `>= 1`."] }
                }
            })
        );
    }
}

mod issue125 {
    #![allow(non_snake_case)]

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
        #[validate(unique_items)]
        #[serde(borrow)]
        names: Box<[&'a str]>,
    }

    #[test]
    fn boxed_slice_supports_array_validators() {
        let value = BorrowedNames {
            names: vec!["", "duplicate", "duplicate"].into_boxed_slice(),
        };

        assert_eq!(
            serde_json::to_value(value.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "names": {
                        "errors": [
                            "The length of the items must be `>= 4`.",
                            "The length of the items must be `<= 2`.",
                            "The items must be unique."
                        ]
                    }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(BorrowedSlice { data: &[1] }.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "data": { "errors": ["The length of the items must be `>= 2`."] }
                }
            })
        );
        assert_eq!(
            serde_json::to_value(BorrowedSlice { data: &[1, 2, 3] }.validate().unwrap_err())
                .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "data": { "errors": ["The length of the items must be `<= 2`."] }
                }
            })
        );
        assert_eq!(
            serde_json::to_value(BorrowedSlice { data: &[1, 1] }.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "data": { "errors": ["The items must be unique."] }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(BoxedString { value: "".into() }.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The length of the value must be `>= 1`."] }
                }
            })
        );
        assert_eq!(
            serde_json::to_value(
                BoxedString {
                    value: "too long".into(),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The length of the value must be `<= 2`."] }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(
                BoxedStringConstraints {
                    value: "beta".into(),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": {
                        "errors": ["The value must match the pattern of \"^(alpha|gamma)$\"."]
                    }
                }
            })
        );

        assert_eq!(
            serde_json::to_value(
                BoxedStringConstraints {
                    value: "gamma".into(),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [alpha, beta]."] }
                }
            })
        );
    }

    #[allow(non_snake_case)]
    #[derive(Validate)]
    struct PatternIdentifierHygiene {
        #[validate(pattern = "^a$")]
        #[validate(pattern = "^b$")]
        value: String,
        #[validate(pattern = "^a$")]
        foo: String,
        #[validate(pattern = "^b$")]
        FOO: String,
        #[validate(pattern = "^a$")]
        r#type: String,
        #[validate(pattern = "^a$")]
        __pattern: String,
        #[validate(pattern = "^a$")]
        __SERDE_VALID_PATTERN: String,
    }

    #[test]
    fn pattern_validators_use_field_independent_scoped_identifiers() {
        let errors = serde_json::to_value(
            PatternIdentifierHygiene {
                value: "invalid".to_owned(),
                foo: "invalid".to_owned(),
                FOO: "invalid".to_owned(),
                r#type: "invalid".to_owned(),
                __pattern: "invalid".to_owned(),
                __SERDE_VALID_PATTERN: "invalid".to_owned(),
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
                    "value": {
                        "errors": [
                            "The value must match the pattern of \"^a$\".",
                            "The value must match the pattern of \"^b$\"."
                        ]
                    },
                    "foo": { "errors": ["The value must match the pattern of \"^a$\"."] },
                    "FOO": { "errors": ["The value must match the pattern of \"^b$\"."] },
                    "type": { "errors": ["The value must match the pattern of \"^a$\"."] },
                    "__pattern": { "errors": ["The value must match the pattern of \"^a$\"."] },
                    "__SERDE_VALID_PATTERN": {
                        "errors": ["The value must match the pattern of \"^a$\"."]
                    }
                }
            })
        );
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

    #[test]
    fn boxed_and_cow_values_delegate_enum_validation_to_their_inner_type() {
        assert!(BoxedIntegerConstraint { value: Box::new(1) }
            .validate()
            .is_ok());
        assert_eq!(
            serde_json::to_value(
                BoxedIntegerConstraint { value: Box::new(3) }
                    .validate()
                    .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [1, 2]."] }
                }
            })
        );

        assert!(BoxedCustomEnumConstraint {
            value: Box::new(CustomEnumValue("alpha".to_owned())),
        }
        .validate()
        .is_ok());
        assert_eq!(
            serde_json::to_value(
                BoxedCustomEnumConstraint {
                    value: Box::new(CustomEnumValue("gamma".to_owned())),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [alpha, beta]."] }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(
                CowStringConstraint {
                    value: std::borrow::Cow::Owned("gamma".to_owned()),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [alpha, beta]."] }
                }
            })
        );
        assert!(CowOsStrConstraint {
            value: std::borrow::Cow::Borrowed(std::ffi::OsStr::new("alpha")),
        }
        .validate()
        .is_ok());
        assert_eq!(
            serde_json::to_value(
                CowOsStrConstraint {
                    value: std::borrow::Cow::Borrowed(std::ffi::OsStr::new("gamma")),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [alpha, beta]."] }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(
                CowPathConstraint {
                    value: std::borrow::Cow::Borrowed(std::path::Path::new("gamma")),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [alpha, beta]."] }
                }
            })
        );
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
        assert_eq!(
            serde_json::to_value(
                CowIntegerConstraint {
                    value: std::borrow::Cow::Owned(3),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [1, 2]."] }
                }
            })
        );

        let custom = CustomEnumValue("alpha".to_owned());
        assert!(BorrowedCustomEnumConstraint { value: &custom }
            .validate()
            .is_ok());
        let integer = 3;
        assert_eq!(
            serde_json::to_value(
                BorrowedIntegerConstraint { value: &integer }
                    .validate()
                    .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [1, 2]."] }
                }
            })
        );
    }

    #[derive(Clone, Debug, PartialEq)]
    struct NonCopyCandidate(i32);

    impl From<NonCopyCandidate> for serde_valid::validation::Literal {
        fn from(candidate: NonCopyCandidate) -> Self {
            candidate.0.into()
        }
    }

    #[derive(Clone, Debug)]
    struct CandidateValue(i32);

    impl ValidateEnum<NonCopyCandidate> for CandidateValue {
        fn validate_enum(&self, candidates: &[NonCopyCandidate]) -> Result<(), EnumError> {
            if candidates.iter().any(|candidate| candidate.0 == self.0) {
                Ok(())
            } else {
                Err(EnumError::new(candidates))
            }
        }
    }

    #[test]
    fn enum_wrappers_support_non_copy_candidate_types() {
        use serde_valid::__private::ValidateCompositedEnum;

        let candidates = vec![NonCopyCandidate(1), NonCopyCandidate(2)];

        let value = CandidateValue(1);
        let borrowed = &value;
        assert!(ValidateCompositedEnum::validate_composited_enum(&borrowed, &candidates).is_ok());

        let boxed = Box::new(CandidateValue(1));
        assert!(ValidateCompositedEnum::validate_composited_enum(&boxed, &candidates).is_ok());

        let cow: std::borrow::Cow<'_, CandidateValue> = std::borrow::Cow::Owned(CandidateValue(1));
        assert!(ValidateCompositedEnum::validate_composited_enum(&cow, &candidates).is_ok());
    }

    #[derive(Debug, Validate)]
    struct BoxedArray {
        #[validate(min_items = 4)]
        #[validate(max_items = 2)]
        #[validate(unique_items)]
        values: Box<[i32; 3]>,
    }

    #[derive(Debug, Validate)]
    struct BorrowedArray<'a> {
        #[validate(min_items = 4)]
        #[validate(max_items = 2)]
        #[validate(unique_items)]
        values: &'a [i32; 3],
    }

    #[test]
    fn boxed_and_borrowed_arrays_support_array_validators() {
        let array_errors = json!({
            "errors": [],
            "properties": {
                "values": {
                    "errors": [
                        "The length of the items must be `>= 4`.",
                        "The length of the items must be `<= 2`.",
                        "The items must be unique."
                    ]
                }
            }
        });
        assert_eq!(
            serde_json::to_value(
                BoxedArray {
                    values: Box::new([0, 1, 1]),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            array_errors
        );

        let values = [0, 1, 1];
        assert_eq!(
            serde_json::to_value(BorrowedArray { values: &values }.validate().unwrap_err())
                .unwrap(),
            array_errors
        );
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
        let nested_errors = json!({
            "errors": [],
            "properties": {
                "children": {
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
        });
        assert_eq!(
            serde_json::to_value(BoxedChildren { children }.validate().unwrap_err()).unwrap(),
            nested_errors
        );

        let children = [Child { value: 0 }];
        assert_eq!(
            serde_json::to_value(
                BorrowedChildren {
                    children: &children,
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            nested_errors
        );
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

        impl std::fmt::Display for AliasedKey {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("same")
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
    fn stringifiable_map_key_contract_supports_owned_and_borrowed_keys() {
        #[derive(Eq, Hash, PartialEq)]
        struct CustomKey(String);

        impl std::fmt::Display for CustomKey {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        let string_keys = HashMap::from([("string".to_owned(), Child { value: 0 })]);
        let child_map_errors = json!({
            "errors": [],
            "properties": {
                "value": { "errors": ["The number must be `>= 1`."] }
            }
        });
        assert_eq!(
            serde_json::to_value(string_keys.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "string": child_map_errors
                }
            })
        );

        let custom_keys = HashMap::from([(CustomKey("custom".to_owned()), Child { value: 0 })]);
        assert_eq!(
            serde_json::to_value(custom_keys.validate().unwrap_err()).unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "custom": child_map_errors
                }
            })
        );

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
    }

    #[test]
    fn nested_maps_merge_errors_for_aliased_keys() {
        #[derive(Eq, Hash, PartialEq)]
        struct AliasedKey(u8);

        impl std::fmt::Display for AliasedKey {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("same")
            }
        }

        let values = IndexMap::from([
            (AliasedKey(1), Child { value: 0 }),
            (AliasedKey(2), Child { value: 0 }),
        ]);
        let errors = serde_json::to_value(values.validate().unwrap_err()).unwrap();

        assert_eq!(
            errors["properties"]["same"]["properties"]["value"]["errors"],
            json!(["The number must be `>= 1`.", "The number must be `>= 1`."])
        );
    }

    #[derive(Debug, Validate)]
    #[allow(clippy::box_collection)]
    struct ArrayValidatorWrappers<'a> {
        #[validate(min_items = 3)]
        #[validate(max_items = 1)]
        #[validate(unique_items)]
        boxed_vec: Box<Vec<i32>>,
        #[validate(min_items = 3)]
        #[validate(max_items = 1)]
        #[validate(unique_items)]
        borrowed_vec: &'a Vec<i32>,
        #[validate(min_items = 3)]
        #[validate(max_items = 1)]
        #[validate(unique_items)]
        cow_slice: std::borrow::Cow<'a, [i32]>,
    }

    #[test]
    fn standard_wrappers_support_array_validators() {
        let borrowed_vec = vec![1, 1];
        let errors = serde_json::to_value(
            ArrayValidatorWrappers {
                boxed_vec: Box::new(vec![1, 1]),
                borrowed_vec: &borrowed_vec,
                cow_slice: std::borrow::Cow::Borrowed(&[1, 1]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        let item_errors = json!({
            "errors": [
                "The length of the items must be `>= 3`.",
                "The length of the items must be `<= 1`.",
                "The items must be unique."
            ]
        });
        assert_eq!(
            errors,
            json!({
                "errors": [],
                "properties": {
                    "boxed_vec": item_errors,
                    "borrowed_vec": item_errors,
                    "cow_slice": item_errors,
                }
            })
        );
    }

    #[derive(Debug, Validate)]
    #[allow(clippy::box_collection)]
    struct ForwardedArrayValidatorWrappers {
        #[validate(min_items = 3)]
        #[validate(max_items = 1)]
        #[validate(unique_items)]
        rc_vec: std::rc::Rc<Vec<i32>>,
        #[validate(min_items = 3)]
        #[validate(max_items = 1)]
        #[validate(unique_items)]
        arc_vec: std::sync::Arc<Vec<i32>>,
        #[validate(min_items = 3)]
        #[validate(max_items = 1)]
        #[validate(unique_items)]
        pinned_vec: std::pin::Pin<Box<Vec<i32>>>,
        #[validate(min_items = 3)]
        #[validate(max_items = 1)]
        #[validate(unique_items)]
        custom_vec: InherentMethodShadow<Vec<i32>>,
    }

    #[test]
    fn forwarded_wrappers_support_array_validators() {
        let errors = serde_json::to_value(
            ForwardedArrayValidatorWrappers {
                rc_vec: std::rc::Rc::new(vec![1, 1]),
                arc_vec: std::sync::Arc::new(vec![1, 1]),
                pinned_vec: Box::pin(vec![1, 1]),
                custom_vec: InherentMethodShadow(vec![1, 1]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        let item_errors = json!({
            "errors": [
                "The length of the items must be `>= 3`.",
                "The length of the items must be `<= 1`.",
                "The items must be unique."
            ]
        });
        assert_eq!(
            errors,
            json!({
                "errors": [],
                "properties": {
                    "rc_vec": item_errors,
                    "arc_vec": item_errors,
                    "pinned_vec": item_errors,
                    "custom_vec": item_errors,
                }
            })
        );
    }

    #[derive(Debug)]
    struct InherentMethodShadow<T>(T);

    impl<T> std::ops::Deref for InherentMethodShadow<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> serde_valid::Validate for InherentMethodShadow<T>
    where
        T: serde_valid::Validate,
    {
        fn validate(&self) -> Result<(), serde_valid::validation::Errors> {
            serde_valid::Validate::validate(&self.0)
        }
    }

    macro_rules! impl_inherent_method_shadow_validation {
        ($Trait:ident, $method:ident, $Error:ident) => {
            impl<T> serde_valid::$Trait for InherentMethodShadow<T>
            where
                T: serde_valid::$Trait,
            {
                fn $method(&self) -> Result<(), serde_valid::$Error> {
                    serde_valid::$Trait::$method(&self.0)
                }
            }
        };
        ($Trait:ident, $method:ident, $Argument:ty, $Error:ident) => {
            impl<T> serde_valid::$Trait for InherentMethodShadow<T>
            where
                T: serde_valid::$Trait,
            {
                fn $method(&self, argument: $Argument) -> Result<(), serde_valid::$Error> {
                    serde_valid::$Trait::$method(&self.0, argument)
                }
            }
        };
    }

    impl_inherent_method_shadow_validation!(
        ValidateMinItems,
        validate_min_items,
        usize,
        MinItemsError
    );
    impl_inherent_method_shadow_validation!(
        ValidateMaxItems,
        validate_max_items,
        usize,
        MaxItemsError
    );
    impl_inherent_method_shadow_validation!(
        ValidateUniqueItems,
        validate_unique_items,
        UniqueItemsError
    );

    macro_rules! impl_inherent_method_shadow_composited_validation {
        ($Trait:ident, $method:ident, $Argument:ty, $Error:ident) => {
            impl<T> serde_valid::$Trait for InherentMethodShadow<T>
            where
                T: serde_valid::$Trait,
            {
                fn $method(&self, argument: $Argument) -> Result<(), serde_valid::$Error> {
                    serde_valid::$Trait::$method(&self.0, argument)
                }
            }
        };
        (generic_owned $Trait:ident, $method:ident, $Error:ident) => {
            impl<C, T> serde_valid::$Trait<C> for InherentMethodShadow<T>
            where
                T: serde_valid::$Trait<C>,
            {
                fn $method(&self, argument: C) -> Result<(), serde_valid::$Error> {
                    serde_valid::$Trait::$method(&self.0, argument)
                }
            }
        };
        (generic_slice $Trait:ident, $method:ident, $Error:ident) => {
            impl<C, T> serde_valid::$Trait<C> for InherentMethodShadow<T>
            where
                T: serde_valid::$Trait<C>,
            {
                fn $method(&self, argument: &[C]) -> Result<(), serde_valid::$Error> {
                    serde_valid::$Trait::$method(&self.0, argument)
                }
            }
        };
    }

    impl_inherent_method_shadow_composited_validation!(
        generic_slice ValidateEnum,
        validate_enum,
        EnumError
    );
    impl_inherent_method_shadow_composited_validation!(
        generic_owned ValidateMultipleOf,
        validate_multiple_of,
        MultipleOfError
    );
    impl_inherent_method_shadow_composited_validation!(
        generic_owned ValidateMinimum,
        validate_minimum,
        MinimumError
    );
    impl_inherent_method_shadow_composited_validation!(
        generic_owned ValidateMaximum,
        validate_maximum,
        MaximumError
    );
    impl_inherent_method_shadow_composited_validation!(
        generic_owned ValidateExclusiveMinimum,
        validate_exclusive_minimum,
        ExclusiveMinimumError
    );
    impl_inherent_method_shadow_composited_validation!(
        generic_owned ValidateExclusiveMaximum,
        validate_exclusive_maximum,
        ExclusiveMaximumError
    );
    impl_inherent_method_shadow_composited_validation!(
        ValidateMinProperties,
        validate_min_properties,
        usize,
        MinPropertiesError
    );
    impl_inherent_method_shadow_composited_validation!(
        ValidateMaxProperties,
        validate_max_properties,
        usize,
        MaxPropertiesError
    );
    impl_inherent_method_shadow_composited_validation!(
        ValidateMinLength,
        validate_min_length,
        usize,
        MinLengthError
    );
    impl_inherent_method_shadow_composited_validation!(
        ValidateMaxLength,
        validate_max_length,
        usize,
        MaxLengthError
    );
    impl_inherent_method_shadow_composited_validation!(
        ValidatePattern,
        validate_pattern,
        &serde_valid::export::regex::Regex,
        PatternError
    );

    #[allow(dead_code)]
    impl<T> InherentMethodShadow<T> {
        fn validate_min_items(&self, _minimum: usize) -> Result<(), serde_valid::MinItemsError> {
            Ok(())
        }

        fn validate_max_items(&self, _maximum: usize) -> Result<(), serde_valid::MaxItemsError> {
            Ok(())
        }

        fn validate_unique_items(&self) -> Result<(), serde_valid::UniqueItemsError> {
            Ok(())
        }

        fn validate(&self) -> Result<(), serde_valid::validation::Errors> {
            Ok(())
        }

        fn validate_composited_enum<C>(
            &self,
            _candidates: &[C],
        ) -> Result<(), serde_valid::EnumError> {
            Ok(())
        }

        fn validate_composited_multiple_of<A>(
            &self,
            _multiple_of: A,
        ) -> Result<(), serde_valid::MultipleOfError> {
            Ok(())
        }

        fn validate_composited_minimum<A>(
            &self,
            _minimum: A,
        ) -> Result<(), serde_valid::MinimumError> {
            Ok(())
        }

        fn validate_composited_maximum<A>(
            &self,
            _maximum: A,
        ) -> Result<(), serde_valid::MaximumError> {
            Ok(())
        }

        fn validate_composited_exclusive_minimum<A>(
            &self,
            _minimum: A,
        ) -> Result<(), serde_valid::ExclusiveMinimumError> {
            Ok(())
        }

        fn validate_composited_exclusive_maximum<A>(
            &self,
            _maximum: A,
        ) -> Result<(), serde_valid::ExclusiveMaximumError> {
            Ok(())
        }

        fn validate_composited_min_properties(
            &self,
            _minimum: usize,
        ) -> Result<(), serde_valid::MinPropertiesError> {
            Ok(())
        }

        fn validate_composited_max_properties(
            &self,
            _maximum: usize,
        ) -> Result<(), serde_valid::MaxPropertiesError> {
            Ok(())
        }

        fn validate_composited_min_length(
            &self,
            _minimum: usize,
        ) -> Result<(), serde_valid::MinLengthError> {
            Ok(())
        }

        fn validate_composited_max_length(
            &self,
            _maximum: usize,
        ) -> Result<(), serde_valid::MaxLengthError> {
            Ok(())
        }

        fn validate_composited_pattern(
            &self,
            _pattern: &serde_valid::export::regex::Regex,
        ) -> Result<(), serde_valid::PatternError> {
            Ok(())
        }
    }

    #[derive(Debug, Validate)]
    struct InherentMethodShadowConstraints {
        #[validate]
        nested: InherentMethodShadow<WrappedChild>,
        #[validate(r#enum = [1, 2])]
        enum_values: InherentMethodShadow<i32>,
        #[validate(multiple_of = 2)]
        multiple_of: InherentMethodShadow<i32>,
        #[validate(minimum = 1)]
        minimum: InherentMethodShadow<i32>,
        #[validate(maximum = 1)]
        maximum: InherentMethodShadow<i32>,
        #[validate(exclusive_minimum = 1)]
        exclusive_minimum: InherentMethodShadow<i32>,
        #[validate(exclusive_maximum = 1)]
        exclusive_maximum: InherentMethodShadow<i32>,
        #[validate(min_properties = 1)]
        min_properties: InherentMethodShadow<HashMap<String, i32>>,
        #[validate(max_properties = 1)]
        max_properties: InherentMethodShadow<HashMap<String, i32>>,
        #[validate(min_length = 1)]
        min_length: InherentMethodShadow<String>,
        #[validate(max_length = 1)]
        max_length: InherentMethodShadow<String>,
        #[validate(pattern = "^[a-z]+$")]
        pattern: InherentMethodShadow<String>,
    }

    #[test]
    fn derive_dispatch_ignores_inherent_validator_method_names() {
        let errors = serde_json::to_value(
            InherentMethodShadowConstraints {
                nested: InherentMethodShadow(WrappedChild { value: 0 }),
                enum_values: InherentMethodShadow(3),
                multiple_of: InherentMethodShadow(3),
                minimum: InherentMethodShadow(0),
                maximum: InherentMethodShadow(2),
                exclusive_minimum: InherentMethodShadow(1),
                exclusive_maximum: InherentMethodShadow(1),
                min_properties: InherentMethodShadow(HashMap::new()),
                max_properties: InherentMethodShadow(HashMap::from([
                    ("one".to_owned(), 1),
                    ("two".to_owned(), 2),
                ])),
                min_length: InherentMethodShadow(String::new()),
                max_length: InherentMethodShadow("xx".to_owned()),
                pattern: InherentMethodShadow("123".to_owned()),
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
                    "nested": {
                        "errors": [],
                        "properties": {
                            "value": { "errors": ["The number must be `>= 1`."] }
                        }
                    },
                    "enum_values": { "errors": ["The value must be in [1, 2]."] },
                    "multiple_of": { "errors": ["The value must be multiple of `2`."] },
                    "minimum": { "errors": ["The number must be `>= 1`."] },
                    "maximum": { "errors": ["The number must be `<= 1`."] },
                    "exclusive_minimum": { "errors": ["The number must be `> 1`."] },
                    "exclusive_maximum": { "errors": ["The number must be `< 1`."] },
                    "min_properties": { "errors": ["The size of the properties must be `>= 1`."] },
                    "max_properties": { "errors": ["The size of the properties must be `<= 1`."] },
                    "min_length": { "errors": ["The length of the value must be `>= 1`."] },
                    "max_length": { "errors": ["The length of the value must be `<= 1`."] },
                    "pattern": { "errors": ["The value must match the pattern of \"^[a-z]+$\"."] }
                }
            })
        );
    }

    struct CustomScalarEnumValue;

    impl serde_valid::ValidateEnum<i32> for CustomScalarEnumValue {
        fn validate_enum(&self, candidates: &[i32]) -> Result<(), serde_valid::EnumError> {
            Err(serde_valid::EnumError::new(candidates))
        }
    }

    #[derive(Validate)]
    struct CustomScalarEnumConstraint {
        #[validate(r#enum = [1, 2])]
        value: CustomScalarEnumValue,
    }

    #[test]
    fn derive_promotes_custom_scalar_enum_implementations() {
        assert_eq!(
            serde_json::to_value(
                CustomScalarEnumConstraint {
                    value: CustomScalarEnumValue,
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "value": { "errors": ["The value must be in [1, 2]."] }
                }
            })
        );
    }

    #[derive(Validate)]
    struct DstForwardingConstraints {
        #[validate(min_length = 1)]
        min_length: std::rc::Rc<str>,
        #[validate(pattern = "^[a-z]+$")]
        pattern: std::rc::Rc<str>,
        #[validate(r#enum = ["allowed"])]
        enum_value: std::rc::Rc<str>,
    }

    #[test]
    fn standard_forwarding_supports_dynamically_sized_targets() {
        let errors = serde_json::to_value(
            DstForwardingConstraints {
                min_length: std::rc::Rc::from(""),
                pattern: std::rc::Rc::from("123"),
                enum_value: std::rc::Rc::from("denied"),
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
                    "min_length": { "errors": ["The length of the value must be `>= 1`."] },
                    "pattern": { "errors": ["The value must match the pattern of \"^[a-z]+$\"."] },
                    "enum_value": { "errors": ["The value must be in [allowed]."] }
                }
            })
        );
    }

    #[derive(Debug, Clone, Validate)]
    struct WrappedChild {
        #[validate(minimum = 1)]
        value: i32,
    }

    #[derive(Debug, Validate)]
    #[allow(clippy::box_collection)]
    struct NestedValidatorWrappers<'a> {
        #[validate]
        boxed_vec: Option<Box<Vec<WrappedChild>>>,
        #[validate]
        borrowed_vec: Option<&'a Vec<WrappedChild>>,
        #[validate]
        cow_slice: Option<std::borrow::Cow<'a, [WrappedChild]>>,
    }

    #[test]
    fn standard_wrappers_support_nested_validation() {
        let borrowed_vec = vec![WrappedChild { value: 0 }];
        let errors = serde_json::to_value(
            NestedValidatorWrappers {
                boxed_vec: Some(Box::new(vec![WrappedChild { value: 0 }])),
                borrowed_vec: Some(&borrowed_vec),
                cow_slice: Some(std::borrow::Cow::Owned(vec![WrappedChild { value: 0 }])),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        let nested_errors = json!({
            "errors": [],
            "items": {
                "0": {
                    "errors": [],
                    "properties": {
                        "value": { "errors": ["The number must be `>= 1`."] }
                    }
                }
            }
        });
        assert_eq!(
            errors,
            json!({
                "errors": [],
                "properties": {
                    "boxed_vec": nested_errors,
                    "borrowed_vec": nested_errors,
                    "cow_slice": nested_errors,
                }
            })
        );
    }

    #[derive(Debug, Validate)]
    struct IndexMapPropertyConstraints {
        #[validate(min_properties = 1)]
        #[validate(max_properties = 1)]
        values: IndexMap<String, i32>,
    }

    #[test]
    fn index_map_supports_property_count_validators() {
        assert!(IndexMapPropertyConstraints {
            values: IndexMap::from([("one".to_owned(), 1)]),
        }
        .validate()
        .is_ok());
        assert_eq!(
            serde_json::to_value(
                IndexMapPropertyConstraints {
                    values: IndexMap::new(),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "values": { "errors": ["The size of the properties must be `>= 1`."] }
                }
            })
        );
        assert_eq!(
            serde_json::to_value(
                IndexMapPropertyConstraints {
                    values: IndexMap::from([("one".to_owned(), 1), ("two".to_owned(), 2)]),
                }
                .validate()
                .unwrap_err()
            )
            .unwrap(),
            json!({
                "errors": [],
                "properties": {
                    "values": { "errors": ["The size of the properties must be `<= 1`."] }
                }
            })
        );
    }

    #[derive(Debug, Eq, Hash, PartialEq)]
    struct AliasedKey(u8);

    impl std::fmt::Display for AliasedKey {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("same")
        }
    }

    #[derive(Debug)]
    struct DataDependentValidation(bool);

    impl Validate for DataDependentValidation {
        fn validate(&self) -> Result<(), serde_valid::validation::Errors> {
            use serde_valid::validation::{
                ArrayErrors, Error, Errors, ItemErrorsMap, ObjectErrors, PropertyErrorsMap,
            };

            if self.0 {
                Err(Errors::Object(ObjectErrors::new(
                    vec![Error::Custom("object error".to_owned())],
                    PropertyErrorsMap::from([(
                        "property".into(),
                        Errors::NewType(vec![Error::Custom("property error".to_owned())]),
                    )]),
                )))
            } else {
                Err(Errors::Array(ArrayErrors::new(
                    vec![Error::Custom("array error".to_owned())],
                    ItemErrorsMap::from([(
                        0,
                        Errors::NewType(vec![Error::Custom("item error".to_owned())]),
                    )]),
                )))
            }
        }
    }

    #[test]
    fn aliased_map_keys_with_different_error_shapes_do_not_panic() {
        let index_map = IndexMap::from([
            (AliasedKey(1), DataDependentValidation(false)),
            (AliasedKey(2), DataDependentValidation(true)),
        ]);
        let index_map_errors = serde_json::to_value(index_map.validate().unwrap_err()).unwrap();
        assert_eq!(
            index_map_errors["properties"]["same"],
            json!({
                "errors": ["array error", "object error"],
                "properties": {
                    "property": { "errors": ["property error"] }
                }
            })
        );

        let hash_map = HashMap::from([
            (AliasedKey(1), DataDependentValidation(false)),
            (AliasedKey(2), DataDependentValidation(true)),
        ]);
        let mut hash_map_errors = serde_json::to_value(hash_map.validate().unwrap_err()).unwrap();
        hash_map_errors["properties"]["same"]["errors"]
            .as_array_mut()
            .unwrap()
            .sort_by(|left, right| left.as_str().cmp(&right.as_str()));
        assert_eq!(
            hash_map_errors["properties"]["same"],
            json!({
                "errors": ["array error", "object error"],
                "properties": {
                    "property": { "errors": ["property error"] }
                }
            })
        );
    }
}
