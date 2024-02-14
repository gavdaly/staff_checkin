use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local};

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
