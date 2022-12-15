mod database;
mod redis;
mod tracing;
use anyhow::Result;
use dotenv::dotenv;

pub use self::redis::init_redis;
pub use database::init_database;

pub fn init() -> Result<()> {
    dotenv()?;

    tracing::init_tracing()?;

    Ok(())
}
