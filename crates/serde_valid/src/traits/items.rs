/// A collection whose item count is measured by [`ValidateMinItems`](crate::ValidateMinItems) and
/// [`ValidateMaxItems`](crate::ValidateMaxItems).
///
/// Implement this trait once to provide both minimum-item and maximum-item validation through
/// their blanket implementations. This capability describes array item count, not string length or
/// object property count.
///
/// # Examples
///
/// ```rust
/// use serde_valid::{traits::Items, ValidateMaxItems, ValidateMinItems};
///
/// struct Ids(Vec<i32>);
///
/// impl Items for Ids {
///     fn items(&self) -> usize {
///         self.0.items()
///     }
/// }
///
/// let ids = Ids(vec![1, 2, 3]);
/// assert!(ids.validate_min_items(3).is_ok());
/// assert!(ids.validate_max_items(2).is_err());
/// ```
pub trait Items {
    /// Returns the item count compared by [`ValidateMinItems`](crate::ValidateMinItems) and
    /// [`ValidateMaxItems`](crate::ValidateMaxItems).
    fn items(&self) -> usize;
}

impl<T> Items for Box<T>
where
    T: Items + ?Sized,
{
    fn items(&self) -> usize {
        self.as_ref().items()
    }
}

impl<T> Items for &T
where
    T: Items + ?Sized,
{
    fn items(&self) -> usize {
        (**self).items()
    }
}

impl<T> Items for &mut T
where
    T: Items + ?Sized,
{
    fn items(&self) -> usize {
        (**self).items()
    }
}

impl<T> Items for std::rc::Rc<T>
where
    T: Items + ?Sized,
{
    fn items(&self) -> usize {
        self.as_ref().items()
    }
}

impl<T> Items for std::sync::Arc<T>
where
    T: Items + ?Sized,
{
    fn items(&self) -> usize {
        self.as_ref().items()
    }
}

impl<P> Items for std::pin::Pin<P>
where
    P: std::ops::Deref,
    P::Target: Items,
{
    fn items(&self) -> usize {
        self.as_ref().get_ref().items()
    }
}

impl<T> Items for std::borrow::Cow<'_, T>
where
    T: std::borrow::ToOwned + Items + ?Sized,
{
    fn items(&self) -> usize {
        self.as_ref().items()
    }
}

impl<T> Items for Vec<T> {
    fn items(&self) -> usize {
        self.len()
    }
}

impl<T> Items for [T] {
    fn items(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> Items for [T; N] {
    fn items(&self) -> usize {
        self.len()
    }
}
