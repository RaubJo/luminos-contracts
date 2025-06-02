use std::collections::VecDeque;

use super::super::{Contains, InteractsWithData, IntoData};

impl<T: PartialEq> Contains<T> for VecDeque<T> {
    fn contains_item(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T: Clone + PartialEq> InteractsWithData for VecDeque<T> {
    type Value = VecDeque<T>;
    type Item = T;

    fn data(&self) -> &Self::Value {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_vecdeque_interacts_with_data() {
        let mut deque = VecDeque::new();
        deque.push_back(10);
        deque.push_back(20);
        deque.push_back(30);

        // Check all()
        let expected: VecDeque<_> = vec![10, 20, 30].into_iter().collect();
        assert_eq!(deque.all(), expected);

        // Check exists()
        assert!(deque.exists(&20)); // true
        assert!(!deque.exists(&99)); // false
    }
}
