use crate::PatternError;
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

#[allow(deprecated)]
impl<T> ValidatePattern for T
where
    T: crate::traits::IsMatch + ?Sized,
{
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternError> {
        if self.is_match(pattern) {
            Ok(())
        } else {
            Err(PatternError::new(pattern.to_string()))
        }
    }
}

macro_rules! impl_validate_pattern_str {
    ($type:ty) => {
        impl ValidatePattern for $type {
            fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternError> {
                if pattern.is_match(self) {
                    Ok(())
                } else {
                    Err(PatternError::new(pattern.to_string()))
                }
            }
        }
    };
}

impl_validate_pattern_str!(str);
impl_validate_pattern_str!(&str);
impl_validate_pattern_str!(String);
impl_validate_pattern_str!(std::borrow::Cow<'_, str>);

macro_rules! impl_validate_pattern_os_str {
    ($type:ty) => {
        impl ValidatePattern for $type {
            fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternError> {
                self.to_string_lossy().validate_pattern(pattern)
            }
        }
    };
}

impl_validate_pattern_os_str!(std::ffi::OsStr);
impl_validate_pattern_os_str!(&std::ffi::OsStr);
impl_validate_pattern_os_str!(std::ffi::OsString);
impl_validate_pattern_os_str!(std::borrow::Cow<'_, std::ffi::OsStr>);

macro_rules! impl_validate_pattern_path {
    ($type:ty) => {
        impl ValidatePattern for $type {
            fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternError> {
                self.as_os_str().validate_pattern(pattern)
            }
        }
    };
}

impl_validate_pattern_path!(std::path::Path);
impl_validate_pattern_path!(&std::path::Path);
impl_validate_pattern_path!(std::path::PathBuf);
impl_validate_pattern_path!(std::borrow::Cow<'_, std::path::Path>);

macro_rules! impl_validate_pattern_pointer {
    ($type:ty) => {
        impl ValidatePattern for $type {
            fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternError> {
                (**self).validate_pattern(pattern)
            }
        }
    };
}

impl_validate_pattern_pointer!(Box<str>);
impl_validate_pattern_pointer!(Box<String>);
impl_validate_pattern_pointer!(Box<std::ffi::OsStr>);
impl_validate_pattern_pointer!(Box<std::ffi::OsString>);
impl_validate_pattern_pointer!(Box<std::path::Path>);
impl_validate_pattern_pointer!(Box<std::path::PathBuf>);
impl_validate_pattern_pointer!(std::rc::Rc<str>);
impl_validate_pattern_pointer!(std::rc::Rc<String>);
impl_validate_pattern_pointer!(std::rc::Rc<std::ffi::OsStr>);
impl_validate_pattern_pointer!(std::rc::Rc<std::ffi::OsString>);
impl_validate_pattern_pointer!(std::rc::Rc<std::path::Path>);
impl_validate_pattern_pointer!(std::rc::Rc<std::path::PathBuf>);
impl_validate_pattern_pointer!(std::sync::Arc<str>);
impl_validate_pattern_pointer!(std::sync::Arc<String>);
impl_validate_pattern_pointer!(std::sync::Arc<std::ffi::OsStr>);
impl_validate_pattern_pointer!(std::sync::Arc<std::ffi::OsString>);
impl_validate_pattern_pointer!(std::sync::Arc<std::path::Path>);
impl_validate_pattern_pointer!(std::sync::Arc<std::path::PathBuf>);
impl_validate_pattern_pointer!(std::pin::Pin<Box<str>>);
impl_validate_pattern_pointer!(std::pin::Pin<Box<String>>);
impl_validate_pattern_pointer!(std::pin::Pin<std::rc::Rc<str>>);
impl_validate_pattern_pointer!(std::pin::Pin<std::rc::Rc<String>>);
impl_validate_pattern_pointer!(std::pin::Pin<std::sync::Arc<str>>);
impl_validate_pattern_pointer!(std::pin::Pin<std::sync::Arc<String>>);

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[allow(deprecated)]
    struct CustomMatch(bool);

    #[allow(deprecated)]
    impl crate::traits::IsMatch for CustomMatch {
        fn is_match(&self, _pattern: &Regex) -> bool {
            self.0
        }
    }

    #[test]
    #[allow(deprecated)]
    fn custom_is_match_implementations_are_validated() {
        let pattern = Regex::new(".*").unwrap();
        assert!(ValidatePattern::validate_pattern(&CustomMatch(true), &pattern).is_ok());
        assert!(ValidatePattern::validate_pattern(&CustomMatch(false), &pattern).is_err());
    }

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
        assert!(
            crate::composited::ValidateCompositedPattern::validate_composited_pattern(
                &Cow::from("2020-09-10"),
                &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
            )
            .is_ok()
        );
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
