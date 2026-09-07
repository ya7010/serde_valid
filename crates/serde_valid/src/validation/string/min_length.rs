use crate::{traits::Length, MinLengthError};

/// Min length validation of the string.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMinLength};
///
/// struct MyType(String);
///
/// impl ValidateMinLength for MyType {
///     fn validate_min_length(
///         &self,
///         min_length: usize,
///     ) -> Result<(), serde_valid::MinLengthError> {
///         self.0.validate_min_length(min_length)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(min_length = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType(String::from("abc")),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The length of the value must be `>= 5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMinLength {
    fn validate_min_length(&self, min_length: usize) -> Result<(), MinLengthError>;
}

impl<T> ValidateMinLength for T
where
    T: Length + ?Sized,
{
    fn validate_min_length(&self, min_length: usize) -> Result<(), MinLengthError> {
        if min_length <= self.length() {
            Ok(())
        } else {
            Err(MinLengthError::new(min_length))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_min_length_ascii_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"abcde", 5).is_ok());
        assert!(ValidateMinLength::validate_min_length(&"abcde", 4).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_unicode_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"a̐éö̲", 3).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_japanese_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"あ堯", 2).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_emoji_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_string_type() {
        assert!(ValidateMinLength::validate_min_length(&String::from("abcde"), 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_cow_str_type() {
        assert!(ValidateMinLength::validate_min_length(&Cow::from("abcde"), 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_os_str_type() {
        assert!(ValidateMinLength::validate_min_length(&OsStr::new("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_os_string_type() {
        assert!(ValidateMinLength::validate_min_length(&OsString::from("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_path_type() {
        assert!(ValidateMinLength::validate_min_length(&Path::new("./foo/bar.txt"), 13).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_path_buf_type() {
        assert!(
            ValidateMinLength::validate_min_length(&PathBuf::from("./foo/bar.txt"), 13).is_ok()
        );
    }
}
