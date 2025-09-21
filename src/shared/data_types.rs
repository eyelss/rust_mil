#[derive(Debug)]
pub enum PrimitiveType {
  String(String),
  Boolean(bool),
  Integer(i64),
  Float(f64),
  Null
}