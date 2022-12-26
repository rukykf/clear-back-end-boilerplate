use crate::PooledConnection;
use anyhow::Context;
use secrecy::Secret;
use uuid::Uuid;

pub struct UserCredentialsDto {
    pub user_id: Uuid,
    pub username: String,
    pub password_hash: Secret<String>,
}

pub async fn get_stored_credentials(
    db_conn: &mut PooledConnection,
    username: String,
) -> anyhow::Result<Option<UserCredentialsDto>> {
    let user_credentials: Option<UserCredentialsDto> = sqlx::query!(
        r#"SELECT user_id, username, password_hash FROM users
        WHERE username = $1"#,
        username
    )
    .fetch_optional(db_conn)
    .await
    .context("Failed to get user credentials")?
    .map(|row| UserCredentialsDto {
        user_id: row.user_id,
        username: row.username,
        password_hash: Secret::new(row.password_hash),
    });

    Ok(user_credentials)
}
