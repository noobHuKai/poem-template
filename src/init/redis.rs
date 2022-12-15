use std::sync::Arc;

use anyhow::{Ok, Result};
use redis::aio::Connection;

pub async fn init_redis() -> Result<Arc<Connection>> {
    let redis_connection_str = std::env::var("REDIS_URL")?;

    let client = redis::Client::open(redis_connection_str)?;
    let con = client.get_async_connection().await?;

    Ok(Arc::new(con))
}
