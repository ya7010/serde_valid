use super::ValidateEnum;
use crate::EnumError;

/// Deprecated alias for enumerated-value validation.
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

    #[test]
    fn delegates_to_validate_enum() {
        assert!(ValidateEnumerate::validate_enumerate(&1, &[1, 2, 3]).is_ok());
        assert!(ValidateEnumerate::validate_enumerate(&1, &[2, 3, 4]).is_err());
    }
}
