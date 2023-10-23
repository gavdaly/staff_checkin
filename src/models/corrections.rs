use cfg_if::cfg_if;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Correction {
    pub id: Uuid,
    pub assignation_id: Uuid,
    pub reason: Option<String>,
    pub response: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub duration: u64,
    pub original_start_time: NaiveDateTime,
    pub original_end_time: NaiveDateTime,
    pub original_duration: u64,
    pub assignation_integer_id: u64,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    // use sqlx;

    struct SqlCorrection {
        integer_id: u64,
        reason: Option<String>,
        response: Option<String>,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        original_start_time: NaiveDateTime,
        original_end_time: NaiveDateTime,
        assignation_integer_id: u64,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        id: Uuid,
        assignation_id: Uuid,
    }
}
}
