#![allow(dead_code)]
#![deny(unfulfilled_lint_expectations)]

#[expect(deprecated)]
#[derive(serde_valid::Validate)]
enum NamedVariant {
    Value {
        #[validate(enumerate = ["a"])]
        value: String,
    },
}

#[expect(deprecated)]
#[derive(serde_valid::Validate)]
enum TupleVariant {
    Value(#[validate(enumerate = ["a"])] String),
}

#[test]
fn deprecated_enumerate_warnings_are_emitted_for_enum_fields() {
    assert!(serde_valid::Validate::validate(&NamedVariant::Value {
        value: "a".to_owned(),
    })
    .is_ok());
    assert!(serde_valid::Validate::validate(&TupleVariant::Value("a".to_owned())).is_ok());
}
