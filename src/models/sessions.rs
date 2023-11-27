use chrono::{DateTime, Utc};
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
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub state: i32,
    pub id: Uuid,
    pub user_id: Uuid,
    pub new_start_time: Option<DateTime<Utc>>,
    pub new_end_time: Option<DateTime<Utc>>,
    pub original_start_time: Option<DateTime<Utc>>,
    pub original_end_time: Option<DateTime<Utc>>,
    pub reason: Option<String>,
    pub response: Option<String>
}

#[cfg(feature = "ssr")]
use crate::database::get_db;

#[cfg(feature = "ssr")]
pub async fn get_sessions_for(user_id: &Uuid,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,) -> Result<Vec<SessionAndCorrection>, sqlx::Error> {
        let db = get_db();
        sqlx::query_as!(SessionAndCorrection, r#"
SELECT
    s.start_time,
    s.end_time,
    s.id,
    s.state,
    s.user_id,
    c.new_start_time,
    c.new_end_time,
    c.original_start_time,
    c.original_end_time,
    c.reason,
    c.response
FROM sessions AS s
LEFT JOIN corrections AS c
ON s.id = c.session_id
WHERE s.user_id = $1 AND s.start_time BETWEEN $2 AND $3
ORDER BY s.start_time;"#,
        user_id,
        start_date,
        end_date).fetch_all(db).await
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
