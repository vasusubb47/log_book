use std::env::var;

use sqlx::{Pool, Postgres};

pub async fn db_connection() -> Pool<Postgres> {
    let url = var("DATABASE_URL").expect("Couldn't find database url from environment variable.");
    let pool = sqlx::postgres::PgPool::connect(&url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./sql")
        .run(&pool)
        .await
        .expect("Failed to migrate");

    pool
}
