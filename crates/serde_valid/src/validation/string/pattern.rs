use crate::{traits::IsMatch, PatternError};
use regex::Regex;

/// Pattern validation of the string.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#regular-expressions>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidatePattern};
///
/// struct MyType(String);
///
/// impl ValidatePattern for MyType {
///     fn validate_pattern(
///         &self,
///         pattern: &regex::Regex,
///     ) -> Result<(), serde_valid::PatternError> {
///         self.0.validate_pattern(pattern)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType(String::from("2020/09/10")),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": [r#"The value must match the pattern of "^\d{4}-\d{2}-\d{2}$"."#]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidatePattern {
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternError>;
}

impl<T> ValidatePattern for T
where
    T: IsMatch + ?Sized,
{
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternError> {
        if self.is_match(pattern) {
            Ok(())
        } else {
            Err(PatternError::new(pattern.to_string()))
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
    fn test_validate_string_pattern_str_type() {
        assert!(ValidatePattern::validate_pattern(
            "2020-09-10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_string_type() {
        assert!(ValidatePattern::validate_pattern(
            &String::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_cow_str_type() {
        assert!(ValidatePattern::validate_pattern(
            &Cow::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_os_str_type() {
        assert!(ValidatePattern::validate_pattern(
            OsStr::new("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_os_string_type() {
        assert!(ValidatePattern::validate_pattern(
            &OsString::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_path_type() {
        assert!(ValidatePattern::validate_pattern(
            Path::new("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_path_buf_type() {
        assert!(ValidatePattern::validate_pattern(
            &PathBuf::from("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_is_false() {
        assert!(ValidatePattern::validate_pattern(
            "2020/09/10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_err());
    }
}
