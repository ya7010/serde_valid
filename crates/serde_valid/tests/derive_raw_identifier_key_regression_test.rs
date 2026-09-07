use serde::Serialize;
use serde_valid::Validate;

#[derive(Serialize, Validate)]
struct RawIdentifierField {
    #[validate(min_length = 2)]
    r#type: String,
}

#[derive(Serialize, Validate)]
struct RenamedRawIdentifierField {
    #[serde(rename = "kind")]
    #[validate(min_length = 2)]
    r#type: String,
}

#[test]
fn validation_error_key_matches_the_serialized_raw_identifier_key() {
    let value = RawIdentifierField {
        r#type: "x".to_owned(),
    };

    let serialized = serde_json::to_value(&value).unwrap();
    let errors = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(serialized.get("type").is_some());
    assert!(errors["properties"].get("type").is_some(), "{errors}");
    assert!(errors["properties"].get("r#type").is_none(), "{errors}");
}

#[test]
fn explicit_serde_rename_still_applies_to_a_raw_identifier() {
    let value = RenamedRawIdentifierField {
        r#type: "x".to_owned(),
    };

    let serialized = serde_json::to_value(&value).unwrap();
    let errors = serde_json::to_value(value.validate().unwrap_err()).unwrap();

    assert!(serialized.get("kind").is_some());
    assert!(errors["properties"].get("kind").is_some(), "{errors}");
    assert!(errors["properties"].get("type").is_none(), "{errors}");
    assert!(errors["properties"].get("r#type").is_none(), "{errors}");
}
