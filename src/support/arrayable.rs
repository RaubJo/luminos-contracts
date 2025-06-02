pub trait Arrayable{
    type T;

    fn to_array(&self) -> Vec<Self::T>;
}
