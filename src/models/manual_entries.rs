use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct ManualEntry {
    id: Uuid,
    instance_id: Uuid,
    category: Category,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Category {
    Pending,
    NeedEditing,
    Accepted,
    Rejected,
}
