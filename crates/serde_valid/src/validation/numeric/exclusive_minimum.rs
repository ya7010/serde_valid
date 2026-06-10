use crate::validation::{
    impl_generic_composited_validation_1args, ValidateCompositedExclusiveMinimum,
};
use crate::ExclusiveMinimumError;

/// Exclusive minimum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateExclusiveMinimum};
/// struct MyType(i32);
///
/// impl ValidateExclusiveMinimum<i32> for MyType {
///     fn validate_exclusive_minimum(&self, exclusive_minimum: i32) -> Result<(), serde_valid::ExclusiveMinimumError> {
///         self.0.validate_exclusive_minimum(exclusive_minimum)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(exclusive_minimum = 5)]
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
///                 "errors": ["The number must be `> 5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateExclusiveMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_exclusive_minimum(&self, exclusive_minimum: T)
        -> Result<(), ExclusiveMinimumError>;
}

macro_rules! impl_validate_numeric_exclusive_minimum {
    ($type:ty) => {
        impl ValidateExclusiveMinimum<$type> for $type {
            fn validate_exclusive_minimum(
                &self,
                exclusive_minimum: $type,
            ) -> Result<(), ExclusiveMinimumError> {
                if *self > exclusive_minimum {
                    Ok(())
                } else {
                    Err(ExclusiveMinimumError::new(exclusive_minimum))
                }
            }
        }

        impl_generic_composited_validation_1args!(
            ValidateCompositedExclusiveMinimum,
            validate_composited_exclusive_minimum,
            ValidateExclusiveMinimum,
            validate_exclusive_minimum,
            ExclusiveMinimumError,
            $type
        );
    };
}

impl_validate_numeric_exclusive_minimum!(i8);
impl_validate_numeric_exclusive_minimum!(i16);
impl_validate_numeric_exclusive_minimum!(i32);
impl_validate_numeric_exclusive_minimum!(i64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_minimum!(i128);
impl_validate_numeric_exclusive_minimum!(isize);
impl_validate_numeric_exclusive_minimum!(u8);
impl_validate_numeric_exclusive_minimum!(u16);
impl_validate_numeric_exclusive_minimum!(u32);
impl_validate_numeric_exclusive_minimum!(u64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_minimum!(u128);
impl_validate_numeric_exclusive_minimum!(usize);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI8);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI16);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI32);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI128);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroIsize);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU8);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU16);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU32);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU64);
#[cfg(feature = "i128")]
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU128);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroUsize);
impl_validate_numeric_exclusive_minimum!(f32);
impl_validate_numeric_exclusive_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_true() {
        // Unspecified generic type:
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9).is_ok());
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_false() {
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 11).is_err());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 10).is_err());
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_specified_type() {
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9i8).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9i16).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9i32).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9i64).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9isize).is_ok());

        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9u8).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9u16).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9u32).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9u64).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9usize).is_ok());

        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroI8::new(10).unwrap(),
            std::num::NonZeroI8::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroI16::new(10).unwrap(),
            std::num::NonZeroI16::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroI32::new(10).unwrap(),
            std::num::NonZeroI32::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroI64::new(10).unwrap(),
            std::num::NonZeroI64::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroIsize::new(10).unwrap(),
            std::num::NonZeroIsize::new(9).unwrap()
        )
        .is_ok());

        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroU8::new(10).unwrap(),
            std::num::NonZeroU8::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroU16::new(10).unwrap(),
            std::num::NonZeroU16::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroU32::new(10).unwrap(),
            std::num::NonZeroU32::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroU64::new(10).unwrap(),
            std::num::NonZeroU64::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroUsize::new(10).unwrap(),
            std::num::NonZeroUsize::new(9).unwrap()
        )
        .is_ok());

        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10.0, 9.9f32).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10.0, 9.9f64).is_ok());
    }

    #[test]
    #[cfg(feature = "i128")]
    fn test_validate_numeric_exclusive_minimum_128() {
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9i128).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9u128).is_ok());

        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroI128::new(10).unwrap(),
            std::num::NonZeroI128::new(9).unwrap()
        )
        .is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(
            &std::num::NonZeroU128::new(10).unwrap(),
            std::num::NonZeroU128::new(9).unwrap()
        )
        .is_ok());
    }
}
