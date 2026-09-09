use std::collections::BTreeMap;
use std::collections::HashMap;

use indexmap::IndexMap;

/// An object whose property count is measured by
/// [`ValidateMinProperties`](crate::ValidateMinProperties) and
/// [`ValidateMaxProperties`](crate::ValidateMaxProperties).
///
/// Implement this trait once to provide both minimum-property and maximum-property validation
/// through their blanket implementations. This capability describes object size, not string length
/// or array item count.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// use serde_valid::{traits::Size, ValidateMaxProperties, ValidateMinProperties};
///
/// struct Labels(HashMap<String, String>);
///
/// impl Size for Labels {
///     fn size(&self) -> usize {
///         self.0.size()
///     }
/// }
///
/// let mut labels = Labels(HashMap::new());
/// labels.0.insert("env".to_owned(), "prod".to_owned());
/// assert!(labels.validate_min_properties(1).is_ok());
/// assert!(labels.validate_max_properties(0).is_err());
/// ```
pub trait Size {
    /// Returns the property count compared by
    /// [`ValidateMinProperties`](crate::ValidateMinProperties) and
    /// [`ValidateMaxProperties`](crate::ValidateMaxProperties).
    fn size(&self) -> usize;
}

impl<T> Size for std::rc::Rc<T>
where
    T: Size + ?Sized,
{
    fn size(&self) -> usize {
        self.as_ref().size()
    }
}

impl<T> Size for std::sync::Arc<T>
where
    T: Size + ?Sized,
{
    fn size(&self) -> usize {
        self.as_ref().size()
    }
}

impl<P> Size for std::pin::Pin<P>
where
    P: std::ops::Deref,
    P::Target: Size,
{
    fn size(&self) -> usize {
        self.as_ref().get_ref().size()
    }
}

impl<K, V> Size for HashMap<K, V> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<K, V> Size for BTreeMap<K, V> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<K, V> Size for IndexMap<K, V> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl Size for serde_json::Map<String, serde_json::Value> {
    fn size(&self) -> usize {
        self.len()
    }
}
