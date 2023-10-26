use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocationTracker {
    pub id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<i32>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::get_db;


    pub async fn insert(_lat: f64, _long: f64, _accuracy: i32) -> Result<LocationTracker, sqlx::Error> {
        let _db = get_db();

    //     sqlx::query_as!(LocationTracker, "
    //         INSERT INTO location_trackers (latitude, longitude, accuracy)
    //             VALUES ($1, $2, $3)
    //             RETURNING id, latitude, longitude, accuracy", lat as BigDecimal, long as BigDecimal, accuracy)
    //         .fetch_one(db)
    //         .await
        todo!()
    }
}
}
