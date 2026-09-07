use std::collections::BTreeMap;
use std::collections::HashMap;

use indexmap::IndexMap;

pub trait Size {
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

macro_rules! impl_for_pin_pointer {
    ($pointer:ty) => {
        impl<T> Size for std::pin::Pin<$pointer>
        where
            T: Size + ?Sized,
        {
            fn size(&self) -> usize {
                self.as_ref().get_ref().size()
            }
        }
    };
}

for_each_standard_pin_pointer!(impl_for_pin_pointer);

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
