/// A collection whose values can be validated by numeric position.
///
/// Implement this trait once to make a custom sequence participate in
/// element-wise validation performed by [`Validate`](crate::Validate). This
/// capability only describes traversal. It does not define string length,
/// item count, uniqueness, or object size.
///
/// # Examples
///
/// ```rust
/// use serde_valid::{traits::Sequence, Validate};
///
/// struct MyVec<T>(Vec<T>);
///
/// impl<T> Sequence for MyVec<T> {
///     type Item = T;
///
///     fn for_each(&self, visitor: impl FnMut(&Self::Item)) {
///         self.0.iter().for_each(visitor);
///     }
/// }
///
/// #[derive(Validate)]
/// struct Request {
///     #[validate(min_length = 3)]
///     names: MyVec<String>,
/// }
///
/// let request = Request {
///     names: MyVec(vec!["Alice".to_owned(), "Bo".to_owned()]),
/// };
/// assert!(request.validate().is_err());
/// ```
pub trait Sequence {
    type Item;

    /// Visits each value in index order.
    ///
    /// The visitor is [`FnMut`] so the validation engine can accumulate an index and errors. The
    /// sequence and its values remain immutably borrowed.
    fn for_each(&self, visitor: impl FnMut(&Self::Item));
}

impl<T> Sequence for Vec<T> {
    type Item = T;

    fn for_each(&self, visitor: impl FnMut(&Self::Item)) {
        self.iter().for_each(visitor);
    }
}

impl<T> Sequence for [T] {
    type Item = T;

    fn for_each(&self, visitor: impl FnMut(&Self::Item)) {
        self.iter().for_each(visitor);
    }
}

impl<T, const N: usize> Sequence for [T; N] {
    type Item = T;

    fn for_each(&self, visitor: impl FnMut(&Self::Item)) {
        self.iter().for_each(visitor);
    }
}
