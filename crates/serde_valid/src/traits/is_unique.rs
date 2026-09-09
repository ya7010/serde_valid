/// A custom collection compared by [`ValidateUniqueItems`](crate::ValidateUniqueItems).
///
/// Built-in sequences implement [`ValidateUniqueItems`](crate::ValidateUniqueItems) directly.
/// Implement that validator for a custom collection as well. This trait remains as a temporary
/// helper and will be removed; it does not derive the validator.
///
/// # Examples
///
/// ```rust
/// # #![allow(deprecated)]
/// use serde_valid::traits::IsUnique;
/// use serde_valid::ValidateUniqueItems;
///
/// struct Ids(Vec<i32>);
///
/// impl IsUnique for Ids {
///     fn is_unique(&self) -> bool {
///         self.0.validate_unique_items().is_ok()
///     }
/// }
///
/// assert!(Ids(vec![1, 2, 3]).is_unique());
/// assert!(!Ids(vec![1, 1]).is_unique());
/// ```
#[deprecated(since = "3.0.0", note = "implement `ValidateUniqueItems` instead")]
pub trait IsUnique {
    /// Returns whether all items are unique.
    ///
    /// [`ValidateUniqueItems`](crate::ValidateUniqueItems) treats a `false` result as a validation
    /// error.
    fn is_unique(&self) -> bool;
}
