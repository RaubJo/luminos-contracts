pub trait Vectorable{
    type T;
    
    fn to_vec(&self) -> Vec<Self::T>;
}
