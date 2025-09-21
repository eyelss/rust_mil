#[derive(Debug)]
pub enum SupportedDataType {
  String(String),
  Boolean(bool),
  Integer(i64),
  Float(f64),
  Null
}