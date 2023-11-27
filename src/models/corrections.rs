use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Correction {
    pub id: Uuid,
    pub session_id: Uuid,
    pub reason: String,
    pub response: String,
    pub original_start_time: DateTime<Utc>,
    pub original_end_time: DateTime<Utc>,
    pub new_start_time: DateTime<Utc>,
    pub new_end_time: DateTime<Utc>,
}

#[cfg(feature = "ssr")]
pub async fn get_corrections_for(session_id: &Uuid) -> Result<Option<Correction>, sqlx::Error> {
    use crate::database::get_db;
    let db = get_db();
    sqlx::query_as!(Correction, "
            SELECT id, session_id, reason, response, original_start_time, original_end_time, new_start_time, new_end_time
                FROM corrections
                WHERE session_id = $1
            ", session_id).fetch_optional(db).await
}
