use crate::schema::tasks;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[DieselType = "Task_priority"]
pub enum TaskPriority {
    #[db_rename = "low"]
    Low,
    #[db_rename = "medium"]
    Medium,
    #[db_rename = "high"]
    High,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
    pub deleted: bool,
    pub project_id: Option<String>,
}

// Optional: デフォルトを持たせたい場合
impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Medium
    }
}
