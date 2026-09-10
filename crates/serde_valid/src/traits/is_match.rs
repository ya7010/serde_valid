/// A custom string-like type compared by [`ValidatePattern`](crate::ValidatePattern).
///
/// Built-in string types implement [`ValidatePattern`](crate::ValidatePattern) directly. Prefer
/// implementing that validator for a custom type as well. This trait remains as a temporary
/// convenience, like [`Numeric`](crate::traits::Numeric), and will be removed.
///
/// # Examples
///
/// ```rust
/// # #![allow(deprecated)]
/// use regex::Regex;
/// use serde_valid::{traits::IsMatch, ValidatePattern};
///
/// struct Identifier(String);
///
/// impl IsMatch for Identifier {
///     fn is_match(&self, pattern: &Regex) -> bool {
///         pattern.is_match(&self.0)
///     }
/// }
///
/// let pattern = Regex::new(r"^[A-Z]+$").unwrap();
/// assert!(Identifier("ABC".to_owned()).validate_pattern(&pattern).is_ok());
/// assert!(Identifier("abc".to_owned()).validate_pattern(&pattern).is_err());
/// ```
#[deprecated(since = "3.0.0", note = "implement `ValidatePattern` instead")]
pub trait IsMatch {
    /// Returns whether the value matches `pattern`.
    ///
    /// [`ValidatePattern`](crate::ValidatePattern) treats a `false` result as a validation error.
    fn is_match(&self, pattern: &regex::Regex) -> bool;
}
