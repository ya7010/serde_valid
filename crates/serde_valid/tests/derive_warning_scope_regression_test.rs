#![allow(dead_code)]
#![deny(deprecated)]

use serde_valid::Validate;

#[derive(Validate)]
struct NamedStruct {
    #[allow(deprecated)]
    #[validate(enumerate = ["a"])]
    value: String,
}

#[derive(Validate)]
struct TupleStruct(
    #[allow(deprecated)]
    #[validate(enumerate = ["a"])]
    String,
);

#[derive(Validate)]
enum Enum {
    Named {
        #[allow(deprecated)]
        #[validate(enumerate = ["a"])]
        value: String,
    },
    Tuple(
        #[allow(deprecated)]
        #[validate(enumerate = ["a"])]
        String,
    ),
}

#[test]
fn field_lint_allowances_apply_to_generated_deprecation_warnings() {
    assert!(NamedStruct {
        value: "a".to_owned()
    }
    .validate()
    .is_ok());
    assert!(TupleStruct("a".to_owned()).validate().is_ok());
    assert!(Enum::Named {
        value: "a".to_owned()
    }
    .validate()
    .is_ok());
    assert!(Enum::Tuple("a".to_owned()).validate().is_ok());
}
