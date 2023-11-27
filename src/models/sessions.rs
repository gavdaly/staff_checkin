use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::corrections::Correction;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Active = 0,
    Editable = 1,
    Error = 2,
    Pending = 3,
    Accepted = 4,
    Rejected = 5,
    Finalized = 6,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub state: i32,
    pub id: Uuid,
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionAndCorrection {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub state: i32,
    pub user_id: Uuid,
    pub correction: Option<Correction>,
}

#[cfg(feature = "ssr")]
use crate::database::get_db;

#[cfg(feature = "ssr")]
pub async fn get_sessions_for(user_id: &Uuid,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,) -> Result<Vec<SessionAndCorrection>, sqlx::Error> {
        use crate::models::corrections::get_corrections_for;
        let db = get_db();
        let sessions = sqlx::query_as!(Session, r##"
        SELECT 
            id,
            start_time,
            end_time,
            state, 
            user_id
        FROM sessions
        WHERE user_id = $1 AND start_time BETWEEN $2 AND $3"##,
        user_id,
        start_date,
        end_date
    ).fetch_all(db).await?;

    let mut result = Vec::with_capacity(sessions.len());
    // TODO: remove N + 1 query. LEFT JOINs were only fetching the data as an inner join in sqlx.
    for session in sessions {
        let correction =  match get_corrections_for(&session.id).await {
            Ok(s) => s,
            Err(e) => {leptos::tracing::error!("Error getting collection: {e}"); None}
        };
        let s = SessionAndCorrection {
            id: session.id,
            start_time: session.start_time,
            end_time: session.end_time,
            state: session.state,
            user_id: session.user_id,
            correction
        };
        result.push(s);
    }

    Ok(result)
     
}

#[cfg(feature = "ssr")]
use chrono::Local;

#[cfg(feature = "ssr")]
pub async fn add_correction(id: Option<Uuid>, start_time: DateTime<Local>, end_time: DateTime<Local>, reason: String, user_id: Uuid) -> Result<(), sqlx::Error> {
    let db = get_db();
    match id {
        Some(id) => {
            let session = sqlx::query_as!(Session, "SELECT start_time, end_time, state, id, user_id
                FROM sessions
                WHERE id = $1 AND state = 1", id).fetch_one(db).await?;
            sqlx::query!("UPDATE sessions SET state = 3 WHERE id = $1", id).execute(db).await?;
            sqlx::query!("INSERT INTO corrections(start_time, end_time, original_start_time, original_end_time, new_start_time, new_end_time, session_id, reason) 
            VALUES ($1, $2, $1, $2, $3, $4, $5, $6)", session.start_time, session.end_time, start_time, end_time, id, reason).execute(db).await?;
        },
        None => {
            let session = sqlx::query_as!(Session, "INSERT INTO sessions(start_time, end_time, state, user_id)
                VALUES ($1, $2, 3, $3) RETURNING start_time, end_time, state, user_id, id", start_time, end_time, user_id).fetch_one(db).await?;
            sqlx::query!("INSERT INTO corrections(start_time, end_time, original_start_time, original_end_time, new_start_time, new_end_time, session_id, reason)
                VALUES ($1, $1, $1, $1, $1, $2, $3, $4)", start_time, end_time, session.id, reason).execute(db).await?;
        }
    }
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_open_sessions(user_id: &Uuid) -> Result<Vec<Session>, sqlx::Error> {
    let db = get_db();

    sqlx::query_as!(
        Session,
        "
            SELECT start_time, end_time, state, id, user_id
            FROM sessions
            WHERE user_id = $1 AND end_time IS NULL",
        user_id
    )
    .fetch_all(db)
    .await
}

#[cfg(feature = "ssr")]
pub async fn get_session(uuid: &Uuid) -> Result<Session, sqlx::Error> {
    let db = get_db();

    sqlx::query_as!(
        Session,
        "
            SELECT start_time, end_time, state, id, user_id
            FROM sessions
            WHERE id = $1",
        uuid
    )
    .fetch_one(db)
    .await
}

#[cfg(feature = "ssr")]
pub async fn get_open_session(user_id: &Uuid) -> Result<Session, sqlx::Error> {
    let db = get_db();

    sqlx::query_as!(
        Session,
        "
            SELECT start_time, end_time, state, id, user_id
            FROM sessions
            WHERE user_id = $1 AND end_time IS NULL",
        user_id
    )
    .fetch_one(db)
    .await
}

#[cfg(feature = "ssr")]
pub async fn close_session(id: &Uuid) -> Result<(), sqlx::Error> {
    let db = get_db();
    sqlx::query!(
        "UPDATE sessions SET end_time = NOW(), state = 1 WHERE id = $1",
        id
    )
    .execute(db)
    .await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn new_session(user_id: &Uuid) -> Result<Session, sqlx::Error> {
    let db = get_db();
    sqlx::query_as!(Session, "
            INSERT INTO sessions (user_id) VALUES ($1) RETURNING start_time, end_time, state, id, user_id
        ", user_id).fetch_one(db).await
}
