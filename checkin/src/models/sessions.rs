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

#[cfg(feature = "ssr")]
use crate::database::get_db;

#[cfg(feature = "ssr")]
pub async fn get_sessions_for(
    user_id: &Uuid,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<Vec<Session>, sqlx::Error> {
    let db = get_db();

    sqlx::query_as!(
        Session,
        "
            SELECT start_time, end_time, state, id, user_id
            FROM sessions
            WHERE user_id = $1 AND start_time BETWEEN $2 AND $3",
        user_id,
        start_date,
        end_date
    )
    .fetch_all(db)
    .await
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
