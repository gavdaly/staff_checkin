use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use std::sync::OnceLock;
    use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

    static DB: OnceLock<Pool<Postgres>> = OnceLock::new();

    async fn create_pool() -> Pool<Postgres> {
        let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(database_url.as_str())
            .await
            .expect("could not connect to database_url");

        // sqlx::migrate!()
        //     .run(&pool)
        //     .await
        //     .expect("migrations failed");

        pool
    }

    pub async fn init_db() -> Result<(), Pool<Postgres>> {
        DB.set(create_pool().await)
    }

    pub fn get_db<'a>() -> &'a Pool<Postgres> {
        DB.get().expect("database unitialized")
    }

}}
