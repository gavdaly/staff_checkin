#[cfg(feature = "ssr")]
use {
    sqlx::{postgres::PgPoolOptions, Pool, Postgres},
    std::sync::OnceLock,
};

/// Static variable representing a connection pool to a PostgreSQL database.
/// 
/// The `DB` variable is of type `OnceLock<Pool<Postgres>>` and is declared as `static`, meaning it will have a single instance shared across all threads.
/// The `OnceLock` type ensures that the variable is only initialized once, even if the code is executed multiple times.
#[cfg(feature = "ssr")]
static DB: OnceLock<Pool<Postgres>> = OnceLock::new();

/// Asynchronous function that creates a connection pool to a PostgreSQL database using the 'sqlx' crate.
///
/// # Returns
/// The function returns a connection pool of type 'Pool<Postgres>' from the 'sqlx' crate.
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

/// Initializes a connection pool to a PostgreSQL database.
///
/// # Returns
///
/// Returns a `Result` indicating success or failure. If the connection pool is successfully initialized, it returns an empty `Ok` value. Otherwise, it returns a `Result` with an error indicating the reason for failure.
#[cfg(feature = "ssr")]
pub async fn init_db() -> Result<(), Pool<Postgres>> {
    DB.set(create_pool().await)
}

/// Returns a reference to a connection pool of type `Pool<Postgres>`.
///
/// # Panics
///
/// If the `get` method returns `None`, the function will panic with the error message "database uninitialized".
#[cfg(feature = "ssr")]
pub fn get_db<'a>() -> &'a Pool<Postgres> {
    DB.get().expect("database unitialized")
}
