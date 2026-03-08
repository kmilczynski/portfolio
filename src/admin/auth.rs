use crate::admin::db::DbPool;
use chrono::{Duration, Utc};
use std::env;
use uuid::Uuid;

pub fn verify_password(input: &str) -> bool {
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_default();
    if admin_password.is_empty() {
        eprintln!("Warning: ADMIN_PASSWORD environment variable is not set");
        return false;
    }
    input == admin_password
}

pub async fn create_session(pool: &DbPool) -> Result<String, sqlx::Error> {
    let session_id = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);
    let expires_at_str = expires_at.format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query("INSERT INTO admin_sessions (id, expires_at) VALUES (?, ?)")
        .bind(&session_id)
        .bind(&expires_at_str)
        .execute(pool)
        .await?;

    Ok(session_id)
}

pub async fn validate_session(pool: &DbPool, token: &str) -> Result<bool, sqlx::Error> {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let result: Option<(i64,)> =
        sqlx::query_as("SELECT 1 FROM admin_sessions WHERE id = ? AND expires_at > ?")
            .bind(token)
            .bind(&now)
            .fetch_optional(pool)
            .await?;

    Ok(result.is_some())
}

pub async fn delete_session(pool: &DbPool, token: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM admin_sessions WHERE id = ?")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}
