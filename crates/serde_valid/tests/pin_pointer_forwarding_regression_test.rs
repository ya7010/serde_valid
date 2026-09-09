use serde_valid::{Validate, ValidateExclusiveMinimum};
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn every_standard_pin_pointer_family_has_composited_forwarding() {
    let pattern = regex::Regex::new("^[a-z]+$").unwrap();
    let boxed = Pin::new(Box::<str>::from("allowed"));
    let rc = Pin::new(Rc::<str>::from("allowed"));
    let arc = Pin::new(Arc::<str>::from("allowed"));

    macro_rules! assert_string {
        ($value:expr) => {{
            assert!(serde_valid::__private::ValidateCompositedMinLength::validate_composited_min_length(&$value, 2).is_ok());
            assert!(serde_valid::__private::ValidateCompositedPattern::validate_composited_pattern(&$value, &pattern).is_ok());
            assert!(serde_valid::__private::ValidateCompositedEnum::validate_composited_enum(&$value, &["allowed"]).is_ok());
        }};
    }

    assert_string!(boxed);
    assert_string!(rc);
    assert_string!(arc);

    assert!(serde_valid::__private::ValidateCompositedMinProperties::validate_composited_min_properties(
        &Pin::new(Box::new(HashMap::<String, String>::new())), 1,
    ).is_err());
    assert!(serde_valid::__private::ValidateCompositedMinProperties::validate_composited_min_properties(
        &Pin::new(Rc::new(HashMap::<String, String>::new())), 1,
    ).is_err());
    assert!(serde_valid::__private::ValidateCompositedMinProperties::validate_composited_min_properties(
        &Pin::new(Arc::new(HashMap::<String, String>::new())), 1,
    ).is_err());
}

#[derive(Validate)]
struct PinnedPointerConstraints {
    #[validate(min_length = 2)]
    #[validate(pattern = "^[a-z]+$")]
    #[validate(r#enum = ["allowed"])]
    string: Pin<Rc<str>>,
    #[validate(min_properties = 1)]
    object: Pin<Arc<HashMap<String, String>>>,
}

#[test]
fn pin_forwards_every_scalar_validator_through_its_pointer_target() {
    let errors = serde_json::to_value(
        PinnedPointerConstraints {
            string: Pin::new(Rc::from("1")),
            object: Pin::new(Arc::new(HashMap::new())),
        }
        .validate()
        .unwrap_err(),
    )
    .unwrap();

    assert!(errors["properties"].get("string").is_some(), "{errors}");
    assert!(errors["properties"].get("object").is_some(), "{errors}");
}

struct CustomNumber(i32);

impl ValidateExclusiveMinimum<i32> for CustomNumber {
    fn validate_exclusive_minimum(
        &self,
        exclusive_minimum: i32,
    ) -> Result<(), serde_valid::ExclusiveMinimumError> {
        self.0.validate_exclusive_minimum(exclusive_minimum)
    }
}

#[derive(Validate)]
struct CustomScalarConstraint {
    #[validate(exclusive_minimum = 1)]
    value: CustomNumber,
}

#[test]
fn public_validator_impl_automatically_provides_composited_validation() {
    fn assert_composited<T: serde_valid::__private::ValidateCompositedExclusiveMinimum<i32>>() {}

    assert_composited::<CustomNumber>();
    assert!(CustomScalarConstraint {
        value: CustomNumber(1),
    }
    .validate()
    .is_err());
}
