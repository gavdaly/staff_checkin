use cfg_if::cfg_if;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Adjustment {
    pub category: Category,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub duration: u64,
    pub reason: String,
    pub response: String,
    pub state: State,
    pub id: Uuid,
    pub user_id: Uuid,
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
    use chrono::{  NaiveDateTime};
    use sqlx::PgConnection;
    use crate::utils::db;

    struct SqlAdjustment {
        integer_id: u64,
        category: usize,
        start_date: NaiveDate,
        end_date: NaiveDate,
        duration: u32,
        reason: String,
        response: String,
        state: usize,
        user_integer_id: u64,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        id: Uuid,
        user_id: Uuid,
    }

    pub fn get_adjustments_for(user_id: &Uuid) -> Vec<SqlAdjustment> {
        let db = db();


        // connection.close();
        vec![]
    }

}
}
