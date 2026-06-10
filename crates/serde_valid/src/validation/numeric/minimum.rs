use crate::validation::{impl_generic_composited_validation_1args, ValidateCompositedMinimum};
use crate::MinimumError;

/// Minimum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMinimum};
/// struct MyType(i32);
///
/// impl ValidateMinimum<i32> for MyType {
///     fn validate_minimum(&self, minimum: i32) -> Result<(), serde_valid::MinimumError> {
///         self.0.validate_minimum(minimum)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(minimum = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct { val: MyType(3) };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The number must be `>= 5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_minimum(&self, minimum: T) -> Result<(), MinimumError>;
}

macro_rules! impl_validate_numeric_minimum {
    ($type:ty) => {
        impl ValidateMinimum<$type> for $type {
            fn validate_minimum(&self, minimum: $type) -> Result<(), MinimumError> {
                if *self >= minimum {
                    Ok(())
                } else {
                    Err(MinimumError::new(minimum))
                }
            }
        }

        impl_generic_composited_validation_1args!(
            ValidateCompositedMinimum,
            validate_composited_minimum,
            ValidateMinimum,
            validate_minimum,
            MinimumError,
            $type
        );
    };
}

impl_validate_numeric_minimum!(i8);
impl_validate_numeric_minimum!(i16);
impl_validate_numeric_minimum!(i32);
impl_validate_numeric_minimum!(i64);
#[cfg(feature = "i128")]
impl_validate_numeric_minimum!(i128);
impl_validate_numeric_minimum!(isize);
impl_validate_numeric_minimum!(u8);
impl_validate_numeric_minimum!(u16);
impl_validate_numeric_minimum!(u32);
impl_validate_numeric_minimum!(u64);
#[cfg(feature = "i128")]
impl_validate_numeric_minimum!(u128);
impl_validate_numeric_minimum!(usize);
impl_validate_numeric_minimum!(std::num::NonZeroI8);
impl_validate_numeric_minimum!(std::num::NonZeroI16);
impl_validate_numeric_minimum!(std::num::NonZeroI32);
impl_validate_numeric_minimum!(std::num::NonZeroI64);
#[cfg(feature = "i128")]
impl_validate_numeric_minimum!(std::num::NonZeroI128);
impl_validate_numeric_minimum!(std::num::NonZeroIsize);
impl_validate_numeric_minimum!(std::num::NonZeroU8);
impl_validate_numeric_minimum!(std::num::NonZeroU16);
impl_validate_numeric_minimum!(std::num::NonZeroU32);
impl_validate_numeric_minimum!(std::num::NonZeroU64);
#[cfg(feature = "i128")]
impl_validate_numeric_minimum!(std::num::NonZeroU128);
impl_validate_numeric_minimum!(std::num::NonZeroUsize);
impl_validate_numeric_minimum!(f32);
impl_validate_numeric_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_minimum_is_true() {
        assert!(ValidateMinimum::validate_minimum(&10, 9).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10).is_ok());
    }

    #[test]
    fn test_validate_numeric_minimum_is_false() {
        assert!(ValidateMinimum::validate_minimum(&10, 11).is_err());
    }

    #[test]
    fn test_validate_numeric_minimum_specified_type() {
        assert!(ValidateMinimum::validate_minimum(&10, 10i8).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10i16).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10i32).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10i64).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10isize).is_ok());

        assert!(ValidateMinimum::validate_minimum(&10, 10u8).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10u16).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10u32).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10u64).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10usize).is_ok());

        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroI8::new(10).unwrap(),
            std::num::NonZeroI8::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroI16::new(10).unwrap(),
            std::num::NonZeroI16::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroI32::new(10).unwrap(),
            std::num::NonZeroI32::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroI64::new(10).unwrap(),
            std::num::NonZeroI64::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroIsize::new(10).unwrap(),
            std::num::NonZeroIsize::new(10).unwrap()
        )
        .is_ok());

        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroU8::new(10).unwrap(),
            std::num::NonZeroU8::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroU16::new(10).unwrap(),
            std::num::NonZeroU16::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroU32::new(10).unwrap(),
            std::num::NonZeroU32::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroU64::new(10).unwrap(),
            std::num::NonZeroU64::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroUsize::new(10).unwrap(),
            std::num::NonZeroUsize::new(10).unwrap()
        )
        .is_ok());

        assert!(ValidateMinimum::validate_minimum(&10.0, 10.0f32).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10.0, 10.0f64).is_ok());
    }

    #[test]
    #[cfg(feature = "i128")]
    fn test_validate_numeric_minimum_128() {
        assert!(ValidateMinimum::validate_minimum(&10, 10i128).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10u128).is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroI128::new(10).unwrap(),
            std::num::NonZeroI128::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMinimum::validate_minimum(
            &std::num::NonZeroU128::new(10).unwrap(),
            std::num::NonZeroU128::new(10).unwrap()
        )
        .is_ok());
    }
}
