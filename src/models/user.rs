use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub provider: Option<i32>,
    pub phone_number: String,
    pub display_name: Option<String>,
    pub api_id: Option<i32>,
    pub state: State,
    pub settings: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Inactive = 0,
    Salary = 1,
    Hourly = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub state: i32,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::*;

    impl UserPublic {
        pub async fn get_all_hourly() -> Result<Vec<Self>, sqlx::Error>  {
            use crate::database;
            let db = database::get_db();
            query_as!(UserPublic, "SELECT id, last_name, first_name, phone_number, state From users
                            WHERE state = 2
                            ORDER BY last_name, first_name;").fetch_all(db).await
        }


        pub async fn get(id: Uuid) -> Result<Self, sqlx::Error> {
            use crate::database;
            let db = database::get_db();
            query_as!(UserPublic, "SELECT id, last_name, first_name, phone_number, state From users
                            WHERE id = $1
                            ORDER BY last_name, first_name;", id).fetch_one(db).await
        }

        pub async fn get_phone(phone: &str) -> Result<Self, sqlx::Error> {
          use crate::database;
          let db = database::get_db();
          query_as!(UserPublic, "SELECT id, last_name, first_name, phone_number, state From users WHERE phone_number = $1;", phone).fetch_one(db).await
      }
    }
}
}
