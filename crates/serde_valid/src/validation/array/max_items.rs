/// Max length validation of the array items.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{traits::Items, Validate};
///
/// struct MyType(Vec<i32>);
///
/// impl Items for MyType {
///     fn items(&self) -> usize {
///         self.0.items()
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(max_items = 2)]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType(vec![1, 2, 3]),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The length of the items must be `<= 2`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMaxItems {
    fn validate_max_items(&self, max_items: usize) -> Result<(), crate::MaxItemsError>;
}

impl<T> ValidateMaxItems for T
where
    T: crate::traits::Items + ?Sized,
{
    fn validate_max_items(&self, max_items: usize) -> Result<(), crate::MaxItemsError> {
        if max_items >= self.items() {
            Ok(())
        } else {
            Err(crate::MaxItemsError::new(max_items))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(ValidateMaxItems::validate_max_items(&vec!['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_array_type() {
        assert!(ValidateMaxItems::validate_max_items(&['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_is_true() {
        assert!(ValidateMaxItems::validate_max_items(&[1, 2, 3], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_is_false() {
        assert!(ValidateMaxItems::validate_max_items(&[1, 2, 3], 2).is_err());
    }
}
