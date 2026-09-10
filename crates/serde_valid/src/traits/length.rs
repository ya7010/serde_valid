use unicode_segmentation::UnicodeSegmentation;

/// A string-like value whose length is measured by [`ValidateMinLength`](crate::ValidateMinLength)
/// and [`ValidateMaxLength`](crate::ValidateMaxLength).
///
/// Implement this trait once to provide both minimum-length and maximum-length validation through
/// their blanket implementations. This capability describes character length, not array item count
/// ([`Items`](crate::traits::Items)) or object property count ([`Properties`](crate::traits::Properties)).
///
/// Pointer wrappers (`Box`, `&`, `Cow`, …) are handled by composited [`Transparent`](crate::composited::path::Transparent)
/// paths rather than capability forwarding, so they are not implemented here.
///
/// # Examples
///
/// ```rust
/// use serde_valid::{traits::Length, ValidateMaxLength, ValidateMinLength};
///
/// struct Identifier(String);
///
/// impl Length for Identifier {
///     fn length(&self) -> usize {
///         self.0.chars().count()
///     }
/// }
///
/// let value = Identifier("abc".to_owned());
/// assert!(value.validate_min_length(3).is_ok());
/// assert!(value.validate_max_length(2).is_err());
/// ```
pub trait Length {
    /// Returns the length compared by [`ValidateMinLength`](crate::ValidateMinLength) and
    /// [`ValidateMaxLength`](crate::ValidateMaxLength).
    fn length(&self) -> usize;
}

macro_rules! impl_for_str {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.graphemes(true).count()
            }
        }
    };
}

impl_for_str!(str);
impl_for_str!(String);

macro_rules! impl_for_os_str {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.to_string_lossy().length()
            }
        }
    };
}

impl_for_os_str!(std::ffi::OsStr);
impl_for_os_str!(std::ffi::OsString);

macro_rules! impl_for_path {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.as_os_str().length()
            }
        }
    };
}

impl_for_path!(std::path::Path);
impl_for_path!(std::path::PathBuf);
