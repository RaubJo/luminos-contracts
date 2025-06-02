use super::super::{Contains, InteractsWithData, IntoData};

impl<T> IntoData for Vec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn data(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T: PartialEq> Contains<T> for Vec<T> {
    fn contains_item(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T: Clone + PartialEq> InteractsWithData for Vec<T> {
    type Value = Vec<T>;
    type Item = T;

    fn data(&self) -> &Self::Value {
        self
    }
}


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
}
