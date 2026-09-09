use serde::Serialize;
use serde_json::json;
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

    assert_eq!(
        serde_json::to_value(&value).unwrap(),
        json!({ "type": "x" })
    );
    assert_eq!(
        serde_json::to_value(value.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "type": { "errors": ["The length of the value must be `>= 2`."] }
            }
        })
    );
}

#[test]
fn explicit_serde_rename_still_applies_to_a_raw_identifier() {
    let value = RenamedRawIdentifierField {
        r#type: "x".to_owned(),
    };

    assert_eq!(
        serde_json::to_value(&value).unwrap(),
        json!({ "kind": "x" })
    );
    assert_eq!(
        serde_json::to_value(value.validate().unwrap_err()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "kind": { "errors": ["The length of the value must be `>= 2`."] }
            }
        })
    );
}
