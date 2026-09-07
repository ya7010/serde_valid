use std::collections::BTreeMap;
use std::collections::HashMap;

use indexmap::IndexMap;

pub trait Size {
    fn size(&self) -> usize;
}

impl<T> Size for &T
where
    T: Size + ?Sized,
{
    fn size(&self) -> usize {
        (*self).size()
    }
}

impl<T> Size for Box<T>
where
    T: Size + ?Sized,
{
    fn size(&self) -> usize {
        self.as_ref().size()
    }
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

impl<T> Size for std::borrow::Cow<'_, T>
where
    T: std::borrow::ToOwned + Size + ?Sized,
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
