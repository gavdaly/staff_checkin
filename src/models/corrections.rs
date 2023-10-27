use cfg_if::cfg_if;
use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Correction {
    pub id: Uuid,
    pub session_id: Uuid,
    pub reason: String,
    pub response: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub original_start_time: DateTime<Utc>,
    pub original_end_time: DateTime<Utc>,
    pub new_start_time: DateTime<Utc>,
    pub new_end_time: DateTime<Utc>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::get_db;

    pub async fn get_corrections_for(session_id: &Uuid) -> Result<Option<Correction>, sqlx::Error> {
        let db = get_db();
        sqlx::query_as!(Correction, "
            SELECT id, session_id, reason, response, start_time, end_time, original_start_time, original_end_time, new_start_time, new_end_time
                FROM corrections
                WHERE session_id = $1
            ", session_id).fetch_optional(db).await

    }



}
}
