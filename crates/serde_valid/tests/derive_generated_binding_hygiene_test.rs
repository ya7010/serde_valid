#![allow(dead_code, non_upper_case_globals)]

use serde_valid::Validate;

static field: () = ();
static errors: () = ();
static error: () = ();
static other: () = ();
static index: () = ();
static __field_errors: () = ();
static __array_errors: () = ();
static __object_errors: () = ();
static error_params: &str = "custom minimum";
static value: () = ();
static __0: () = ();

#[derive(Validate)]
struct Input {
    value: String,
}

#[derive(Validate)]
struct TupleInput(#[validate(min_length = 1)] String);

#[derive(Validate)]
struct InputWithMessage {
    #[validate(min_items = 1, message_fn = |_| error_params.to_owned())]
    values: Vec<u8>,
}

#[test]
fn generated_error_bindings_do_not_collide_with_user_statics() {
    assert!(Input {
        value: String::new(),
    }
    .validate()
    .is_ok());
    assert!(TupleInput("value".to_owned()).validate().is_ok());
}

#[test]
fn custom_message_expressions_resolve_user_statics() {
    let validation_errors = InputWithMessage { values: vec![] }.validate().unwrap_err();
    assert!(validation_errors.to_string().contains(error_params));
}
