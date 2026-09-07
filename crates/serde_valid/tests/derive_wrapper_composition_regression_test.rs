#![allow(
    clippy::borrowed_box,
    clippy::box_collection,
    clippy::redundant_allocation,
    clippy::type_complexity
)]

use serde_valid::Validate;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Validate)]
struct AdditionalCompositedWrappers<'a> {
    #[validate(minimum = 1)]
    triple_boxed_vec: Box<Box<Box<Vec<i32>>>>,
    #[validate(minimum = 1)]
    deeply_boxed_vec: Box<Box<Box<Box<Box<Vec<i32>>>>>>,
    #[validate(minimum = 1)]
    mixed_wrappers: Box<Rc<Arc<Pin<Box<Vec<i32>>>>>>,
    #[validate(minimum = 1)]
    rc_option: Rc<Option<i32>>,
    #[validate(minimum = 1)]
    arc_hash_map: Arc<HashMap<String, i32>>,
    #[validate(minimum = 1)]
    rc_slice: Rc<[i32]>,
    #[validate(minimum = 1)]
    pinned_option: Pin<Box<Option<i32>>>,
    #[validate(min_length = 2)]
    pinned_str: Pin<Box<str>>,
    #[validate(min_length = 2)]
    pinned_string: Pin<Box<String>>,
    #[validate(min_length = 2)]
    pinned_rc_str: Pin<Rc<str>>,
    #[validate(min_length = 2)]
    pinned_arc_string: Pin<Arc<String>>,
    #[validate(minimum = 1)]
    rc_borrowed_vec: Rc<&'a Vec<i32>>,
}

#[test]
fn composited_validation_supports_additional_standard_wrapper_shapes() {
    let borrowed_vec = vec![0];
    let errors = serde_json::to_value(
        AdditionalCompositedWrappers {
            triple_boxed_vec: Box::new(Box::new(Box::new(vec![0]))),
            deeply_boxed_vec: Box::new(Box::new(Box::new(Box::new(Box::new(vec![0]))))),
            mixed_wrappers: Box::new(Rc::new(Arc::new(Box::pin(vec![0])))),
            rc_option: Rc::new(Some(0)),
            arc_hash_map: Arc::new(HashMap::from([("value".to_owned(), 0)])),
            rc_slice: Rc::from([0]),
            pinned_option: Box::pin(Some(0)),
            pinned_str: Pin::from(Box::<str>::from("x")),
            pinned_string: Box::pin("x".to_owned()),
            pinned_rc_str: Pin::new(Rc::<str>::from("x")),
            pinned_arc_string: Pin::new(Arc::new("x".to_owned())),
            rc_borrowed_vec: Rc::new(&borrowed_vec),
        }
        .validate()
        .unwrap_err(),
    )
    .unwrap();

    for field in [
        "triple_boxed_vec",
        "deeply_boxed_vec",
        "mixed_wrappers",
        "rc_option",
        "arc_hash_map",
        "rc_slice",
        "pinned_option",
        "pinned_str",
        "pinned_string",
        "pinned_rc_str",
        "pinned_arc_string",
        "rc_borrowed_vec",
    ] {
        assert!(
            errors["properties"].get(field).is_some(),
            "missing validation error for {field}: {errors}"
        );
    }
}
