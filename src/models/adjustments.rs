use cfg_if::cfg_if;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Adjustment {
    pub category: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub duration: Option<i32>,
    pub reason: Option<String>,
    pub response: Option<String>,
    pub state: Option<i32>,
    pub id: Uuid,
    pub user_id: Option<Uuid>,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Category {
    Admin = 0,
    Vacation = 1,
    Statutory = 2,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum State {
    Error = 0,
    Pending = 1,
    Accepted = 2,
    Rejected = 3,
    Finalized = 4,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::get_db;

    pub async fn get_adjustments_for(user_id: &Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<Adjustment>, sqlx::Error> {
        let db = get_db();

        sqlx::query_as!(Adjustment, "
            SELECT category, start_date, end_date, duration, reason, response, state, id, user_id
            FROM adjustments
            WHERE user_id = $1 AND start_date BETWEEN $2 AND $3", user_id, start_date, end_date).fetch_all(db).await
    }
}
}
