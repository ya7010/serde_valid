#![allow(clippy::redundant_allocation)]

use serde_json::json;
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
    let child_errors = json!({
        "errors": [],
        "properties": {
            "value": { "errors": ["The number must be `>= 1`."] }
        }
    });

    assert_eq!(
        serde_json::to_value(
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
        .unwrap(),
        json!({
            "errors": [],
            "properties": {
                "triple_boxed": child_errors,
                "rc_option": child_errors,
                "arc": child_errors,
                "pinned": child_errors,
                "rc_borrowed": child_errors,
            }
        })
    );
}
