use serde_valid::composited::{Composited, ValidateCompositedEnum, ValidateCompositedMinimum};
use serde_valid::validation::IntoError;
use serde_valid::{MinimumError, Validate, ValidateEnum, ValidateMinimum};
use std::collections::{BTreeMap, HashMap};

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

#[test]
fn public_composited_path_markers_are_nameable() {
    fn assert_path<P>() {}

    assert_path::<serde_valid::composited::path::Scalar>();
    assert_path::<serde_valid::composited::path::Sequence<()>>();
}

#[test]
fn scalar_and_composited_results_remain_distinct() {
    let scalar: Result<(), MinimumError> = CustomNumber(0).validate_minimum(1);
    assert!(scalar.is_err());

    let composited: Result<(), Composited<MinimumError>> =
        ValidateCompositedMinimum::validate_composited_minimum(&CustomNumber(0), 1);
    assert!(matches!(composited, Err(Composited::Single(_))));
}

#[test]
fn a_scalar_implementation_is_automatically_available_to_nested_sequences() {
    let values = vec![[CustomNumber(0)]];
    let error = ValidateCompositedMinimum::validate_composited_minimum(&values, 1)
        .unwrap_err()
        .into_error();
    let error = serde_json::to_value(error).unwrap();

    assert!(error["items"]["0"]["items"]["0"].is_object());
}

#[derive(Clone, Debug)]
struct Candidate(i32);

impl From<Candidate> for serde_valid::validation::Literal {
    fn from(value: Candidate) -> Self {
        value.0.into()
    }
}

impl ValidateEnum<Candidate> for CustomNumber {
    fn validate_enum(&self, candidates: &[Candidate]) -> Result<(), serde_valid::EnumError> {
        if candidates.iter().any(|candidate| candidate.0 == self.0) {
            Ok(())
        } else {
            Err(serde_valid::EnumError::new(candidates))
        }
    }
}

#[test]
fn enum_candidates_are_composited_without_an_extra_user_impl() {
    let values = vec![CustomNumber(2)];
    assert!(ValidateCompositedEnum::validate_composited_enum(&values, &[Candidate(1)]).is_err());
}

#[test]
fn maps_use_the_sequence_path_and_preserve_real_property_keys() {
    fn assert_sequence<
        T: ValidateCompositedMinimum<
            i32,
            serde_valid::composited::path::Sequence<serde_valid::composited::path::Scalar>,
        >,
    >() {
    }

    assert_sequence::<HashMap<String, CustomNumber>>();
    assert_sequence::<BTreeMap<String, CustomNumber>>();
    assert_sequence::<indexmap::IndexMap<String, CustomNumber>>();

    let values = BTreeMap::from([("actual-key".to_owned(), vec![CustomNumber(0)])]);
    let error = ValidateCompositedMinimum::validate_composited_minimum(&values, 1)
        .unwrap_err()
        .into_error();
    let error = serde_json::to_value(error).unwrap();

    assert!(error["properties"]["actual-key"]["items"]["0"].is_object());
}

#[derive(Validate)]
struct Derived {
    #[validate(minimum = 1)]
    values: Vec<Vec<i32>>,
}

#[test]
fn derive_uses_the_inferred_sequence_path() {
    let value = Derived {
        values: vec![vec![0]],
    };
    let error = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(error["properties"]["values"]["items"]["0"]["items"]["0"].is_object());
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
