use std::fmt::Debug;

#[derive(Debug)]
enum DataSourceType {
  Database(DatabaseConfig)
}

#[derive(Debug)]
struct DatabaseConfig {
  database: String,
  url: String
}

#[derive(Debug)]
struct DataSourceConfig {
  source: DataSourceType 
}

#[derive(Debug)]
pub struct AppConfig {
  source_config: DataSourceConfig
}

impl AppConfig {
    pub fn debug() -> Self {
      Self { 
        source_config: DataSourceConfig { 
          source: DataSourceType::Database(DatabaseConfig {
            database: String::from("base_table"),
            url: String::from("./dev.db")
          })
        } 
      }
    }

    pub fn from_env() -> Result<Self, std::env::VarError> {

      Ok(Self {
        source_config: DataSourceConfig { 
          source: DataSourceType::Database(DatabaseConfig {
            database: std::env::var("DATABASE_NAME")?,
            url: std::env::var("DATABASE_URL")?
          })
        } 
      })
    }
}