use crate::{traits::Length, MaxLengthError};

/// Max length validation of the string.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMaxLength};
///
/// struct MyType(String);
///
/// impl ValidateMaxLength for MyType {
///     fn validate_max_length(
///         &self,
///         max_length: usize,
///     ) -> Result<(), serde_valid::MaxLengthError> {
///         self.0.validate_max_length(max_length)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(max_length = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType(String::from("abcdef")),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The length of the value must be `<= 5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMaxLength {
    fn validate_max_length(&self, max_length: usize) -> Result<(), MaxLengthError>;
}

macro_rules! impl_validate_max_length {
    ($($type:ty),+ $(,)?) => {$(
        impl ValidateMaxLength for $type {
            fn validate_max_length(&self, max_length: usize) -> Result<(), MaxLengthError> {
                if max_length >= self.length() { Ok(()) } else { Err(MaxLengthError::new(max_length)) }
            }
        }
    )+};
}

impl_validate_max_length!(
    str,
    String,
    std::ffi::OsStr,
    std::ffi::OsString,
    std::path::Path,
    std::path::PathBuf
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_max_length_ascii_is_true() {
        assert!(ValidateMaxLength::validate_max_length("abcde", 5).is_ok());
        assert!(ValidateMaxLength::validate_max_length("abcde", 6).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_unicode_is_true() {
        assert!(ValidateMaxLength::validate_max_length("a̐éö̲", 3).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_japanese_is_true() {
        assert!(ValidateMaxLength::validate_max_length("あ堯", 2).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_emoji_is_true() {
        assert!(ValidateMaxLength::validate_max_length("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_string_type() {
        assert!(ValidateMaxLength::validate_max_length(&String::from("abcde"), 5).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_cow_str_type() {
        assert!(
            crate::validation::ValidateCompositedMaxLength::validate_composited_max_length(
                &Cow::from("abcde"),
                5
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_string_max_length_os_str_type() {
        assert!(ValidateMaxLength::validate_max_length(OsStr::new("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_os_string_type() {
        assert!(ValidateMaxLength::validate_max_length(&OsString::from("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_path_type() {
        assert!(ValidateMaxLength::validate_max_length(Path::new("./foo/bar.txt"), 13).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_path_buf_type() {
        assert!(
            ValidateMaxLength::validate_max_length(&PathBuf::from("./foo/bar.txt"), 13).is_ok()
        );
    }
}
