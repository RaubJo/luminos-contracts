use std::option::Option;

use super::super::{Contains, InteractsWithData, IntoData};

impl<T> Contains<T> for Option<T> {
    fn contains_item(&self, key: &T) -> bool {
        self.is_some()
    }
}

impl<T: Clone + PartialEq> InteractsWithData for Option<T> {
    type Value = Option<T>;
    type Item = T;

    fn data(&self) -> &Self::Value {
        self
    }

    fn exists(&self, key: &Self::Item) -> bool {
        match self.data() {
            Some(value) => value == key,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_interacts_with_data() {
        let option = Some("a");

        let expected = Some("a");
        assert_eq!(option.all(), expected);
        
        assert!(option.exists(&"a")); 
        assert!(!option.exists(&"c")); 
    }
}
