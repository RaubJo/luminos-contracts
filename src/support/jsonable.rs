pub trait Jsonable {
    fn to_json(&self) -> String;
    fn to_json_pretty(&self) -> String;
}
