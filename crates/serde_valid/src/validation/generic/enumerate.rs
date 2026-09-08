use super::ValidateEnum;
use crate::EnumError;

/// Deprecated alias for enumerated-value validation.
///
/// # Examples
///
/// ```rust
/// #![allow(deprecated)]
/// use serde_valid::ValidateEnumerate;
///
/// assert!("red".validate_enumerate(&["red", "green"]).is_ok());
/// assert!("blue".validate_enumerate(&["red", "green"]).is_err());
/// ```
#[deprecated(
    since = "2.0.2",
    note = "use `ValidateEnum` and `validate_enum` instead"
)]
pub trait ValidateEnumerate<C> {
    fn validate_enumerate(&self, enumerate: &[C]) -> Result<(), EnumError>;
}

#[allow(deprecated)]
impl<C, T: ValidateEnum<C> + ?Sized> ValidateEnumerate<C> for T {
    fn validate_enumerate(&self, enumerate: &[C]) -> Result<(), EnumError> {
        ValidateEnum::validate_enum(self, enumerate)
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;

    struct MyType(String);

    impl ValidateEnumerate<&'static str> for MyType {
        fn validate_enumerate(&self, candidates: &[&'static str]) -> Result<(), EnumError> {
            self.0.validate_enumerate(candidates)
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
            ValidateEnumerate::validate_enumerate(&MyType("a".to_owned()), &["a", "b"]).is_ok()
        );
    }
}
