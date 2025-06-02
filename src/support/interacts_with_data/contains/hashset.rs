use std::{collections::HashSet, hash::BuildHasher};

use super::super::{Contains, InteractsWithData, IntoData};

impl<K: Eq + std::hash::Hash, V: BuildHasher> Contains<K> for HashSet<K, V> {
    fn contains_item(&self, key: &K) -> bool {
        self.contains(key)
    }
}

impl<K: Eq + std::hash::Hash, V: BuildHasher> InteractsWithData for HashSet<K, V> {
    type Value = HashSet<K, V>;
    type Item = K;

    fn data(&self) -> &Self::Value {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashset_interacts_with_data() {
        let mut map = HashSet::new();
        map.insert("a");
        map.insert("b");

        let expected_map = HashSet::from(["a", "b"]);
        assert_eq!(map.all(), expected_map);
        
        assert!(map.exists(&"b")); 
        assert!(!map.exists(&"c")); 
    }
}
