use crate::validation::{impl_generic_composited_validation_1args, ValidateCompositedMaximum};
use crate::MaximumError;

/// Maximum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMaximum};
/// struct MyType(i32);
///
/// impl ValidateMaximum<i32> for MyType {
///     fn validate_maximum(&self, maximum: i32) -> Result<(), serde_valid::MaximumError> {
///         self.0.validate_maximum(maximum)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(maximum = 5)]
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
///                 "errors": ["The number must be `<= 5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMaximum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_maximum(&self, maximum: T) -> Result<(), MaximumError>;
}

macro_rules! impl_validate_numeric_maximum {
    ($type:ty) => {
        impl ValidateMaximum<$type> for $type {
            fn validate_maximum(&self, maximum: $type) -> Result<(), MaximumError> {
                if *self <= maximum {
                    Ok(())
                } else {
                    Err(MaximumError::new(maximum))
                }
            }
        }

        impl_generic_composited_validation_1args!(
            ValidateCompositedMaximum,
            validate_composited_maximum,
            ValidateMaximum,
            validate_maximum,
            MaximumError,
            $type
        );
    };
}

impl_validate_numeric_maximum!(i8);
impl_validate_numeric_maximum!(i16);
impl_validate_numeric_maximum!(i32);
impl_validate_numeric_maximum!(i64);
#[cfg(feature = "i128")]
impl_validate_numeric_maximum!(i128);
impl_validate_numeric_maximum!(isize);
impl_validate_numeric_maximum!(u8);
impl_validate_numeric_maximum!(u16);
impl_validate_numeric_maximum!(u32);
impl_validate_numeric_maximum!(u64);
#[cfg(feature = "i128")]
impl_validate_numeric_maximum!(u128);
impl_validate_numeric_maximum!(usize);
impl_validate_numeric_maximum!(std::num::NonZeroI8);
impl_validate_numeric_maximum!(std::num::NonZeroI16);
impl_validate_numeric_maximum!(std::num::NonZeroI32);
impl_validate_numeric_maximum!(std::num::NonZeroI64);
#[cfg(feature = "i128")]
impl_validate_numeric_maximum!(std::num::NonZeroI128);
impl_validate_numeric_maximum!(std::num::NonZeroIsize);
impl_validate_numeric_maximum!(std::num::NonZeroU8);
impl_validate_numeric_maximum!(std::num::NonZeroU16);
impl_validate_numeric_maximum!(std::num::NonZeroU32);
impl_validate_numeric_maximum!(std::num::NonZeroU64);
#[cfg(feature = "i128")]
impl_validate_numeric_maximum!(std::num::NonZeroU128);
impl_validate_numeric_maximum!(std::num::NonZeroUsize);
impl_validate_numeric_maximum!(f32);
impl_validate_numeric_maximum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_maximum_is_true() {
        // Unspecified generic type:
        assert!(ValidateMaximum::validate_maximum(&10, 11).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10).is_ok());
    }

    #[test]
    fn test_validate_numeric_maximum_is_false() {
        assert!(ValidateMaximum::validate_maximum(&10, 9).is_err());
    }

    #[test]
    fn test_validate_numeric_maximum_specified_type() {
        assert!(ValidateMaximum::validate_maximum(&10, 10i8).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10i16).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10i32).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10i64).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10isize).is_ok());

        assert!(ValidateMaximum::validate_maximum(&10, 10u8).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10u16).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10u32).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10u64).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10usize).is_ok());

        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroI8::new(10).unwrap(),
            std::num::NonZeroI8::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroI16::new(10).unwrap(),
            std::num::NonZeroI16::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroI32::new(10).unwrap(),
            std::num::NonZeroI32::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroI64::new(10).unwrap(),
            std::num::NonZeroI64::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroIsize::new(10).unwrap(),
            std::num::NonZeroIsize::new(10).unwrap()
        )
        .is_ok());

        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroU8::new(10).unwrap(),
            std::num::NonZeroU8::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroU16::new(10).unwrap(),
            std::num::NonZeroU16::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroU32::new(10).unwrap(),
            std::num::NonZeroU32::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroU64::new(10).unwrap(),
            std::num::NonZeroU64::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroUsize::new(10).unwrap(),
            std::num::NonZeroUsize::new(10).unwrap()
        )
        .is_ok());

        assert!(ValidateMaximum::validate_maximum(&10.0, 10.0f32).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10.0, 10.0f64).is_ok());
    }

    #[test]
    fn test_validate_numeric_maximum_128() {
        assert!(ValidateMaximum::validate_maximum(&10, 10i128).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10u128).is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroI128::new(10).unwrap(),
            std::num::NonZeroI128::new(10).unwrap()
        )
        .is_ok());
        assert!(ValidateMaximum::validate_maximum(
            &std::num::NonZeroU128::new(10).unwrap(),
            std::num::NonZeroU128::new(10).unwrap()
        )
        .is_ok());
    }
}
