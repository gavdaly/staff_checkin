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

#[cfg(feature = "ssr")]
pub async fn correction_response(id: Uuid, state: u32, response: &str) -> Result<(), sqlx::Error> {
    use crate::database::get_db;
    let db = get_db();
    sqlx::query!(
        "
            UPDATE corrections
                SET response = $1
                WHERE id = $2
            ",
        response,
        id
    )
    .execute(db)
    .await?;
    if state == 4 {
        sqlx::query!(
            "
                UPDATE sessions
                    SET start_time = (SELECT new_start_time FROM corrections WHERE id = $1),
                        end_time = (SELECT new_end_time FROM corrections WHERE id = $1),
                        state = 4
                    WHERE id = (SELECT session_id FROM corrections WHERE id = $1)
                ",
            id
        )
        .execute(db)
        .await?;
    }
    if state == 5 {
        sqlx::query!(
            "
                UPDATE sessions
                    SET state = 5
                    WHERE id = (SELECT session_id FROM corrections WHERE id = $1)
                ",
            id
        )
        .execute(db)
        .await?;
    }
    Ok(())
}
