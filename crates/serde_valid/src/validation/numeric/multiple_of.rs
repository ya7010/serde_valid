use crate::validation::{impl_generic_composited_validation_1args, ValidateCompositedMultipleOf};
use crate::MultipleOfError;

/// Multipl validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#multiples>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMultipleOf};
///
/// struct MyType(i32);
///
/// impl ValidateMultipleOf<i32> for MyType {
///     fn validate_multiple_of(
///         &self,
///         multiple_of: i32,
///     ) -> Result<(), serde_valid::MultipleOfError> {
///         self.0.validate_multiple_of(multiple_of)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(multiple_of = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct { val: MyType(6) };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The value must be multiple of `5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMultipleOf<T>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
{
    fn validate_multiple_of(&self, multiple_of: T) -> Result<(), crate::MultipleOfError>;
}

macro_rules! impl_validate_numeric_multiple_of {
    ($type:ty) => {
        impl ValidateMultipleOf<$type> for $type {
            fn validate_multiple_of(
                &self,
                multiple_of: $type,
            ) -> Result<(), crate::MultipleOfError> {
                if std::cmp::PartialEq::<$type>::eq(
                    &(*self % multiple_of),
                    &num_traits::Zero::zero(),
                ) {
                    Ok(())
                } else {
                    Err(crate::MultipleOfError::new(multiple_of))
                }
            }
        }

        impl_generic_composited_validation_1args!(
            ValidateCompositedMultipleOf,
            validate_composited_multiple_of,
            ValidateMultipleOf,
            validate_multiple_of,
            MultipleOfError,
            $type
        );
    };
}

impl_validate_numeric_multiple_of!(i8);
impl_validate_numeric_multiple_of!(i16);
impl_validate_numeric_multiple_of!(i32);
impl_validate_numeric_multiple_of!(i64);
#[cfg(feature = "i128")]
impl_validate_numeric_multiple_of!(i128);
impl_validate_numeric_multiple_of!(isize);
impl_validate_numeric_multiple_of!(u8);
impl_validate_numeric_multiple_of!(u16);
impl_validate_numeric_multiple_of!(u32);
impl_validate_numeric_multiple_of!(u64);
#[cfg(feature = "i128")]
impl_validate_numeric_multiple_of!(u128);
impl_validate_numeric_multiple_of!(usize);
impl_validate_numeric_multiple_of!(f32);
impl_validate_numeric_multiple_of!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_true() {
        assert!(ValidateMultipleOf::validate_multiple_of(&10i8, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10i16, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10i32, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10i64, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10isize, 5).is_ok());

        assert!(ValidateMultipleOf::validate_multiple_of(&10u8, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u16, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u32, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u64, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10usize, 5).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_false() {
        assert!(ValidateMultipleOf::validate_multiple_of(&10, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10i8, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10i16, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10i32, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10i64, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10isize, 3).is_err());

        assert!(ValidateMultipleOf::validate_multiple_of(&10u8, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u16, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u32, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u64, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10usize, 3).is_err());
    }

    #[test]
    fn test_validate_numeric_multiple_of_float_is_true() {
        assert!(ValidateMultipleOf::validate_multiple_of(&12.0, 1.0).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&12.5, 0.5).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_float_is_false() {
        assert!(ValidateMultipleOf::validate_multiple_of(&12.0, 5.0).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&12.5, 0.3).is_err());
    }

    #[test]
    #[cfg(feature = "i128")]
    fn test_validate_numeric_multiple_of_128bit_integer_is_true() {
        assert!(ValidateMultipleOf::validate_multiple_of(&10i128, 5).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u128, 5).is_ok());
    }

    #[test]
    #[cfg(feature = "i128")]
    fn test_validate_numeric_multiple_of_128bit_integer_is_false() {
        assert!(ValidateMultipleOf::validate_multiple_of(&10i128, 3).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&10u128, 3).is_err());
    }
}
