pub trait JsonSerializable {
    type Value;
    fn json_serialize(&self) -> Self::Value;
}
