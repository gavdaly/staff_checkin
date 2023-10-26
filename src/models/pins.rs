use cfg_if::cfg_if;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pin {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub number: Option<String>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database;

    impl Pin {
        pub async fn create_pin_for(user_id: Uuid) -> Result<Self, sqlx::Error> {
            use rand::Rng;
            use rand::thread_rng;
            let mut rng = thread_rng();
            let numb: usize = rng.gen_range(100000..999999);

            let db = database::get_db();
            let _ = sqlx::query!("DELETE FROM pins WHERE user_id = $1", user_id).execute(db).await;
            sqlx::query_as!(Pin, "
                INSERT
                    INTO pins
                        (user_id, number)
                    VALUES
                        ($1, $2)
                    RETURNING id, user_id, number;
                ", user_id, numb.to_string()).fetch_one(db).await
        }
    }
}
}
