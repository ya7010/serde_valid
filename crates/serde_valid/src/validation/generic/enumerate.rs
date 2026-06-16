#![allow(deprecated)]

use crate::validation::ValidateCompositedEnumerate;
use crate::EnumError;

/// Enumerate validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values>
///
/// Note: `#[validate(enumerate = ...)]` is deprecated; use `#[validate(r#enum = ...)]`.
#[deprecated(
    since = "2.0.2",
    note = "use `ValidateEnum` and `validate_enum` instead"
)]
pub trait ValidateEnumerate<T> {
    fn validate_enumerate(&self, enumerate: &[T]) -> Result<(), EnumError>;
}

macro_rules! impl_validate_generic_enumerate_literal {
    ($type:ty) => {
        impl ValidateEnumerate<$type> for $type {
            fn validate_enumerate(&self, enumerate: &[$type]) -> Result<(), EnumError> {
                if enumerate.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(EnumError::new(enumerate))
                }
            }
        }

        impl<T> ValidateCompositedEnumerate<&[$type]> for T
        where
            T: ValidateEnumerate<$type>,
        {
            fn validate_composited_enumerate(
                &self,
                limit: &[$type],
            ) -> Result<(), crate::validation::Composited<EnumError>> {
                self.validate_enumerate(limit)
                    .map_err(crate::validation::Composited::Single)
            }
        }
    };
}

impl_validate_generic_enumerate_literal!(i8);
impl_validate_generic_enumerate_literal!(i16);
impl_validate_generic_enumerate_literal!(i32);
impl_validate_generic_enumerate_literal!(i64);
#[cfg(feature = "i128")]
impl_validate_generic_enumerate_literal!(i128);
impl_validate_generic_enumerate_literal!(isize);
impl_validate_generic_enumerate_literal!(u8);
impl_validate_generic_enumerate_literal!(u16);
impl_validate_generic_enumerate_literal!(u32);
impl_validate_generic_enumerate_literal!(u64);
#[cfg(feature = "i128")]
impl_validate_generic_enumerate_literal!(u128);
impl_validate_generic_enumerate_literal!(usize);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI8);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI16);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI32);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI64);
#[cfg(feature = "i128")]
impl_validate_generic_enumerate_literal!(std::num::NonZeroI128);
impl_validate_generic_enumerate_literal!(std::num::NonZeroIsize);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU8);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU16);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU32);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU64);
#[cfg(feature = "i128")]
impl_validate_generic_enumerate_literal!(std::num::NonZeroU128);
impl_validate_generic_enumerate_literal!(std::num::NonZeroUsize);
impl_validate_generic_enumerate_literal!(f32);
impl_validate_generic_enumerate_literal!(f64);
impl_validate_generic_enumerate_literal!(char);

macro_rules! impl_validate_generic_enumerate_str {
    ($type:ty) => {
        impl ValidateEnumerate<&'static str> for $type {
            fn validate_enumerate(&self, enumerate: &[&'static str]) -> Result<(), EnumError> {
                if enumerate.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(EnumError::new(enumerate))
                }
            }
        }
    };
}

impl_validate_generic_enumerate_str!(&str);
impl_validate_generic_enumerate_str!(String);
impl_validate_generic_enumerate_str!(std::borrow::Cow<'_, str>);
impl_validate_generic_enumerate_str!(&std::ffi::OsStr);
impl_validate_generic_enumerate_str!(std::ffi::OsString);

macro_rules! impl_validate_generic_enumerate_path {
    ($type:ty) => {
        impl ValidateEnumerate<&'static str> for $type {
            fn validate_enumerate(&self, enumerate: &[&'static str]) -> Result<(), EnumError> {
                if enumerate
                    .iter()
                    .any(|candidate| &std::path::Path::new(candidate) == self)
                {
                    Ok(())
                } else {
                    Err(EnumError::new(enumerate))
                }
            }
        }
    };
}

impl_validate_generic_enumerate_path!(&std::path::Path);
impl_validate_generic_enumerate_path!(std::path::PathBuf);

impl<T> ValidateCompositedEnumerate<&[&'static str]> for T
where
    T: ValidateEnumerate<&'static str>,
{
    fn validate_composited_enumerate(
        &self,
        limit: &[&'static str],
    ) -> Result<(), crate::validation::Composited<EnumError>> {
        self.validate_enumerate(limit)
            .map_err(crate::validation::Composited::Single)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyType(String);

    impl ValidateEnumerate<&'static str> for MyType {
        fn validate_enumerate(&self, enumerate: &[&'static str]) -> Result<(), EnumError> {
            self.0.validate_enumerate(enumerate)
        }
    }

    #[test]
    fn test_validate_integer_vec_type_is_true() {
        assert!(ValidateEnumerate::validate_enumerate(&1, &[1, 2, 3]).is_ok());
    }

    #[test]
    fn test_validate_integer_vec_type_is_false() {
        assert!(ValidateEnumerate::validate_enumerate(&1, &[2, 3, 4]).is_err());
    }

    #[test]
    fn test_validate_custom_type() {
        assert!(
            ValidateEnumerate::validate_enumerate(&MyType("a".to_string()), &["a", "b"]).is_ok()
        );
    }
}
