use crate::PooledConnection;
use anyhow::Context;
use sqlx::types::chrono::{DateTime, Local, Utc};
use sqlx::Database;
use uuid::Uuid;

pub struct PhotoEntryDto {
    pub entry_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub base64_image: String,
}

pub async fn get_photo_entries(
    db_conn: &mut PooledConnection,
) -> anyhow::Result<Vec<PhotoEntryDto>> {
    sqlx::query_as!(
        PhotoEntryDto,
        r#"SELECT entry_id, created_at, base64_image FROM photo_entries"#
    )
    .fetch_all(db_conn)
    .await
    .context("Failed to get photo entries")
}

pub async fn create_new_photo_entry(
    db_conn: &mut PooledConnection,
    entry_id: Uuid,
    created_at: DateTime<Local>,
    base64_image: String,
    user_id: Uuid,
) -> anyhow::Result<<sqlx::Postgres as Database>::QueryResult> {
    sqlx::query!(
        r#"INSERT INTO photo_entries (entry_id, created_at, base64_image, user_id) Values ($1, $2, $3, $4)"#,
        entry_id,
        created_at,
        base64_image,
        user_id
    )
    .execute(db_conn)
    .await
    .context("Failed to insert new photo entry into the DB")
}
