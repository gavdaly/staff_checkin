use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Adjustment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: i32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub duration: i32,
    pub reason: String,
    pub response: String,
    pub state: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Category {
    Admin = 0,
    Vacation = 1,
    Statutory = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum State {
    Error = 0,
    Pending = 1,
    Accepted = 2,
    Rejected = 3,
    Finalized = 4,
}

#[cfg(feature = "ssr")]
pub async fn create_adjustment(
    user_id: &Uuid,
    date: NaiveDate,
    time: i32,
    reason: &str,
) -> Result<(), sqlx::Error> {
    use crate::database::get_db;
    let db = get_db();

    sqlx::query!(
        "INSERT INTO adjustments (user_id, category, start_date, duration, reason, state)
        VALUES ($1, 0, $2, $3, $4, 4)",
        user_id,
        date,
        time,
        reason
    )
    .execute(db)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_adjustments_for(
    user_id: &Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<Adjustment>, sqlx::Error> {
    use crate::database::get_db;
    let db = get_db();

    sqlx::query_as!(
        Adjustment,
        "
            SELECT category, start_date, end_date, duration, reason, response, state, id, user_id
            FROM adjustments
            WHERE user_id = $1 AND start_date BETWEEN $2 AND $3",
        user_id,
        start_date,
        end_date
    )
    .fetch_all(db)
    .await
}
