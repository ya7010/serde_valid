#![allow(dead_code)]

use serde_valid::{Validate, ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Validate)]
struct Child {
    #[validate(min_length = 2)]
    value: String,
}

#[derive(Debug, Validate)]
struct NumericKeyMap {
    #[validate(min_length = 2)]
    values: HashMap<u32, String>,
}

#[test]
fn composited_map_validation_accepts_keys_convertible_to_strings() {
    let errors = serde_json::to_value(
        NumericKeyMap {
            values: HashMap::from([(7, "x".to_owned())]),
        }
        .validate()
        .unwrap_err(),
    )
    .unwrap();

    assert!(errors["properties"]["values"]["properties"]["7"].is_object());
}

#[derive(Debug, Validate)]
struct BTreeMapParent {
    #[validate]
    children: BTreeMap<String, Child>,
}

#[test]
fn nested_validation_supports_btree_map_and_preserves_its_keys() {
    let errors = serde_json::to_value(
        BTreeMapParent {
            children: BTreeMap::from([(
                "actual-key".to_owned(),
                Child {
                    value: "x".to_owned(),
                },
            )]),
        }
        .validate()
        .unwrap_err(),
    )
    .unwrap();

    assert!(
        errors["properties"]["children"]["properties"]["actual-key"]["properties"]["value"]
            .is_object()
    );
}

fn assert_invalid<T: Validate>(value: T) {
    assert!(value.validate().is_err());
}

fn assert_not_unique<T: ValidateUniqueItems>(value: T) {
    assert!(value.validate_unique_items().is_err());
}

fn assert_invalid_item_count<T: ValidateMaxItems + ValidateMinItems>(value: T) {
    assert!(value.validate_min_items(3).is_err());
    assert!(value.validate_max_items(1).is_err());
}

#[test]
fn mutable_references_forward_validation_traits() {
    let mut child = Child {
        value: "x".to_owned(),
    };
    assert_invalid(&mut child);

    let mut values = [1, 1];
    assert_not_unique(&mut values[..]);
    assert_invalid_item_count(&mut values[..]);
}
