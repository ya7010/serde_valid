/// Min length validation of the array items.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMinItems};
///
/// struct MyType(Vec<i32>);
///
/// impl ValidateMinItems for MyType {
///     fn validate_min_items(
///         &self,
///         min_items: usize,
///     ) -> Result<(), serde_valid::MinItemsError> {
///         self.0.validate_min_items(min_items)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(min_items = 2)]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType(vec![1]),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The length of the items must be `>= 2`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMinItems {
    fn validate_min_items(&self, min_items: usize) -> Result<(), crate::MinItemsError>;
}

impl<T> ValidateMinItems for Vec<T> {
    fn validate_min_items(&self, min_items: usize) -> Result<(), crate::MinItemsError> {
        if min_items <= self.len() {
            Ok(())
        } else {
            Err(crate::MinItemsError::new(min_items))
        }
    }
}

impl<T> ValidateMinItems for [T] {
    fn validate_min_items(&self, min_items: usize) -> Result<(), crate::MinItemsError> {
        if min_items <= self.len() {
            Ok(())
        } else {
            Err(crate::MinItemsError::new(min_items))
        }
    }
}

impl<T, const N: usize> ValidateMinItems for [T; N] {
    fn validate_min_items(&self, min_items: usize) -> Result<(), crate::MinItemsError> {
        if min_items <= self.len() {
            Ok(())
        } else {
            Err(crate::MinItemsError::new(min_items))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_min_items_is_true() {
        assert!(ValidateMinItems::validate_min_items(&[1, 2, 3], 3).is_ok());
    }

    #[test]
    fn test_validate_array_min_items_is_false() {
        assert!(ValidateMinItems::validate_min_items(&[1, 2, 3], 4).is_err());
    }

    #[test]
    fn test_validate_array_min_items_vec_is_true() {
        assert!(ValidateMinItems::validate_min_items(&vec!['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_min_items_array_is_true() {
        assert!(ValidateMinItems::validate_min_items(&['a', 'b', 'c'], 3).is_ok());
    }
}
