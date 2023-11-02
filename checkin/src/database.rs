#[cfg(feature = "ssr")]
use {
    sqlx::{postgres::PgPoolOptions, Pool, Postgres},
    std::sync::OnceLock,
};

#[cfg(feature = "ssr")]
static DB: OnceLock<Pool<Postgres>> = OnceLock::new();

#[cfg(feature = "ssr")]
async fn create_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    let _ = sqlx::migrate!().run(&pool).await;

    pool
}

#[cfg(feature = "ssr")]
pub async fn init_db() -> Result<(), Pool<Postgres>> {
    DB.set(create_pool().await)
}

#[cfg(feature = "ssr")]
pub fn get_db<'a>() -> &'a Pool<Postgres> {
    DB.get().expect("database unitialized")
}
