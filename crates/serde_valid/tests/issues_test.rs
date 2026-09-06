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
        let errors = ::serde_json::to_value(errors).unwrap();
        assert_eq!(errors["errors"], ::serde_json::json!(["struct error"]));
        assert!(errors["properties"]["__property_vec_errors_map"].is_object());
        assert!(errors["properties"]["__rule_vec_errors"].is_object());
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
        assert!(::serde_valid::Validate::validate(&collector_names).is_err());
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
            assert!(::serde_valid::Validate::validate(&Input { value: -1 }).is_err());
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
            assert!(::serde_valid::Validate::validate(&Input { value: -1 }).is_err());
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
            assert!(errors.to_string().contains("too small"));
        }
    }
}

#[allow(deprecated)]
mod deprecated_enumerate_warning_hygiene {
    fn __enumerate_0(_value: &String) -> Result<(), ::serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(::serde_valid::Validate)]
    struct Input {
        #[validate(enumerate = ["a"])]
        enumerated: String,
        #[validate(custom = __enumerate_0)]
        custom: String,
    }

    #[test]
    fn warning_helper_does_not_shadow_custom_validator() {
        let value = Input {
            enumerated: "a".to_owned(),
            custom: "value".to_owned(),
        };
        assert!(::serde_valid::Validate::validate(&value).is_ok());
    }
}

mod wrapper_trait_compile_checks {
    #[test]
    fn string_wrappers_implement_public_validation_traits_directly() {
        fn assert_string<T>()
        where
            T: ::serde_valid::ValidateMaxLength
                + ::serde_valid::ValidateMinLength
                + ::serde_valid::ValidatePattern,
        {
        }

        assert_string::<Box<str>>();
        assert_string::<::std::borrow::Cow<'static, ::std::ffi::OsStr>>();
        assert_string::<::std::borrow::Cow<'static, ::std::path::Path>>();
    }

    #[test]
    fn boxed_slices_implement_public_array_traits_directly() {
        fn assert_array<T>()
        where
            T: ::serde_valid::ValidateMaxItems
                + ::serde_valid::ValidateMinItems
                + ::serde_valid::ValidateUniqueItems,
        {
        }

        assert_array::<&'static [i32]>();
        assert_array::<Box<[i32]>>();
        assert_array::<::std::borrow::Cow<'static, [i32]>>();
    }

    #[test]
    fn boxed_slices_keep_generic_and_fixed_composited_forwarding() {
        fn assert_generic<T: ::serde_valid::validation::ValidateCompositedMinimum<i32>>() {}
        fn assert_fixed<T>()
        where
            T: ::serde_valid::validation::ValidateCompositedMaxLength
                + ::serde_valid::validation::ValidateCompositedPattern,
        {
        }

        assert_generic::<Box<[i32]>>();
        assert_fixed::<Box<[String]>>();
    }

    #[test]
    fn every_composited_wrapper_shape_has_direct_trait_implementations() {
        use ::indexmap::IndexMap;
        use ::std::borrow::Cow;
        use ::std::collections::HashMap;

        fn assert_generic<T: ::serde_valid::validation::ValidateCompositedMinimum<i32>>() {}
        fn assert_fixed<T: ::serde_valid::validation::ValidateCompositedMaxLength>() {}

        macro_rules! assert_all_shapes {
            ($assertion:ident, $item:ty) => {
                $assertion::<&'static [$item]>();
                $assertion::<Box<[$item]>>();
                $assertion::<&'static [$item; 1]>();
                $assertion::<Box<[$item; 1]>>();
                $assertion::<&'static Vec<$item>>();
                $assertion::<Box<Vec<$item>>>();
                $assertion::<&'static Option<$item>>();
                $assertion::<Box<Option<$item>>>();
                $assertion::<&'static HashMap<String, $item>>();
                $assertion::<Box<HashMap<String, $item>>>();
                $assertion::<&'static IndexMap<String, $item>>();
                $assertion::<Box<IndexMap<String, $item>>>();
                $assertion::<Cow<'static, Vec<$item>>>();
                $assertion::<Cow<'static, Option<$item>>>();
                $assertion::<Cow<'static, [$item; 1]>>();
                $assertion::<Cow<'static, HashMap<String, $item>>>();
                $assertion::<Cow<'static, IndexMap<String, $item>>>();
                $assertion::<Cow<'static, [$item]>>();
            };
        }

        assert_all_shapes!(assert_generic, i32);
        assert_all_shapes!(assert_fixed, String);
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
    #![allow(non_snake_case)]

    use indexmap::IndexMap;
    use serde::Deserialize;
    use serde_json::json;
    use serde_valid::{EnumError, Validate, ValidateEnum};
    use std::collections::HashMap;

    #[allow(dead_code)]
    struct DownstreamUnsized([u8]);

    impl serde_valid::ValidateMaxLength for DownstreamUnsized {
        fn validate_max_length(
            &self,
            _max_length: usize,
        ) -> Result<(), serde_valid::MaxLengthError> {
            Ok(())
        }
    }

    impl serde_valid::validation::ValidateCompositedMaxLength for DownstreamUnsized {
        fn validate_composited_max_length(
            &self,
            _max_length: usize,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MaxLengthError>> {
            Ok(())
        }
    }

    impl serde_valid::ValidateMinimum<i32> for DownstreamUnsized {
        fn validate_minimum(&self, _minimum: i32) -> Result<(), serde_valid::MinimumError> {
            Ok(())
        }
    }

    impl serde_valid::validation::ValidateCompositedMinimum<i32> for DownstreamUnsized {
        fn validate_composited_minimum(
            &self,
            _minimum: i32,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinimumError>> {
            Ok(())
        }
    }

    impl serde_valid::ValidateEnum<i32> for DownstreamUnsized {
        fn validate_enum(&self, _candidates: &[i32]) -> Result<(), serde_valid::EnumError> {
            Ok(())
        }
    }

    impl<'a> serde_valid::validation::ValidateCompositedEnum<&'a [i32]> for DownstreamUnsized {
        fn validate_composited_enum(
            &self,
            _candidates: &'a [i32],
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::EnumError>> {
            Ok(())
        }
    }

    #[allow(deprecated)]
    impl serde_valid::ValidateEnumerate<i32> for DownstreamUnsized {
        fn validate_enumerate(&self, _candidates: &[i32]) -> Result<(), serde_valid::EnumError> {
            Ok(())
        }
    }

    #[allow(deprecated)]
    impl<'a> serde_valid::validation::ValidateCompositedEnumerate<&'a [i32]> for DownstreamUnsized {
        fn validate_composited_enumerate(
            &self,
            _candidates: &'a [i32],
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::EnumError>> {
            Ok(())
        }
    }

    #[test]
    #[allow(deprecated)]
    fn downstream_unsized_types_can_keep_explicit_composited_impls() {
        fn assert_impl<T: ?Sized>()
        where
            T: serde_valid::ValidateMaxLength
                + serde_valid::validation::ValidateCompositedMaxLength
                + serde_valid::ValidateMinimum<i32>
                + serde_valid::validation::ValidateCompositedMinimum<i32>
                + serde_valid::ValidateEnum<i32>
                + serde_valid::ValidateEnumerate<i32>,
            for<'a> T: serde_valid::validation::ValidateCompositedEnum<&'a [i32]>
                + serde_valid::validation::ValidateCompositedEnumerate<&'a [i32]>,
        {
        }

        assert_impl::<DownstreamUnsized>();
    }

    #[test]
    #[allow(deprecated)]
    fn foreign_dst_terminals_keep_composited_validation_support() {
        fn assert_string_validators<T: ?Sized>()
        where
            T: serde_valid::validation::ValidateCompositedMaxLength
                + serde_valid::validation::ValidateCompositedMinLength
                + serde_valid::validation::ValidateCompositedPattern,
        {
        }

        fn assert_enum_validators<T: ?Sized>()
        where
            for<'a> T: serde_valid::validation::ValidateCompositedEnum<&'a [&'static str]>
                + serde_valid::validation::ValidateCompositedEnumerate<&'a [&'static str]>,
        {
        }

        assert_string_validators::<str>();
        assert_string_validators::<std::ffi::OsStr>();
        assert_string_validators::<std::path::Path>();
        assert_enum_validators::<str>();
        assert_enum_validators::<std::ffi::OsStr>();
        assert_enum_validators::<std::path::Path>();
    }

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
            errors["properties"]["value"]["errors"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        for field in ["foo", "FOO", "r#type", "__pattern", "__SERDE_VALID_PATTERN"] {
            let field_errors = errors["properties"][field]["errors"]
                .as_array()
                .unwrap_or_else(|| panic!("missing pattern error for {field}: {errors}"));
            assert_eq!(
                field_errors.len(),
                1,
                "missing pattern error for {field}: {errors}"
            );
        }
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
        let candidates = vec![NonCopyCandidate(1), NonCopyCandidate(2)];

        let value = CandidateValue(1);
        let borrowed = &value;
        assert!(ValidateEnum::<NonCopyCandidate>::validate_enum(&borrowed, &candidates).is_ok());

        let boxed = Box::new(CandidateValue(1));
        assert!(ValidateEnum::<NonCopyCandidate>::validate_enum(&boxed, &candidates).is_ok());

        let cow: std::borrow::Cow<'_, CandidateValue> = std::borrow::Cow::Owned(CandidateValue(1));
        assert!(ValidateEnum::<NonCopyCandidate>::validate_enum(&cow, &candidates).is_ok());
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

    #[derive(Validate)]
    #[allow(
        clippy::borrowed_box,
        clippy::box_collection,
        clippy::owned_cow,
        clippy::redundant_allocation
    )]
    struct NestedCompositedWrappers<'a> {
        #[validate(minimum = 1)]
        cow_vec: std::borrow::Cow<'a, Vec<i32>>,
        #[validate(minimum = 1)]
        cow_option: std::borrow::Cow<'a, Option<i32>>,
        #[validate(minimum = 1)]
        cow_hash_map: std::borrow::Cow<'a, HashMap<String, i32>>,
        #[validate(minimum = 1)]
        cow_index_map: std::borrow::Cow<'a, IndexMap<String, i32>>,
        #[validate(minimum = 1)]
        cow_array: std::borrow::Cow<'a, [i32; 1]>,
        #[validate(minimum = 1)]
        nested_boxed_vec: Box<Box<Vec<i32>>>,
        #[validate(minimum = 1)]
        borrowed_boxed_vec: &'a Box<Vec<i32>>,
        #[validate(minimum = 1)]
        boxed_borrowed_vec: Box<&'a Vec<i32>>,
        #[validate(minimum = 1)]
        boxed_cow_slice: Box<std::borrow::Cow<'a, [i32]>>,
        #[validate(minimum = 1)]
        rc_vec: std::rc::Rc<Vec<i32>>,
        #[validate(minimum = 1)]
        arc_vec: std::sync::Arc<Vec<i32>>,
        #[validate(minimum = 1)]
        pinned_vec: std::pin::Pin<Box<Vec<i32>>>,
        #[validate(multiple_of = 2)]
        nested_multiple_of: Box<Box<Vec<i32>>>,
        #[validate(min_length = 2)]
        nested_min_length: Box<Box<Vec<String>>>,
        #[validate(pattern = "^[a-z]+$")]
        nested_pattern: Box<Box<Vec<String>>>,
        #[validate(min_properties = 1)]
        nested_min_properties: Box<Box<Vec<HashMap<String, i32>>>>,
        #[validate(r#enum = [1, 2])]
        nested_enum: Box<Box<Vec<i32>>>,
    }

    #[test]
    fn composited_validation_supports_cow_containers_and_nested_wrappers() {
        let borrowed_vec = vec![0];
        let borrowed_boxed_vec = Box::new(vec![0]);
        let errors = serde_json::to_value(
            NestedCompositedWrappers {
                cow_vec: std::borrow::Cow::Owned(vec![0]),
                cow_option: std::borrow::Cow::Owned(Some(0)),
                cow_hash_map: std::borrow::Cow::Owned(HashMap::from([("value".to_owned(), 0)])),
                cow_index_map: std::borrow::Cow::Owned(IndexMap::from([("value".to_owned(), 0)])),
                cow_array: std::borrow::Cow::Owned([0]),
                nested_boxed_vec: Box::new(Box::new(vec![0])),
                borrowed_boxed_vec: &borrowed_boxed_vec,
                boxed_borrowed_vec: Box::new(&borrowed_vec),
                boxed_cow_slice: Box::new(std::borrow::Cow::Borrowed(&[0])),
                rc_vec: std::rc::Rc::new(vec![0]),
                arc_vec: std::sync::Arc::new(vec![0]),
                pinned_vec: Box::pin(vec![0]),
                nested_multiple_of: Box::new(Box::new(vec![3])),
                nested_min_length: Box::new(Box::new(vec!["x".to_owned()])),
                nested_pattern: Box::new(Box::new(vec!["123".to_owned()])),
                nested_min_properties: Box::new(Box::new(vec![HashMap::new()])),
                nested_enum: Box::new(Box::new(vec![3])),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        for field in [
            "cow_vec",
            "cow_option",
            "cow_hash_map",
            "cow_index_map",
            "cow_array",
            "nested_boxed_vec",
            "borrowed_boxed_vec",
            "boxed_borrowed_vec",
            "boxed_cow_slice",
            "rc_vec",
            "arc_vec",
            "pinned_vec",
            "nested_multiple_of",
            "nested_min_length",
            "nested_pattern",
            "nested_min_properties",
            "nested_enum",
        ] {
            assert!(errors["properties"].get(field).is_some(), "missing {field}");
        }
    }

    #[test]
    #[allow(clippy::owned_cow)]
    fn cow_container_composited_traits_are_available_directly() {
        use serde_valid::validation::ValidateCompositedMinimum;

        let vec: std::borrow::Cow<'_, Vec<i32>> = std::borrow::Cow::Owned(vec![1]);
        assert!(ValidateCompositedMinimum::validate_composited_minimum(&vec, 1).is_ok());

        let option: std::borrow::Cow<'_, Option<i32>> = std::borrow::Cow::Owned(Some(1));
        assert!(ValidateCompositedMinimum::validate_composited_minimum(&option, 1).is_ok());

        let hash_map: std::borrow::Cow<'_, HashMap<String, i32>> =
            std::borrow::Cow::Owned(HashMap::from([("value".to_owned(), 1)]));
        assert!(ValidateCompositedMinimum::validate_composited_minimum(&hash_map, 1).is_ok());

        let index_map: std::borrow::Cow<'_, IndexMap<String, i32>> =
            std::borrow::Cow::Owned(IndexMap::from([("value".to_owned(), 1)]));
        assert!(ValidateCompositedMinimum::validate_composited_minimum(&index_map, 1).is_ok());

        let array: std::borrow::Cow<'_, [i32; 1]> = std::borrow::Cow::Owned([1]);
        assert!(ValidateCompositedMinimum::validate_composited_minimum(&array, 1).is_ok());
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

    #[test]
    fn nested_maps_merge_errors_for_aliased_keys() {
        #[derive(Eq, Hash, PartialEq)]
        struct AliasedKey(u8);

        impl AsRef<str> for AliasedKey {
            fn as_ref(&self) -> &str {
                "same"
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

        for field in ["boxed_vec", "borrowed_vec", "cow_slice"] {
            assert_eq!(
                errors["properties"][field]["errors"]
                    .as_array()
                    .unwrap()
                    .len(),
                3
            );
        }
    }

    #[derive(Debug, Validate)]
    struct AutoderefArrayValidatorWrappers {
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
    fn autoderef_wrappers_support_array_validators() {
        let errors = serde_json::to_value(
            AutoderefArrayValidatorWrappers {
                rc_vec: std::rc::Rc::new(vec![1, 1]),
                arc_vec: std::sync::Arc::new(vec![1, 1]),
                pinned_vec: Box::pin(vec![1, 1]),
                custom_vec: InherentMethodShadow(vec![1, 1]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        for field in ["rc_vec", "arc_vec", "pinned_vec", "custom_vec"] {
            assert_eq!(
                errors["properties"][field]["errors"]
                    .as_array()
                    .unwrap()
                    .len(),
                3
            );
        }
    }

    #[derive(Debug)]
    struct InherentMethodShadow<T>(T);

    impl<T> std::ops::Deref for InherentMethodShadow<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

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
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::EnumError>> {
            Ok(())
        }

        fn validate_composited_multiple_of<A>(
            &self,
            _multiple_of: A,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MultipleOfError>> {
            Ok(())
        }

        fn validate_composited_minimum<A>(
            &self,
            _minimum: A,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinimumError>> {
            Ok(())
        }

        fn validate_composited_maximum<A>(
            &self,
            _maximum: A,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MaximumError>> {
            Ok(())
        }

        fn validate_composited_exclusive_minimum<A>(
            &self,
            _minimum: A,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::ExclusiveMinimumError>>
        {
            Ok(())
        }

        fn validate_composited_exclusive_maximum<A>(
            &self,
            _maximum: A,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::ExclusiveMaximumError>>
        {
            Ok(())
        }

        fn validate_composited_min_properties(
            &self,
            _minimum: usize,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinPropertiesError>>
        {
            Ok(())
        }

        fn validate_composited_max_properties(
            &self,
            _maximum: usize,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MaxPropertiesError>>
        {
            Ok(())
        }

        fn validate_composited_min_length(
            &self,
            _minimum: usize,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinLengthError>> {
            Ok(())
        }

        fn validate_composited_max_length(
            &self,
            _maximum: usize,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MaxLengthError>> {
            Ok(())
        }

        fn validate_composited_pattern(
            &self,
            _pattern: &serde_valid::export::regex::Regex,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::PatternError>> {
            Ok(())
        }
    }

    #[derive(Debug, Validate)]
    struct InherentMethodShadowConstraints {
        #[validate]
        nested: InherentMethodShadow<WrappedChild>,
        #[validate(r#enum = [1, 2])]
        enum_values: InherentMethodShadow<Vec<i32>>,
        #[validate(multiple_of = 2)]
        multiple_of: InherentMethodShadow<Vec<i32>>,
        #[validate(minimum = 1)]
        minimum: InherentMethodShadow<Vec<i32>>,
        #[validate(maximum = 1)]
        maximum: InherentMethodShadow<Vec<i32>>,
        #[validate(exclusive_minimum = 1)]
        exclusive_minimum: InherentMethodShadow<Vec<i32>>,
        #[validate(exclusive_maximum = 1)]
        exclusive_maximum: InherentMethodShadow<Vec<i32>>,
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
                enum_values: InherentMethodShadow(vec![3]),
                multiple_of: InherentMethodShadow(vec![3]),
                minimum: InherentMethodShadow(vec![0]),
                maximum: InherentMethodShadow(vec![2]),
                exclusive_minimum: InherentMethodShadow(vec![1]),
                exclusive_maximum: InherentMethodShadow(vec![1]),
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

        for field in [
            "nested",
            "enum_values",
            "multiple_of",
            "minimum",
            "maximum",
            "exclusive_minimum",
            "exclusive_maximum",
            "min_properties",
            "max_properties",
            "min_length",
            "max_length",
            "pattern",
        ] {
            assert!(errors["properties"].get(field).is_some(), "missing {field}");
        }
    }

    struct StaticEnumCandidates;

    impl serde_valid::validation::ValidateCompositedEnum<&'static [i32]> for StaticEnumCandidates {
        fn validate_composited_enum(
            &self,
            candidates: &'static [i32],
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::EnumError>> {
            Err(serde_valid::validation::Composited::Single(
                serde_valid::EnumError::new(candidates),
            ))
        }
    }

    #[derive(Validate)]
    struct LifetimeSpecificEnumConstraint {
        #[validate(r#enum = [1, 2])]
        value: StaticEnumCandidates,
    }

    #[test]
    fn enum_autoderef_preserves_lifetime_specific_trait_impls() {
        assert!(LifetimeSpecificEnumConstraint {
            value: StaticEnumCandidates,
        }
        .validate()
        .is_err());
    }

    #[derive(Validate)]
    struct DstAutoderefConstraints {
        #[validate(min_length = 1)]
        min_length: std::rc::Rc<str>,
        #[validate(pattern = "^[a-z]+$")]
        pattern: std::rc::Rc<str>,
        #[validate(r#enum = ["allowed"])]
        enum_value: std::rc::Rc<str>,
    }

    #[test]
    fn autoderef_supports_dynamically_sized_targets() {
        let errors = serde_json::to_value(
            DstAutoderefConstraints {
                min_length: std::rc::Rc::from(""),
                pattern: std::rc::Rc::from("123"),
                enum_value: std::rc::Rc::from("denied"),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        for field in ["min_length", "pattern", "enum_value"] {
            assert!(
                errors["properties"].get(field).is_some(),
                "missing {field}: {errors}"
            );
        }
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

        for field in ["boxed_vec", "borrowed_vec", "cow_slice"] {
            assert!(errors["properties"].get(field).is_some(), "missing {field}");
        }
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
        assert!(IndexMapPropertyConstraints {
            values: IndexMap::new(),
        }
        .validate()
        .is_err());
        assert!(IndexMapPropertyConstraints {
            values: IndexMap::from([("one".to_owned(), 1), ("two".to_owned(), 2)]),
        }
        .validate()
        .is_err());
    }

    #[derive(Debug, Eq, Hash, PartialEq)]
    struct AliasedKey(u8);

    impl AsRef<str> for AliasedKey {
        fn as_ref(&self) -> &str {
            "same"
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
        let hash_map_errors = serde_json::to_value(hash_map.validate().unwrap_err()).unwrap();
        let errors = hash_map_errors["properties"]["same"]["errors"]
            .as_array()
            .unwrap();
        assert!(errors.contains(&json!("array error")));
        assert!(errors.contains(&json!("object error")));
        assert!(hash_map_errors["properties"]["same"]["properties"]["property"].is_object());
    }

    #[derive(Debug)]
    struct MixedCompositedShape(bool);

    impl serde_valid::validation::ValidateCompositedMinimum<i32> for MixedCompositedShape {
        fn validate_composited_minimum(
            &self,
            minimum: i32,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MinimumError>> {
            use serde_valid::validation::Composited;

            if self.0 {
                Err(Composited::Object(IndexMap::from([(
                    "property".into(),
                    vec![Composited::Single(serde_valid::MinimumError::new(minimum))],
                )])))
            } else {
                Err(Composited::Array(IndexMap::from([(
                    0,
                    Composited::Single(serde_valid::MinimumError::new(minimum)),
                )])))
            }
        }
    }

    impl serde_valid::validation::ValidateCompositedMaximum<i32> for MixedCompositedShape {
        fn validate_composited_maximum(
            &self,
            maximum: i32,
        ) -> Result<(), serde_valid::validation::Composited<serde_valid::MaximumError>> {
            use serde_valid::validation::Composited;

            Err(Composited::Object(IndexMap::from([(
                "property".into(),
                vec![Composited::Single(serde_valid::MaximumError::new(maximum))],
            )])))
        }
    }

    #[derive(Debug, Validate)]
    struct MixedCompositedConstraints {
        #[validate(minimum = 1)]
        #[validate(maximum = 2)]
        direct: MixedCompositedShape,
        #[validate(minimum = 1)]
        aliased: IndexMap<AliasedKey, MixedCompositedShape>,
    }

    #[test]
    fn mixed_composited_error_shapes_use_object_precedence_without_panicking() {
        let errors = serde_json::to_value(
            MixedCompositedConstraints {
                direct: MixedCompositedShape(false),
                aliased: IndexMap::from([
                    (AliasedKey(1), MixedCompositedShape(false)),
                    (AliasedKey(2), MixedCompositedShape(true)),
                ]),
            }
            .validate()
            .unwrap_err(),
        )
        .unwrap();

        assert!(errors["properties"]["direct"]["items"].is_null());
        assert_eq!(
            errors["properties"]["direct"]["properties"]["property"]["errors"],
            json!(["The number must be `<= 2`."])
        );
        assert!(errors["properties"]["aliased"]["properties"]["same"]["items"].is_null());
        assert_eq!(
            errors["properties"]["aliased"]["properties"]["same"]["properties"]["property"]
                ["errors"],
            json!(["The number must be `>= 1`."])
        );
    }
}
