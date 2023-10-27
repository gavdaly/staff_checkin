use cfg_if::cfg_if;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub number: i32,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database;

    impl Pin {
        pub async fn create_pin_for(user_id: Uuid) -> Result<Self, sqlx::Error> {
            let db = database::get_db();
            let _ = sqlx::query!("DELETE FROM pins WHERE user_id = $1", user_id).execute(db).await;
            sqlx::query_as!(Pin, "
                INSERT
                    INTO pins
                        (user_id)
                    VALUES
                        ($1)
                    RETURNING id, user_id, number;
                ", user_id).fetch_one(db).await
        }

        pub async fn get_pin(number: i32) -> Result<Pin, sqlx::Error> {
            let db = database::get_db();
            sqlx::query_as!(Pin, "
                    SELECT id, user_id, number FROM pins WHERE number = $1
                ", number).fetch_one(db).await
        }
    }
}
}
