use std::collections::BTreeMap;

use super::super::{Contains, InteractsWithData, IntoData};

impl<K: Eq + std::hash::Hash + Ord, V> Contains<K> for BTreeMap<K, V> {
    fn contains_item(&self, key: &K) -> bool {
        self.contains_key(key)
    }
}

impl<K: Eq + std::hash::Hash, V> InteractsWithData for BTreeMap<K, V> {
    type Value = BTreeMap<K, V>;
    type Item = K;

    fn data(&self) -> &Self::Value {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_btreemap_interacts_with_data() {
        let mut map = BTreeMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let expected_map = BTreeMap::from([("a", 1), ("b", 2)]);
        assert_eq!(map.all(), expected_map);
        
        assert!(map.exists(&"b")); 
        assert!(!map.exists(&"c")); 
    }
}
