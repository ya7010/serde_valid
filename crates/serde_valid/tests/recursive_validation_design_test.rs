use serde_valid::validation::{Composited, IntoError, ValidateCompositedEnum};
use serde_valid::validation::{ValidateCompositedMinProperties, ValidateCompositedMinimum};
use serde_valid::{MinimumError, Validate, ValidateEnum, ValidateMinProperties, ValidateMinimum};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

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
fn scalar_and_composited_results_remain_distinct() {
    let scalar: Result<(), MinimumError> = CustomNumber(0).validate_minimum(1);
    assert!(scalar.is_err());

    let composited: Result<(), Composited<MinimumError>> =
        ValidateCompositedMinimum::validate_composited_minimum(&CustomNumber(0), 1);
    assert!(matches!(composited, Err(Composited::Single(_))));
}

#[test]
fn a_scalar_implementation_is_automatically_available_to_arbitrary_composition() {
    let values = Pin::new(Rc::new(Arc::new(Box::new(vec![Box::new(Some(
        CustomNumber(0),
    ))]))));

    let error = ValidateCompositedMinimum::validate_composited_minimum(&values, 1)
        .unwrap_err()
        .into_error();
    let error = serde_json::to_value(error).unwrap();

    assert!(error["items"]["0"].is_object());
}

#[test]
fn previously_missing_standard_wrapper_shapes_are_supported() {
    macro_rules! assert_invalid {
        ($value:expr) => {
            assert!(ValidateCompositedMinimum::validate_composited_minimum(&$value, 1).is_err())
        };
    }

    assert_invalid!(Arc::new(Some(0)));
    assert_invalid!(Rc::new([0]));
    assert_invalid!(Rc::new(HashMap::from([("key".to_owned(), 0)])));
    assert_invalid!(Rc::new(indexmap::IndexMap::from([("key".to_owned(), 0)])));
    assert_invalid!(Arc::<[i32]>::from([0]));
    assert_invalid!(Arc::new([0]));
    assert_invalid!(Arc::new(indexmap::IndexMap::from([("key".to_owned(), 0)])));
    assert_invalid!(Pin::new(Box::<[i32]>::from([0])));
    assert_invalid!(Pin::new(Box::new([0])));
    assert_invalid!(Pin::new(Box::new(HashMap::from([("key".to_owned(), 0)]))));
    assert_invalid!(Pin::new(Box::new(indexmap::IndexMap::from([(
        "key".to_owned(),
        0,
    )]))));
    assert_invalid!(Pin::new(Rc::new(vec![0])));
    assert_invalid!(Pin::new(Arc::new(vec![0])));
}

#[test]
fn every_previously_supported_wrapper_shape_remains_composited() {
    macro_rules! assert_minimum_invalid {
        ($value:expr) => {
            assert!(ValidateCompositedMinimum::validate_composited_minimum(&$value, 1).is_err());
        };
    }

    macro_rules! assert_max_length_invalid {
        ($value:expr) => {
            assert!(
                serde_valid::validation::ValidateCompositedMaxLength::validate_composited_max_length(
                    &$value, 1,
                )
                .is_err()
            );
        };
    }

    macro_rules! assert_all_shapes {
        ($assertion:ident, $value:expr) => {
            $assertion!(&[$value][..]);
            $assertion!(vec![$value].into_boxed_slice());
            $assertion!(&[$value]);
            $assertion!(Box::new([$value]));
            $assertion!(&vec![$value]);
            $assertion!(Box::new(vec![$value]));
            $assertion!(Box::new(Box::new(vec![$value])));
            $assertion!(&Box::new(vec![$value]));
            $assertion!(Box::new(&vec![$value]));
            $assertion!(Rc::new(vec![$value]));
            $assertion!(Arc::new(vec![$value]));
            $assertion!(Pin::new(Box::new(vec![$value])));
            $assertion!(&Some($value));
            $assertion!(Box::new(Some($value)));
            $assertion!(&HashMap::from([("key".to_owned(), $value)]));
            $assertion!(Box::new(HashMap::from([("key".to_owned(), $value)])));
            $assertion!(&indexmap::IndexMap::from([("key".to_owned(), $value)]));
            $assertion!(Box::new(indexmap::IndexMap::from([(
                "key".to_owned(),
                $value,
            )])));
            $assertion!(Cow::<Vec<_>>::Owned(vec![$value]));
            $assertion!(Cow::<Option<_>>::Owned(Some($value)));
            $assertion!(Cow::<[_; 1]>::Owned([$value]));
            $assertion!(Cow::<HashMap<String, _>>::Owned(HashMap::from([(
                "key".to_owned(),
                $value,
            )])));
            $assertion!(Cow::<indexmap::IndexMap<String, _>>::Owned(
                indexmap::IndexMap::from([("key".to_owned(), $value)]),
            ));
            $assertion!(Cow::<[_]>::Owned(vec![$value]));
            $assertion!(Box::new(Cow::<[_]>::Owned(vec![$value])));
        };
    }

    assert_all_shapes!(assert_minimum_invalid, 0_i32);
    assert_all_shapes!(assert_max_length_invalid, "too long".to_owned());
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
    let values = Pin::new(Arc::new(vec![Box::new(CustomNumber(2))]));
    assert!(ValidateCompositedEnum::validate_composited_enum(&values, &[Candidate(1)]).is_err());
}

#[test]
fn object_validation_uses_scalar_semantics_for_a_wrapped_map() {
    let raw = HashMap::from([("key".to_owned(), 0)]);
    let scalar: Result<(), serde_valid::MinPropertiesError> = raw.validate_min_properties(2);
    assert!(scalar.is_err());

    let map = Pin::new(Rc::new(raw));
    assert!(matches!(
        ValidateCompositedMinProperties::validate_composited_min_properties(&map, 2),
        Err(Composited::Single(_))
    ));
}

#[test]
fn btree_map_recursion_preserves_real_property_keys() {
    let values = BTreeMap::from([("actual-key".to_owned(), vec![0])]);
    let error = ValidateCompositedMinimum::validate_composited_minimum(&values, 1)
        .unwrap_err()
        .into_error();
    let error = serde_json::to_value(error).unwrap();

    assert!(error["properties"]["actual-key"]["items"]["0"].is_object());
}

type DeepValues = Pin<Rc<Arc<Box<Vec<Box<Option<i32>>>>>>>;

#[derive(Validate)]
struct Derived {
    #[validate(minimum = 1)]
    values: DeepValues,
}

#[test]
fn derive_uses_the_inferred_recursive_path() {
    let value = Derived {
        values: Pin::new(Rc::new(Arc::new(Box::new(vec![Box::new(Some(0))])))),
    };
    let error = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(error["properties"]["values"]["items"]["0"].is_object());
}
