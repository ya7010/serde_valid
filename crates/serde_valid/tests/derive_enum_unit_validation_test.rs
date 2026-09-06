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
    assert!(Input::Unit.validate().is_err());
}

#[test]
fn enum_level_custom_validation_still_runs_for_data_variants() {
    assert!(Input::Tuple(0).validate().is_err());
}
