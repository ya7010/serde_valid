use std::collections::BTreeMap;
use std::collections::HashMap;

use indexmap::IndexMap;

/// An object whose property count is measured by
/// [`ValidateMinProperties`](crate::ValidateMinProperties) and
/// [`ValidateMaxProperties`](crate::ValidateMaxProperties).
///
/// Implement this trait once to provide both minimum-property and maximum-property validation
/// through their blanket implementations. This capability describes object property count, not
/// string length or array item count.
///
/// [`Size`](crate::traits::Size) is a deprecated alias for this trait.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
///
/// use serde_valid::{traits::Properties, ValidateMaxProperties, ValidateMinProperties};
///
/// struct Labels(HashMap<String, String>);
///
/// impl Properties for Labels {
///     fn properties(&self) -> usize {
///         self.0.properties()
///     }
/// }
///
/// let mut labels = Labels(HashMap::new());
/// labels.0.insert("env".to_owned(), "prod".to_owned());
/// assert!(labels.validate_min_properties(1).is_ok());
/// assert!(labels.validate_max_properties(0).is_err());
/// ```
#[doc(alias = "Size")]
pub trait Properties {
    /// Returns the property count compared by
    /// [`ValidateMinProperties`](crate::ValidateMinProperties) and
    /// [`ValidateMaxProperties`](crate::ValidateMaxProperties).
    fn properties(&self) -> usize;

    /// Deprecated alias for [`Properties::properties`].
    #[deprecated(since = "3.0.0", note = "use `Properties::properties` instead")]
    fn size(&self) -> usize {
        self.properties()
    }
}

impl<T> Properties for Box<T>
where
    T: Properties + ?Sized,
{
    fn properties(&self) -> usize {
        self.as_ref().properties()
    }
}

impl<T> Properties for std::rc::Rc<T>
where
    T: Properties + ?Sized,
{
    fn properties(&self) -> usize {
        self.as_ref().properties()
    }
}

impl<T> Properties for std::sync::Arc<T>
where
    T: Properties + ?Sized,
{
    fn properties(&self) -> usize {
        self.as_ref().properties()
    }
}

impl<P> Properties for std::pin::Pin<P>
where
    P: std::ops::Deref,
    P::Target: Properties,
{
    fn properties(&self) -> usize {
        self.as_ref().get_ref().properties()
    }
}

impl<K, V> Properties for HashMap<K, V> {
    fn properties(&self) -> usize {
        self.len()
    }
}

impl<K, V> Properties for BTreeMap<K, V> {
    fn properties(&self) -> usize {
        self.len()
    }
}

impl<K, V> Properties for IndexMap<K, V> {
    fn properties(&self) -> usize {
        self.len()
    }
}

impl Properties for serde_json::Map<String, serde_json::Value> {
    fn properties(&self) -> usize {
        self.len()
    }
}

/// Deprecated alias for [`Properties`].
///
/// Implement [`Properties`] instead. Every [`Properties`] type implements [`Size`] automatically.
#[deprecated(since = "3.0.0", note = "use `Properties` instead")]
pub trait Size: Properties {}

#[allow(deprecated)]
impl<T: Properties + ?Sized> Size for T {}
