use serde_json::json;
use serde_valid::Validate;

fn always_invalid(_value: &Input) -> Result<(), serde_valid::validation::Error> {
    Err(serde_valid::validation::Error::Custom(
        "invalid enum".to_owned(),
    ))
}

#[derive(Validate)]
#[validate(custom = always_invalid)]
enum Input {
    Unit,
    Tuple(u8),
}

#[test]
fn enum_level_custom_validation_runs_for_unit_variants() {
    assert_eq!(
        serde_json::to_value(Input::Unit.validate().unwrap_err()).unwrap(),
        json!({ "errors": ["invalid enum"] })
    );
}

#[test]
fn enum_level_custom_validation_still_runs_for_data_variants() {
    assert_eq!(
        serde_json::to_value(Input::Tuple(0).validate().unwrap_err()).unwrap(),
        json!({ "errors": ["invalid enum"] })
    );
}
