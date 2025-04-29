use chrono::{DateTime, Utc};
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

impl ToSql<Text, Sqlite> for TaskPriority {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
        };
        out.write_all(value.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for TaskPriority {
    fn from_sql(bytes: diesel::backend::RawValue<Sqlite>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"low" => Ok(TaskPriority::Low),
            b"medium" => Ok(TaskPriority::Medium),
            b"high" => Ok(TaskPriority::High),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
