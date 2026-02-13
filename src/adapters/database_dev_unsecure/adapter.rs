use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

use crate::adapters::database::safe_to_unsafe_database;
use crate::adapters::os::path::app_data_dir;
use crate::ports::database::{DatabasePort, SafeDatabase, UnsafeDatabase};

#[derive(Clone, Default)]
pub struct DatabaseDevUnsecureAdapter {}

impl DatabaseDevUnsecureAdapter {
    fn get_db_base_path() -> Result<PathBuf> {
        Ok(app_data_dir()?.join("_dev").join("unsecure"))
    }
}

#[async_trait]
impl DatabasePort for DatabaseDevUnsecureAdapter {
    async fn get_unsafe_database(&self) -> Result<UnsafeDatabase> {
        let db_path = Self::get_db_base_path()?.join("db_unsafe.json");

        let db_raw = tokio::fs::read(&db_path).await?;
        let db: UnsafeDatabase = serde_json::from_slice(&db_raw)?;

        Ok(db)
    }

    async fn get_safe_database(&self) -> Result<SafeDatabase> {
        let db_path = Self::get_db_base_path()?.join("db_safe.json");

        let db_raw = tokio::fs::read(&db_path).await?;
        let db: SafeDatabase = serde_json::from_slice(&db_raw)?;

        Ok(db)
    }

    async fn update_database(&self, safe_database: SafeDatabase) -> Result<()> {
        let db_safe_path = Self::get_db_base_path()?.join("db_safe.json");
        let db_unsafe_path = Self::get_db_base_path()?.join("db_safe.json");

        let unsafe_database = safe_to_unsafe_database(&safe_database)?;

        let unsafe_database_json = serde_json::to_vec(&unsafe_database)?;
        let safe_database_json = serde_json::to_vec(&safe_database)?;

        fs::write(db_safe_path, safe_database_json).await?;
        fs::write(db_unsafe_path, unsafe_database_json).await?;

        Ok(())
    }
}
