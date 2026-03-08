use once_cell::sync::OnceCell;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;
use std::path::Path;

pub type DbPool = SqlitePool;

static POOL: OnceCell<DbPool> = OnceCell::new();

pub async fn create_sqlite_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:data/portfolio.db".to_string());

    // Ensure the data directory exists
    if let Some(db_path) = database_url.strip_prefix("sqlite:") {
        if let Some(parent) = Path::new(db_path).parent() {
            std::fs::create_dir_all(parent).ok();
        }
    }

    // Add ?mode=rwc to create the database if it doesn't exist
    let url_with_create = if database_url.contains('?') {
        format!("{}&mode=rwc", database_url)
    } else {
        format!("{}?mode=rwc", database_url)
    };

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url_with_create)
        .await?;

    run_migrations(&pool).await?;

    Ok(pool)
}

pub fn init_global_pool(pool: DbPool) {
    let _ = POOL.set(pool);
}

pub fn get_pool() -> Option<&'static DbPool> {
    POOL.get()
}

async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Run initial schema migration
    let migration_sql = include_str!("../../migrations/001_create_tables.sql");
    for statement in migration_sql.split(';') {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            sqlx::query(stmt).execute(pool).await?;
        }
    }

    // Run additive migrations, ignoring "duplicate column" errors (already applied)
    let views_migration = include_str!("../../migrations/002_add_views.sql");
    for statement in views_migration.split(';') {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            if let Err(e) = sqlx::query(stmt).execute(pool).await {
                let msg = e.to_string();
                if !msg.contains("duplicate column name") {
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}
