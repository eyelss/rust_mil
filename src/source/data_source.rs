use crate::shared::data_types::SupportedDataType;
pub trait DataSource {
  fn get(&self, var_name: String) -> SupportedDataType;
}