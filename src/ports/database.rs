use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Database {}

#[async_trait]
pub trait DatabasePort: Send + Sync {
    async fn get_unsafe_database(&self) -> Result<Database>;

    async fn get_safe_database(&self) -> Result<Database>;

    async fn update_database(&self, database: Database) -> Result<()>;
}
