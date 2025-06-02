mod contains;

use std::collections::{BTreeMap, HashMap};

pub trait InteractsWithData {
    type Value;
    type Item;

    /// Get the data for the container.
    fn data(&self) -> &Self::Value;

    /// Get all of the data
    fn all(&self) -> Self::Value where Self::Value: Clone {
        self.data().clone()
    }

    /// Does the data container the key? 
    fn exists(&self, key: &Self::Item) -> bool where Self::Value: Contains<Self::Item> {
        self.data().contains_item(key)
    }

    /// Does the data contain the key?
    fn has(&self, key: &Self::Item) -> bool where Self::Value: Contains<Self::Item> {
        self.exists(key)
    }

    /// Execute the closure when the instnce contains the given key.
    fn when_has<F>(&mut self, key: &Self::Item, mut closure: F ) 
    where 
        Self::Value: Contains<Self::Item>,
        F: FnMut(&mut Self),
    {
        if self.has(key) {
            closure(self);
        }
    }

    /// Determine if the instance contains a non-empty value for the key.
    fn filled(&self, key: &Self::Item) -> bool {
        false
    }

    /// Determine if the instance contains an empty value for the key.
    fn is_not_filled(&self, key: &Self::Item) -> bool {
        true
    }

    /// Do any of the given keys contain a non-empty value?
    fn any_filled(&self, keys: Vec<&Self::Item>) -> bool {
        false
    }

}

pub trait IntoData {
    type Item;
    type IntoIter: IntoIterator<Item = Self::Item>;

    fn data(self) -> Self::IntoIter;
}

pub trait Contains<T> {
    fn contains_item(&self, item: &T) -> bool;
}


// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_interacts_with_data() {
        let vec = vec![1, 2, 3, 4];
        
        assert_eq!(vec.all(), vec![1, 2, 3, 4]);
        
        assert!(vec.exists(&3)); 
        assert!(!vec.exists(&5)); 
    }

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
