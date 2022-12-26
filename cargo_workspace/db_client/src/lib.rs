#[macro_use]
extern crate lazy_static;

use std::path::Path;

use anyhow::{Context as _, Result};

mod context;
pub use context::Context;
pub mod config;
use config::APPLICATION_ENV;
use config::DATABASE_URL;
use sqlx::types::chrono;
use sqlx::types::chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Postgres};
use uuid::Uuid;

pub type Pool = sqlx::Pool<Postgres>;
pub type PooledConnection = PoolConnection<Postgres>;

/// Connect to the database and run any migrations. This will
/// log the migration ouput to stdout.
pub async fn connect() -> Result<Pool> {
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(*DATABASE_URL)
        .await
        .with_context(|| "Connecting to the database failed")?;

    // Run migrations
    sqlx::migrate::Migrator::new(Path::new("./migrations"))
        .await
        .with_context(|| "Failed to find migrations")?
        .set_ignore_missing(true)
        .run(&pool)
        .await
        .with_context(|| "Running database migrations failed")?;

    // If in dev environment, seed database
    if *APPLICATION_ENV == "development" {
        seed_entries(&pool).await
    }

    Ok(pool)
}

async fn seed_entries(pool: &Pool) {
    let entries = generate_sample_entries();

    for (id, date, base64_image) in entries {
        let date_time = DateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S %z").unwrap();
        sqlx::query!(
            r#"INSERT INTO photo_entries (entry_id, created_at, base64_image) Values ($1, $2, $3)"#,
            id,
            date_time,
            base64_image
        )
        .execute(pool)
        .await
        .unwrap();
    }
}

fn generate_sample_entries() -> Vec<(Uuid, &'static str, String)> {
    let date =
        DateTime::parse_from_str("1999-01-08 00:00:00 +01:00", "%Y-%m-%d %H:%M:%S %z").unwrap();
    dbg!(date);
    vec![
        (
            Uuid::new_v4(),
            "2022-12-01 00:00:00 +01:00",
            test_utils::sample_base64_image(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-02 00:00:00 +01:00",
            test_utils::sample_base64_image(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-03 00:00:00 +01:00",
            test_utils::sample_base64_image(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-04 00:00:00 +01:00",
            test_utils::sample_base64_image(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-05 00:00:00 +01:00",
            test_utils::sample_base64_image(),
        ),
    ]
}
