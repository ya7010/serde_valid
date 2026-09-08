use crate::traits::IsUnique;

/// Uniqueness validation of the array items.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#unique_items>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateUniqueItems};
///
/// struct MyType(Vec<i32>);
///
/// impl ValidateUniqueItems for MyType {
///     fn validate_unique_items(&self) -> Result<(), serde_valid::UniqueItemsError> {
///         self.0.validate_unique_items()
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(unique_items)]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType(vec![1, 2, 1]),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The items must be unique."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateUniqueItems {
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError>;
}

impl<T> ValidateUniqueItems for &T
where
    T: ValidateUniqueItems + ?Sized,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        (*self).validate_unique_items()
    }
}

impl<T> ValidateUniqueItems for &mut T
where
    T: ValidateUniqueItems + ?Sized,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        (**self).validate_unique_items()
    }
}

impl<T> ValidateUniqueItems for Box<T>
where
    T: ValidateUniqueItems + ?Sized,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        self.as_ref().validate_unique_items()
    }
}

impl<T> ValidateUniqueItems for std::borrow::Cow<'_, T>
where
    T: std::borrow::ToOwned + ValidateUniqueItems + ?Sized,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        self.as_ref().validate_unique_items()
    }
}

impl<T> ValidateUniqueItems for std::rc::Rc<T>
where
    T: ValidateUniqueItems + ?Sized,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        self.as_ref().validate_unique_items()
    }
}

impl<T> ValidateUniqueItems for std::sync::Arc<T>
where
    T: ValidateUniqueItems + ?Sized,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        self.as_ref().validate_unique_items()
    }
}

impl<P> ValidateUniqueItems for std::pin::Pin<P>
where
    P: std::ops::Deref,
    P::Target: ValidateUniqueItems,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        self.as_ref().get_ref().validate_unique_items()
    }
}

impl<T> ValidateUniqueItems for Vec<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        if self.is_unique() {
            Ok(())
        } else {
            Err(crate::UniqueItemsError {})
        }
    }
}

impl<T> ValidateUniqueItems for [T]
where
    T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        if self.is_unique() {
            Ok(())
        } else {
            Err(crate::UniqueItemsError {})
        }
    }
}

impl<T, const N: usize> ValidateUniqueItems for [T; N]
where
    T: std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        if self.is_unique() {
            Ok(())
        } else {
            Err(crate::UniqueItemsError {})
        }
    }
}

impl<T> ValidateUniqueItems for Option<T>
where
    T: ValidateUniqueItems,
{
    fn validate_unique_items(&self) -> Result<(), crate::UniqueItemsError> {
        match self {
            Some(value) => value.validate_unique_items(),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_unique_items_array_type_is_true() {
        assert!(ValidateUniqueItems::validate_unique_items(&[1, 2, 3, 4]).is_ok());
    }

    #[test]
    fn test_validate_array_unique_items_vec_type_is_true() {
        assert!(ValidateUniqueItems::validate_unique_items(&vec![1, 2, 3, 4]).is_ok());
    }

    #[test]
    fn test_validate_array_unique_items_is_false() {
        assert!(ValidateUniqueItems::validate_unique_items(&[1, 2, 3, 3]).is_err());
    }
}
