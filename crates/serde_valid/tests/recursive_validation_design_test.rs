use serde_valid::traits::Sequence;
use serde_valid::{MinimumError, Validate, ValidateEnum, ValidateMinimum};
use std::collections::BTreeMap;

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
struct NestedSequences {
    #[validate(minimum = 1)]
    values: Vec<[CustomNumber; 1]>,
}

#[test]
fn a_scalar_implementation_is_automatically_available_to_nested_sequences() {
    let value = NestedSequences {
        values: vec![[CustomNumber(0)]],
    };
    let error = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(error["properties"]["values"]["items"]["0"]["items"]["0"].is_object());
}

struct CustomString(String);

impl ValidateEnum<&'static str> for CustomString {
    fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), serde_valid::EnumError> {
        self.0.as_str().validate_enum(candidates)
    }
}

#[derive(Validate)]
struct EnumeratedSequence {
    #[validate(r#enum = ["allowed"])]
    values: Vec<CustomString>,
}

#[test]
fn enum_candidates_are_composited_without_an_extra_user_impl() {
    let value = EnumeratedSequence {
        values: vec![CustomString("denied".to_owned())],
    };
    let error = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(error["properties"]["values"]["items"]["0"].is_object());
}

#[derive(Validate)]
struct MapSequence {
    #[validate(minimum = 1)]
    values: BTreeMap<String, Vec<CustomNumber>>,
}

#[test]
fn maps_preserve_real_property_keys() {
    let value = MapSequence {
        values: BTreeMap::from([("actual-key".to_owned(), vec![CustomNumber(0)])]),
    };
    let error = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(error["properties"]["values"]["properties"]["actual-key"]["items"]["0"].is_object());
}

struct MyVec<T>(Vec<T>);

impl<T> Sequence for MyVec<T> {
    type Item = T;

    fn for_each(&self, visitor: impl FnMut(&Self::Item)) {
        self.0.iter().for_each(visitor);
    }
}

#[derive(Validate)]
struct CustomSequences {
    #[validate(min_length = 2)]
    names: MyVec<String>,
    #[validate(minimum = 1)]
    numbers: MyVec<MyVec<i32>>,
}

#[test]
fn one_sequence_implementation_enables_all_composited_rules() {
    let value = CustomSequences {
        names: MyVec(vec!["x".to_owned()]),
        numbers: MyVec(vec![MyVec(vec![0])]),
    };
    let error = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(error["properties"]["names"]["items"]["0"].is_object());
    assert!(error["properties"]["numbers"]["items"]["0"]["items"]["0"].is_object());
}

#[derive(Validate)]
struct Child {
    #[validate(minimum = 1)]
    value: i32,
}

#[derive(Validate)]
struct WrappedChildren {
    #[validate]
    boxed: Box<Child>,
    #[validate]
    optional: Option<Child>,
}

#[test]
fn box_and_option_delegate_nested_validation() {
    let value = WrappedChildren {
        boxed: Box::new(Child { value: 0 }),
        optional: Some(Child { value: 0 }),
    };
    let error = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(error["properties"]["boxed"]["properties"]["value"].is_object());
    assert!(error["properties"]["optional"]["properties"]["value"].is_object());
}
