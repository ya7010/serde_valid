use crate::validation::{
    impl_generic_composited_validation_1args, ValidateCompositedExclusiveMaximum,
};
use crate::ExclusiveMaximumError;

/// Exclusive maximum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateExclusiveMaximum};
/// struct MyType(i32);
///
/// impl ValidateExclusiveMaximum<i32> for MyType {
///     fn validate_exclusive_maximum(&self, exclusive_maximum: i32) -> Result<(), serde_valid::ExclusiveMaximumError> {
///         self.0.validate_exclusive_maximum(exclusive_maximum)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(exclusive_maximum = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct { val: MyType(5) };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The number must be `< 5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateExclusiveMaximum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_exclusive_maximum(&self, exclusive_maximum: T)
        -> Result<(), ExclusiveMaximumError>;
}

macro_rules! impl_validate_numeric_exclusive_maximum {
    ($type:ty) => {
        impl ValidateExclusiveMaximum<$type> for $type {
            fn validate_exclusive_maximum(
                &self,
                exclusive_maximum: $type,
            ) -> Result<(), crate::ExclusiveMaximumError> {
                if *self < exclusive_maximum {
                    Ok(())
                } else {
                    Err(crate::ExclusiveMaximumError::new(exclusive_maximum))
                }
            }
        }

        impl_generic_composited_validation_1args!(
            ValidateCompositedExclusiveMaximum,
            validate_composited_exclusive_maximum,
            ValidateExclusiveMaximum,
            validate_exclusive_maximum,
            ExclusiveMaximumError,
            $type
        );
    };
}

impl_validate_numeric_exclusive_maximum!(i8);
impl_validate_numeric_exclusive_maximum!(i16);
impl_validate_numeric_exclusive_maximum!(i32);
impl_validate_numeric_exclusive_maximum!(i64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_maximum!(i128);
impl_validate_numeric_exclusive_maximum!(isize);
impl_validate_numeric_exclusive_maximum!(u8);
impl_validate_numeric_exclusive_maximum!(u16);
impl_validate_numeric_exclusive_maximum!(u32);
impl_validate_numeric_exclusive_maximum!(u64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_maximum!(u128);
impl_validate_numeric_exclusive_maximum!(usize);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI8);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI16);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI32);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI128);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroIsize);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU8);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU16);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU32);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU128);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroUsize);
impl_validate_numeric_exclusive_maximum!(f32);
impl_validate_numeric_exclusive_maximum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_exclusive_maximum_is_true() {
        // Unspecified generic type:
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11).is_ok());
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_is_false() {
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 9).is_err());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 10).is_err());
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_specified_type() {
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11i8).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11i16).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11i32).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11i64).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11isize).is_ok());

        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11u8).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11u16).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11u32).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11u64).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11usize).is_ok());

        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroI8::new(10).unwrap(),
            std::num::NonZeroI8::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroI16::new(10).unwrap(),
            std::num::NonZeroI16::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroI32::new(10).unwrap(),
            std::num::NonZeroI32::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroI64::new(10).unwrap(),
            std::num::NonZeroI64::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroIsize::new(10).unwrap(),
            std::num::NonZeroIsize::new(11).unwrap()
        )
        .is_ok());

        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroU8::new(10).unwrap(),
            std::num::NonZeroU8::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroU16::new(10).unwrap(),
            std::num::NonZeroU16::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroU32::new(10).unwrap(),
            std::num::NonZeroU32::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroU64::new(10).unwrap(),
            std::num::NonZeroU64::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroUsize::new(10).unwrap(),
            std::num::NonZeroUsize::new(11).unwrap()
        )
        .is_ok());

        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10.0, 10.1f32).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10.0, 10.1f64).is_ok());
    }

    #[test]
    #[cfg(feature = "i128")]
    fn test_validate_numeric_exclusive_maximum_128() {
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11i128).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11u128).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroI128::new(10).unwrap(),
            std::num::NonZeroI128::new(11).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(
            &std::num::NonZeroU128::new(10).unwrap(),
            std::num::NonZeroU128::new(11).unwrap()
        )
        .is_ok());
    }
}
