use crate::{traits::Size, MinPropertiesError};

/// Min size validation of the object properties.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
///
/// ```rust
/// use std::collections::HashMap;
///
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMinProperties};
///
/// struct MyType(HashMap<String, String>);
///
/// impl ValidateMinProperties for MyType {
///     fn validate_min_properties(
///         &self,
///         min_properties: usize,
///     ) -> Result<(), serde_valid::MinPropertiesError> {
///         self.0.validate_min_properties(min_properties)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(min_properties = 2)]
///     val: MyType,
/// }
///
/// let mut map = HashMap::new();
/// map.insert("key1".to_string(), "value1".to_string());
///
/// let s = TestStruct { val: MyType(map) };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The size of the properties must be `>= 2`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMinProperties {
    fn validate_min_properties(&self, min_properties: usize) -> Result<(), MinPropertiesError>;
}

impl<T> ValidateMinProperties for T
where
    T: Size,
{
    fn validate_min_properties(&self, min_properties: usize) -> Result<(), MinPropertiesError> {
        if min_properties <= self.size() {
            Ok(())
        } else {
            Err(MinPropertiesError::new(min_properties))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    #[test]
    fn test_validate_object_min_properties_hash_map_type() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateMinProperties::validate_min_properties(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_min_properties_btree_map_type() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateMinProperties::validate_min_properties(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_min_properties_json_map_type() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateMinProperties::validate_min_properties(map, 2).is_ok());
        assert!(ValidateMinProperties::validate_min_properties(map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_min_properties_is_false() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateMinProperties::validate_min_properties(map, 4).is_err());
    }
}
