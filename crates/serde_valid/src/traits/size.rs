use std::collections::BTreeMap;
use std::collections::HashMap;

use indexmap::IndexMap;

pub trait Size {
    fn size(&self) -> usize;
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
