#![allow(clippy::redundant_allocation)]

use serde_valid::Validate;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Validate)]
struct Child {
    #[validate(minimum = 1)]
    value: i32,
}

#[derive(Validate)]
struct NestedValidationWrappers<'a> {
    #[validate]
    triple_boxed: Box<Box<Box<Child>>>,
    #[validate]
    rc_option: Rc<Option<Child>>,
    #[validate]
    arc: Arc<Child>,
    #[validate]
    pinned: Pin<Box<Child>>,
    #[validate]
    rc_borrowed: Rc<&'a Child>,
}

#[test]
fn derive_delegates_nested_validation_through_standard_wrappers() {
    let borrowed = Child { value: 0 };
    let errors = serde_json::to_value(
        NestedValidationWrappers {
            triple_boxed: Box::new(Box::new(Box::new(Child { value: 0 }))),
            rc_option: Rc::new(Some(Child { value: 0 })),
            arc: Arc::new(Child { value: 0 }),
            pinned: Box::pin(Child { value: 0 }),
            rc_borrowed: Rc::new(&borrowed),
        }
        .validate()
        .unwrap_err(),
    )
    .unwrap();

    for field in ["triple_boxed", "rc_option", "arc", "pinned", "rc_borrowed"] {
        assert!(
            errors["properties"].get(field).is_some(),
            "missing validation error for {field}: {errors}"
        );
    }
}
