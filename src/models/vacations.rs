use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Vacation {
    id: Uuid,
    user_id: Uuid,
    category: i32,
    start_on: NaiveDate,
    end_on: NaiveDate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Category {
    Pending,
    Accepted,
    Rejected,
    Done,
}
