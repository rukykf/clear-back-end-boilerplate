#[macro_use]
extern crate lazy_static;

use std::path::Path;

use anyhow::{Context as _, Result};
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};

mod context;
pub use context::Context;
pub mod auth;
pub mod config;
pub mod photo_entries;

use config::APPLICATION_ENV;
use config::DATABASE_URL;
use sqlx::types::chrono::{DateTime, Local};
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
    let (_, user_id) = generate_sample_user(pool).await;
    let entries = generate_sample_entries(user_id);

    for (id, date, base64_image, user_id) in entries {
        let date_time = DateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S %z").unwrap();
        sqlx::query!(
            r#"INSERT INTO photo_entries (entry_id, created_at, base64_image, user_id) Values ($1, $2, $3, $4)"#,
            id,
            date_time,
            base64_image,
            user_id.clone()
        )
        .execute(pool)
        .await
        .unwrap();
    }

    // Insert the sample auth token into the DB for the seed user
    let token = test_utils::sample_auth_token();
    let expires_at = Local::now();
    sqlx::query!(
        r#"INSERT INTO authentication_tokens (user_id, token, expires_at) Values ($1, $2, $3)"#,
        user_id,
        token,
        expires_at
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn generate_sample_user(pool: &Pool) -> (String, Uuid) {
    let username = "admin".to_string();
    let password_hash = compute_password_hash("admin");
    let user_id = test_utils::sample_user_id();
    let user_id = Uuid::parse_str(user_id.as_str()).unwrap();

    sqlx::query!(
        r#"INSERT INTO users (user_id, username, password_hash) Values ($1, $2, $3)"#,
        user_id,
        username,
        password_hash
    )
    .execute(pool)
    .await
    .unwrap();
    (username, user_id)
}

fn generate_sample_entries(user_id: Uuid) -> Vec<(Uuid, &'static str, String, Uuid)> {
    let date =
        DateTime::parse_from_str("1999-01-08 00:00:00 +01:00", "%Y-%m-%d %H:%M:%S %z").unwrap();
    dbg!(date);
    vec![
        (
            Uuid::new_v4(),
            "2022-12-01 00:00:00 +01:00",
            test_utils::sample_base64_image(),
            user_id.clone(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-02 00:00:00 +01:00",
            test_utils::sample_base64_image(),
            user_id.clone(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-03 00:00:00 +01:00",
            test_utils::sample_base64_image(),
            user_id.clone(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-04 00:00:00 +01:00",
            test_utils::sample_base64_image(),
            user_id.clone(),
        ),
        (
            Uuid::new_v4(),
            "2022-12-05 00:00:00 +01:00",
            test_utils::sample_base64_image(),
            user_id.clone(),
        ),
    ]
}

fn compute_password_hash(password: &str) -> String {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.to_string().as_bytes(), &salt)
    .unwrap()
    .to_string();

    password_hash
}
