use crate::schema::tasks;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::*;
use diesel::prelude::*;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
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

// --- ToSql<Text, Sqlite> 実装 ---
impl ToSql<Text, Sqlite> for TaskPriority {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let value = match self {
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
        };
        <str as ToSql<Text, Sqlite>>::to_sql(value, out)
    }
}

// --- FromSql<Text, Sqlite> 実装 ---
impl FromSql<Text, Sqlite> for TaskPriority {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <*const str as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match unsafe { &*s } {
            "low" => Ok(TaskPriority::Low),
            "medium" => Ok(TaskPriority::Medium),
            "high" => Ok(TaskPriority::High),
            other => Err(format!("Unrecognized TaskPriority variant: {}", other).into()),
        }
    }
}
