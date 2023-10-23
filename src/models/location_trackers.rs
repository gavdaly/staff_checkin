use cfg_if::cfg_if;
use uuid::Uuid;

struct LocationTracker {
    pub id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: usize,
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use chrono::NaiveDateTime;

    struct SqlLocationTracker {
        id: Uuid,
        latitude: f64,
        longitude: f64,
        accuracy: usize,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    }
}
}
