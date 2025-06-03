use std::result::Result;

use super::super::{Contains, InteractsWithData, IntoData};

impl<T, E> Contains<T> for Result<T, E> {
    fn contains_item(&self, key: &T) -> bool {
        self.is_ok()
    }
}

impl<T: PartialEq, E> InteractsWithData for Result<T, E> {
    type Value = Result<T, E>;
    type Item = T;

    fn data(&self) -> &Self::Value {
        self
    }

    fn exists(&self, key: &Self::Item) -> bool {
        match self.data() {
            Ok(value) => value == key,
            _ => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_interacts_with_data() {
        let result: Result<&str, ()> = Ok("a");

        let expected: Result<&str, ()> = Ok("a");
        assert_eq!(result.all(), expected);
        
        assert!(result.exists(&"a")); 
        assert!(!result.exists(&"c")); 
    }
}
