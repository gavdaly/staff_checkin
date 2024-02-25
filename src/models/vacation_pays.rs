use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct VacationPay {
    id: Uuid,
    user_id: Uuid,
    amount: Amount,
    requested_for: NaiveDate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Amount {
    Dollar(u32),
    Duration(u32),
    Remainder,
    None,
}
