use cfg_if::cfg_if;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Correction {
    pub id: Uuid,
    pub assignation_id: Option<Uuid>,
    pub reason: Option<String>,
    pub response: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub original_start_time: Option<NaiveDateTime>,
    pub original_end_time: Option<NaiveDateTime>,
    pub new_start_time: Option<NaiveDateTime>,
    pub new_end_time: Option<NaiveDateTime>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::get_db;

    pub async fn get_corrections_for(assignation_id: &Uuid) -> Result<Option<Correction>, sqlx::Error> {
        let db = get_db();
        sqlx::query_as!(Correction, "
            SELECT id, assignation_id, reason, response, start_time, end_time, original_start_time, original_end_time, new_start_time, new_end_time
                FROM corrections
                WHERE assignation_id = $1
            ", assignation_id).fetch_optional(db).await

    }



}
}
