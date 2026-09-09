use unicode_segmentation::UnicodeSegmentation;

/// A string-like value whose length is measured by [`ValidateMinLength`](crate::ValidateMinLength)
/// and [`ValidateMaxLength`](crate::ValidateMaxLength).
///
/// Implement this trait once to provide both minimum-length and maximum-length validation through
/// their blanket implementations. This capability describes character length, not array item count
/// or object property count.
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

impl<T> Length for Box<T>
where
    T: Length + ?Sized,
{
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<T> Length for std::rc::Rc<T>
where
    T: Length + ?Sized,
{
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<T> Length for std::sync::Arc<T>
where
    T: Length + ?Sized,
{
    fn length(&self) -> usize {
        self.as_ref().length()
    }
}

impl<P> Length for std::pin::Pin<P>
where
    P: std::ops::Deref,
    P::Target: Length,
{
    fn length(&self) -> usize {
        self.as_ref().get_ref().length()
    }
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
impl_for_str!(&str);
impl_for_str!(String);
impl_for_str!(std::borrow::Cow<'_, str>);

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
impl_for_os_str!(&std::ffi::OsStr);
impl_for_os_str!(std::ffi::OsString);
impl_for_os_str!(std::borrow::Cow<'_, std::ffi::OsStr>);

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
impl_for_path!(&std::path::Path);
impl_for_path!(std::path::PathBuf);
impl_for_path!(std::borrow::Cow<'_, std::path::Path>);
