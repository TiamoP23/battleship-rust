use sqlx::{postgres::PgPoolOptions, query, PgPool};
use tokio::sync::OnceCell;

pub static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_database_connection() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    DB_POOL.set(pool).expect("Failed to set database pool");
}
