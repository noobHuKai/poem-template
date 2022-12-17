use crate::model::database::User;
use anyhow::Result;
use sqlx::PgPool;

pub async fn login_service(conn: &PgPool, username: String, password: String) -> Result<User> {
    let user =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1 AND password = $2")
            .bind(username)
            .bind(password)
            .fetch_one(conn)
            .await?;
    Ok(user)
}
