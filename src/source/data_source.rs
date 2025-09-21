use crate::shared::data_types::PrimitiveType;
pub trait DataSource {
  fn get(&self, var_name: String) -> PrimitiveType;
}