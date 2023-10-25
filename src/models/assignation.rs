use cfg_if::cfg_if;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub enum State {
    Active = 0,
    Editable = 1,
    Error = 2,
    Pending = 3,
    Accepted = 4,
    Rejected = 5,
    Finalized = 6,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Assignation {
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub state: Option<i32>,
    pub id: Uuid,
    pub user_id: Option<Uuid>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::get_db;
    use std::ops::Range;


    pub async fn get_assignation_for(user_id: &Uuid, start_date: NaiveDateTime, end_date: NaiveDateTime) -> Result<Vec<Assignation>, sqlx::Error> {
        let db = get_db();

        sqlx::query_as!(Assignation, "
            SELECT start_time, end_time, state, id, user_id
            FROM assignations
            WHERE user_id = $1 AND start_time BETWEEN $2 AND $3", user_id, start_date, end_date).fetch_all(db).await
    }

}
}
