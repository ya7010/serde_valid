use crate::validation::ValidateCompositedEnum;
use crate::EnumError;

/// Enumerate validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values>
///
/// Note: `#[validate(enumerate = ...)]` is deprecated; use `#[validate(r#enum = ...)]`.
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateEnum};
///
/// struct MyType(String);
///
/// impl ValidateEnum<&'static str> for MyType {
///     fn validate_enum(
///         &self,
///         candidates: &[&'static str],
///     ) -> Result<(), serde_valid::EnumError> {
///         self.0.validate_enum(candidates)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(r#enum = ["1", "2", "3"])]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType("4".to_string()),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The value must be in [1, 2, 3]."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateEnum<T> {
    fn validate_enum(&self, candidates: &[T]) -> Result<(), EnumError>;
}

macro_rules! impl_validate_generic_enumerate_literal {
    ($type:ty) => {
        impl ValidateEnum<$type> for $type {
            fn validate_enum(&self, candidates: &[$type]) -> Result<(), EnumError> {
                if candidates.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(EnumError::new(candidates))
                }
            }
        }

        // Keep the blanket impl `Sized` to preserve downstream coherence.
        impl<T> ValidateCompositedEnum<&[$type]> for T
        where
            T: ValidateEnum<$type>,
        {
            fn validate_composited_enum(
                &self,
                limit: &[$type],
            ) -> Result<(), crate::validation::Composited<EnumError>> {
                self.validate_enum(limit)
                    .map_err(|error| crate::validation::Composited::Single(error))
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
        impl ValidateEnum<&'static str> for $type {
            fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
                if candidates.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(EnumError::new(candidates))
                }
            }
        }
    };
}

impl ValidateEnum<&'static str> for str {
    fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
        if candidates.contains(&self) {
            Ok(())
        } else {
            Err(EnumError::new(candidates))
        }
    }
}

impl_validate_generic_enumerate_str!(String);
impl_validate_generic_enumerate_str!(std::ffi::OsString);

impl<C, T> ValidateEnum<C> for &T
where
    T: ValidateEnum<C> + ?Sized,
{
    fn validate_enum(&self, candidates: &[C]) -> Result<(), EnumError> {
        (*self).validate_enum(candidates)
    }
}

impl<C, T> ValidateEnum<C> for std::borrow::Cow<'_, T>
where
    T: std::borrow::ToOwned + ValidateEnum<C> + ?Sized,
{
    fn validate_enum(&self, candidates: &[C]) -> Result<(), EnumError> {
        self.as_ref().validate_enum(candidates)
    }
}

impl ValidateEnum<&'static str> for std::ffi::OsStr {
    fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
        if candidates
            .iter()
            .any(|candidate| std::ffi::OsStr::new(candidate) == self)
        {
            Ok(())
        } else {
            Err(EnumError::new(candidates))
        }
    }
}

impl<C, T> ValidateEnum<C> for Box<T>
where
    T: ValidateEnum<C> + ?Sized,
{
    fn validate_enum(&self, candidates: &[C]) -> Result<(), EnumError> {
        self.as_ref().validate_enum(candidates)
    }
}

impl<C, T> ValidateEnum<C> for std::rc::Rc<T>
where
    T: ValidateEnum<C> + ?Sized,
{
    fn validate_enum(&self, candidates: &[C]) -> Result<(), EnumError> {
        self.as_ref().validate_enum(candidates)
    }
}

impl<C, T> ValidateEnum<C> for std::sync::Arc<T>
where
    T: ValidateEnum<C> + ?Sized,
{
    fn validate_enum(&self, candidates: &[C]) -> Result<(), EnumError> {
        self.as_ref().validate_enum(candidates)
    }
}

impl<C, P> ValidateEnum<C> for std::pin::Pin<P>
where
    P: std::ops::Deref,
    P::Target: ValidateEnum<C>,
{
    fn validate_enum(&self, candidates: &[C]) -> Result<(), EnumError> {
        self.as_ref().get_ref().validate_enum(candidates)
    }
}

macro_rules! impl_validate_generic_enumerate_path {
    ($type:ty) => {
        impl ValidateEnum<&'static str> for $type {
            fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
                if candidates
                    .iter()
                    .any(|candidate| &std::path::Path::new(candidate) == self)
                {
                    Ok(())
                } else {
                    Err(EnumError::new(candidates))
                }
            }
        }
    };
}

impl_validate_generic_enumerate_path!(std::path::PathBuf);

impl ValidateEnum<&'static str> for std::path::Path {
    fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
        if candidates
            .iter()
            .any(|candidate| std::path::Path::new(candidate) == self)
        {
            Ok(())
        } else {
            Err(EnumError::new(candidates))
        }
    }
}

// Keep the blanket impl `Sized` to preserve downstream coherence.
impl<T> ValidateCompositedEnum<&[&'static str]> for T
where
    T: ValidateEnum<&'static str>,
{
    fn validate_composited_enum(
        &self,
        limit: &[&'static str],
    ) -> Result<(), crate::validation::Composited<EnumError>> {
        self.validate_enum(limit)
            .map_err(crate::validation::Composited::Single)
    }
}

macro_rules! impl_unsized_composited_enum {
    ($($type:ty),+ $(,)?) => {
        $(
            impl<'a> ValidateCompositedEnum<&'a [&'static str]> for $type {
                fn validate_composited_enum(
                    &self,
                    limit: &'a [&'static str],
                ) -> Result<(), crate::validation::Composited<EnumError>> {
                    <$type as ValidateEnum<&'static str>>::validate_enum(self, limit)
                        .map_err(crate::validation::Composited::Single)
                }
            }
        )+
    };
}

impl_unsized_composited_enum!(str, std::ffi::OsStr, std::path::Path);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_integer_vec_type_is_true() {
        assert!(ValidateEnum::validate_enum(&1, &[1, 2, 3]).is_ok());
    }

    #[test]
    fn test_validate_integer_vec_type_is_false() {
        assert!(ValidateEnum::validate_enum(&1, &[2, 3, 4]).is_err());
    }

    #[test]
    fn test_validate_float_type_is_true() {
        assert!(ValidateEnum::validate_enum(&0.9, &[0.9, 2.3, -3.0]).is_ok());
    }

    #[test]
    fn test_validate_float_type_is_false() {
        assert!(ValidateEnum::validate_enum(&0.9, &[0.8, 2.3, -3.0]).is_err());
    }

    #[test]
    fn test_validate_unsigned_int_type() {
        assert!(ValidateEnum::validate_enum(&1, &[-1, 0, 1, 2, 3]).is_ok());
    }

    #[test]
    fn test_validate_str_type() {
        assert!(ValidateEnum::validate_enum(&'a', &['a', 'b', 'c']).is_ok());
    }

    #[test]
    fn test_validate_string_type() {
        assert!(ValidateEnum::validate_enum(&'a', &['a', 'b', 'c']).is_ok());
    }

    #[test]
    fn test_validate_os_str_type() {
        assert!(ValidateEnum::validate_enum(&std::ffi::OsStr::new("a"), &["a", "b", "c"]).is_ok());
    }

    #[test]
    fn test_validate_os_string_type() {
        assert!(
            ValidateEnum::validate_enum(&std::ffi::OsString::from("a"), &["a", "b", "c"]).is_ok()
        );
    }

    #[test]
    fn test_validate_path_type() {
        assert!(ValidateEnum::validate_enum(&std::path::Path::new("a"), &["a", "b", "c"]).is_ok());
    }

    #[test]
    fn test_validate_path_buf_type() {
        assert!(
            ValidateEnum::validate_enum(&std::path::PathBuf::from("a"), &["a", "b", "c"]).is_ok()
        );
    }
}
