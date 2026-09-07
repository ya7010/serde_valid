use crate::EnumError;

/// Enumerated-value validation.
pub trait ValidateEnum<C> {
    fn validate_enum(&self, candidates: &[C]) -> Result<(), EnumError>;
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
    fn literals_and_string_like_values_are_validated() {
        assert!(ValidateEnum::validate_enum(&1, &[1, 2, 3]).is_ok());
        assert!(ValidateEnum::validate_enum(&0.9, &[0.8, 2.3]).is_err());
        assert!(ValidateEnum::validate_enum(std::ffi::OsStr::new("a"), &["a", "b"]).is_ok());
        assert!(ValidateEnum::validate_enum(std::path::Path::new("a"), &["a", "b"]).is_ok());
    }
}
