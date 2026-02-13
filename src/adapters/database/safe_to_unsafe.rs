use anyhow::Result;

use crate::ports::database::{SafeDatabase, UnsafeDatabase};

pub fn safe_to_unsafe_database(safe: &SafeDatabase) -> Result<UnsafeDatabase> {
    Ok(UnsafeDatabase {})
}
