use std::collections::BTreeSet;
use std::alloc::Allocator;

use super::super::{Contains, InteractsWithData, IntoData};

impl<K: Eq + std::hash::Hash + Ord, V: Clone + Allocator> Contains<K> for BTreeSet<K, V> {
    fn contains_item(&self, key: &K) -> bool {
        self.contains(key)
    }
}

impl<K: Eq + std::hash::Hash, V: Clone + Allocator> InteractsWithData for BTreeSet<K, V> {
    type Value = BTreeSet<K, V>;
    type Item = K;

    fn data(&self) -> &Self::Value {
        self
    }
}
