use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocationTracker {
    pub id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<f64>,
}

#[cfg(feature = "ssr")]
pub async fn insert(lat: f64, long: f64, accuracy: f64) -> Result<LocationTracker, sqlx::Error> {
    use crate::database::get_db;
    let db = get_db();

    sqlx::query_as!(
        LocationTracker,
        "
            INSERT INTO location_trackers (latitude, longitude, accuracy)
                VALUES ($1, $2, $3)
                RETURNING id, latitude, longitude, accuracy",
        lat,
        long,
        accuracy
    )
    .fetch_one(db)
    .await
}
