use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, AsChangeset, Clone)]
#[table("todos")]
pub struct ToDo {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: String,
    priority: Priority,
    due_date: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub enum Priority {
    Low,
    Medium,
    High,
}