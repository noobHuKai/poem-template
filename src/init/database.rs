use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use anyhow::{Ok, Result};
pub async fn init_database() -> Result<Pool<Postgres>> {
    let db_connection_str = std::env::var("DATABASE_URL")?;

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await?;

    Ok(pool)
}
