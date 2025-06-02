use super::super::{Contains, InteractsWithData, IntoData};

impl<T: PartialEq, const N: usize> Contains<T> for [T; N] {
    fn contains_item(&self, key: &T) -> bool {
        self.contains(key)
    }
}

impl<T, const N: usize> InteractsWithData for [T; N] {
    type Value = [T; N];
    type Item = T;

    fn data(&self) -> &Self::Value {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_interacts_with_data() {
        let array= vec![1, 2, 3, 4];
        
        assert_eq!(array.all(), [1, 2, 3, 4]);
        
        assert!(array.exists(&3)); 
        assert!(!array.exists(&5)); 
    }
}
