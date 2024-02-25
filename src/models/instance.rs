use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Instance {
    id: Uuid,
    user_id: Uuid,
    category: u32,
    reference_id: Uuid,
    date_time: DateTime<Local>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Category {
    Automated,
    MagicCode,
    Manual,
    Error,
}
