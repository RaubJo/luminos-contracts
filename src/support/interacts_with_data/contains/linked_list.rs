use std::collections::LinkedList;
use super::super::{Contains, InteractsWithData, IntoData};

impl<T: PartialEq> Contains<T> for LinkedList<T> {
    fn contains_item(&self, item: &T) -> bool {
        self.iter().any(|x| x == item)
    }
}

impl<T> InteractsWithData for LinkedList<T> {
    type Value = LinkedList<T>;
    type Item = T;

    fn data(&self) -> &Self::Value {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linkedlist_interacts_with_data() {
        let mut map = LinkedList::new();
        map.push_front("a");
        map.push_front("b");

        let expected_map = LinkedList::from(["b", "a"]);
        assert_eq!(map.all(), expected_map);
        
        assert!(map.exists(&"b")); 
        assert!(!map.exists(&"c")); 
    }
}
