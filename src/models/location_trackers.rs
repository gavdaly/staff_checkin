use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::database::get_db;
    use sqlx::types::BigDecimal;
    use uuid::Uuid;

    struct LocationTracker {
        pub id: Uuid,
        pub latitude: Option<BigDecimal>,
        pub longitude: Option<BigDecimal>,
        pub accuracy: Option<i32>,
    }

    pub async fn insert(lat: BigDecimal, long: BigDecimal, accuracy: i32) -> Result<LocationTracker, sqlx::Error> {
        let db = get_db();


        sqlx::query_as!(LocationTracker, "
            INSERT INTO location_trackers (latitude, longitude, accuracy)
                VALUES ($1, $2, $3)
                RETURNING id, latitude, longitude, accuracy", lat, long, accuracy)
            .fetch_one(db)
            .await
    }
}
}
