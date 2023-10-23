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
    pub integer_id: u64,
    pub key: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub state: State,
    pub id: Uuid,
    pub user_id: Uuid,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    struct SqlAssignation {
        integer_id: u64,
        key: String,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        state: i32,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        id: Uuid,
        user_id: Uuid,
    }

}
}
