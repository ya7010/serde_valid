use itertools::Itertools;

/// A collection whose items [`ValidateUniqueItems`](crate::ValidateUniqueItems) checks for
/// uniqueness.
///
/// Slice, array, and [`Vec`] uniqueness validation is implemented in terms of this trait.
///
/// # Examples
///
/// ```rust
/// use serde_valid::traits::IsUnique;
///
/// struct Ids(Vec<i32>);
///
/// impl IsUnique for Ids {
///     fn is_unique(&self) -> bool {
///         self.0.is_unique()
///     }
/// }
///
/// assert!(Ids(vec![1, 2, 3]).is_unique());
/// assert!(!Ids(vec![1, 1]).is_unique());
/// ```
pub trait IsUnique {
    /// Returns whether all items are unique.
    ///
    /// [`ValidateUniqueItems`](crate::ValidateUniqueItems) treats a `false` result as a validation
    /// error.
    fn is_unique(&self) -> bool;
}

impl<T> IsUnique for [T]
where
    T: std::cmp::Eq + std::hash::Hash,
{
    fn is_unique(&self) -> bool {
        let len = self.len();
        let unique = self.iter().unique();
        let (lower, upper) = unique.size_hint();
        if let Some(upper) = upper {
            if lower == len && upper == len {
                return true;
            }
        }
        unique.count() == len
    }
}
