use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SafeDatabase {}

#[derive(Serialize, Deserialize)]
pub struct UnsafeDatabase {}

/// Database port, that securely stores our database.
/// Meant to be backed by crypto, TPM, Windows Hello, or relevant security measures
/// depending on the OS.
/// The database is typically gated by Windows Hello (or other user-interaction schemes).
///
/// So we have two database entrypoints:
/// - a "safe" database, gated by Windows Hello, that can store data with a high assurance on
/// confidentiality and integrity.
/// - an "unsafe" database, that stores a subset of the data of the safe database, that don't need
/// high assurance on confidentiality. It is stored in plaintext on disk, so assume a malicious
/// actor may have tampered with it.
/// Data that needs a high integrity can be stored there, but the code needs to treat them as
/// low integrity, and double check with the safe database before performing sensitive actions.
#[async_trait]
pub trait DatabasePort: Send + Sync {
    async fn get_unsafe_database(&self) -> Result<UnsafeDatabase>;

    async fn get_safe_database(&self) -> Result<SafeDatabase>;

    async fn update_database(&self, database: SafeDatabase) -> Result<()>;
}
