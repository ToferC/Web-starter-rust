use uuid::Uuid;
use chrono::NaiveDateTime;

pub struct ToDo {
    id: Uuid,
    user_id: Uuid,
    description: String,
    due_date: NaiveDateTime,
    priority: Priority,
}

pub enum Priority {
    Low,
    Medium,
    High,
}