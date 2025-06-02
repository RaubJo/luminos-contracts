use std::collections::HashMap;

use super::super::{Contains, InteractsWithData, IntoData};

impl<K: Eq + std::hash::Hash, V> Contains<K> for HashMap<K, V> {
    fn contains_item(&self, key: &K) -> bool {
        self.contains_key(key)
    }
}

impl<K: Eq + std::hash::Hash, V> InteractsWithData for HashMap<K, V> {
    type Value = HashMap<K, V>;
    type Item = K;

    fn data(&self) -> &Self::Value {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_interacts_with_data() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let expected_map = HashMap::from([("a", 1), ("b", 2)]);
        assert_eq!(map.all(), expected_map);
        
        assert!(map.exists(&"b")); 
        assert!(!map.exists(&"c")); 
    }
}
