/// A custom numeric type compared by [`ValidateMinimum`](crate::ValidateMinimum),
/// [`ValidateMaximum`](crate::ValidateMaximum),
/// [`ValidateExclusiveMinimum`](crate::ValidateExclusiveMinimum),
/// [`ValidateExclusiveMaximum`](crate::ValidateExclusiveMaximum), and
/// [`ValidateMultipleOf`](crate::ValidateMultipleOf).
///
/// Built-in numeric types implement those validators directly. Implement this trait for a custom
/// wrapper to get the same validators through their blanket implementations.
///
/// Pointer wrappers (`Box`, `Rc`, …) are handled by composited [`Transparent`](crate::composited::path::Transparent)
/// paths rather than capability forwarding, so they are not implemented here.
///
/// # Examples
///
/// ```rust
/// use serde_valid::{
///     traits::Numeric, ValidateExclusiveMaximum, ValidateMaximum, ValidateMinimum,
///     ValidateMultipleOf,
/// };
///
/// struct Count(i32);
///
/// impl Numeric for Count {
///     type Value = i32;
///
///     fn numeric(&self) -> Self::Value {
///         self.0
///     }
/// }
///
/// let count = Count(10);
/// assert!(count.validate_minimum(10).is_ok());
/// assert!(count.validate_maximum(9).is_err());
/// assert!(count.validate_exclusive_maximum(10).is_err());
/// assert!(count.validate_multiple_of(5).is_ok());
/// ```
pub trait Numeric {
    /// The numeric type compared by the derived validators.
    type Value: Copy;

    /// Returns the number compared by the derived validators.
    fn numeric(&self) -> Self::Value;
}
