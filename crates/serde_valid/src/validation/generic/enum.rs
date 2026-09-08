use crate::EnumError;

/// Enum validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values>
///
/// # Examples
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
/// let value = TestStruct {
///     val: MyType("4".to_owned()),
/// };
///
/// assert_eq!(
///     value.validate().unwrap_err().to_string(),
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

macro_rules! impl_validate_enum_literal {
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
    };
}

impl_validate_enum_literal!(i8);
impl_validate_enum_literal!(i16);
impl_validate_enum_literal!(i32);
impl_validate_enum_literal!(i64);
#[cfg(feature = "i128")]
impl_validate_enum_literal!(i128);
impl_validate_enum_literal!(isize);
impl_validate_enum_literal!(u8);
impl_validate_enum_literal!(u16);
impl_validate_enum_literal!(u32);
impl_validate_enum_literal!(u64);
#[cfg(feature = "i128")]
impl_validate_enum_literal!(u128);
impl_validate_enum_literal!(usize);
impl_validate_enum_literal!(std::num::NonZeroI8);
impl_validate_enum_literal!(std::num::NonZeroI16);
impl_validate_enum_literal!(std::num::NonZeroI32);
impl_validate_enum_literal!(std::num::NonZeroI64);
#[cfg(feature = "i128")]
impl_validate_enum_literal!(std::num::NonZeroI128);
impl_validate_enum_literal!(std::num::NonZeroIsize);
impl_validate_enum_literal!(std::num::NonZeroU8);
impl_validate_enum_literal!(std::num::NonZeroU16);
impl_validate_enum_literal!(std::num::NonZeroU32);
impl_validate_enum_literal!(std::num::NonZeroU64);
#[cfg(feature = "i128")]
impl_validate_enum_literal!(std::num::NonZeroU128);
impl_validate_enum_literal!(std::num::NonZeroUsize);
impl_validate_enum_literal!(f32);
impl_validate_enum_literal!(f64);
impl_validate_enum_literal!(char);

impl ValidateEnum<&'static str> for str {
    fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
        if candidates.contains(&self) {
            Ok(())
        } else {
            Err(EnumError::new(candidates))
        }
    }
}

macro_rules! impl_validate_enum_string {
    ($type:ty, $value:expr) => {
        impl ValidateEnum<&'static str> for $type {
            fn validate_enum(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
                if candidates.iter().any(|candidate| $value(self, candidate)) {
                    Ok(())
                } else {
                    Err(EnumError::new(candidates))
                }
            }
        }
    };
}

impl_validate_enum_string!(String, |value: &String, candidate: &&str| candidate
    == value);
impl_validate_enum_string!(
    std::ffi::OsStr,
    |value: &std::ffi::OsStr, candidate: &&str| std::ffi::OsStr::new(candidate) == value
);
impl_validate_enum_string!(
    std::ffi::OsString,
    |value: &std::ffi::OsString, candidate: &&str| std::ffi::OsStr::new(candidate) == value
);
impl_validate_enum_string!(
    std::path::Path,
    |value: &std::path::Path, candidate: &&str| std::path::Path::new(candidate) == value
);
impl_validate_enum_string!(
    std::path::PathBuf,
    |value: &std::path::PathBuf, candidate: &&str| std::path::Path::new(candidate) == value
);

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
        assert!(ValidateEnum::validate_enum(&1_u32, &[0, 1, 2, 3]).is_ok());
        assert!(ValidateEnum::validate_enum(&4_u32, &[0, 1, 2, 3]).is_err());
    }

    #[test]
    fn test_validate_str_type() {
        assert!(ValidateEnum::validate_enum("a", &["a", "b", "c"]).is_ok());
        assert!(ValidateEnum::validate_enum("d", &["a", "b", "c"]).is_err());
    }

    #[test]
    fn test_validate_string_type() {
        assert!(ValidateEnum::validate_enum(&"a".to_owned(), &["a", "b", "c"]).is_ok());
        assert!(ValidateEnum::validate_enum(&"d".to_owned(), &["a", "b", "c"]).is_err());
    }

    #[test]
    fn test_validate_char_type() {
        assert!(ValidateEnum::validate_enum(&'a', &['a', 'b', 'c']).is_ok());
        assert!(ValidateEnum::validate_enum(&'d', &['a', 'b', 'c']).is_err());
    }

    #[test]
    fn test_validate_os_str_type() {
        assert!(ValidateEnum::validate_enum(std::ffi::OsStr::new("a"), &["a", "b", "c"]).is_ok());
        assert!(ValidateEnum::validate_enum(std::ffi::OsStr::new("d"), &["a", "b", "c"]).is_err());
    }

    #[test]
    fn test_validate_os_string_type() {
        assert!(
            ValidateEnum::validate_enum(&std::ffi::OsString::from("a"), &["a", "b", "c"]).is_ok()
        );
        assert!(
            ValidateEnum::validate_enum(&std::ffi::OsString::from("d"), &["a", "b", "c"]).is_err()
        );
    }

    #[test]
    fn test_validate_path_type() {
        assert!(ValidateEnum::validate_enum(std::path::Path::new("a"), &["a", "b", "c"]).is_ok());
        assert!(ValidateEnum::validate_enum(std::path::Path::new("d"), &["a", "b", "c"]).is_err());
    }

    #[test]
    fn test_validate_path_buf_type() {
        assert!(
            ValidateEnum::validate_enum(&std::path::PathBuf::from("a"), &["a", "b", "c"]).is_ok()
        );
        assert!(
            ValidateEnum::validate_enum(&std::path::PathBuf::from("d"), &["a", "b", "c"]).is_err()
        );
    }
}
