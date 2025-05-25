use core::fmt;

use crate::schema::tasks;
use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::*,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
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

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        if let Some(str) = &self.description {
            writeln!(f, "Description: {:?}", str)?;
        } else {
            writeln!(f, "Description: No description")?;
        }
        if let Some(pri) = &self.priority {
            writeln!(f, "Priority: {:?}", pri)?;
        }
        if let Some(due) = &self.due_date {
            writeln!(f, "DueDate: {}", due.format("%Y/%m/%d").to_string())?;
        } else {
            writeln!(f, "DueDate: No due date set")?;
        }
        if let Some(pid) = &self.project_id {
            writeln!(f, "Project: {:?}", pid)?;
        } else {
            writeln!(f, "Project: No related project")?;
        }
        writeln!(
            f,
            "Created: {}",
            self.created_at.format("%Y/%m/%d %H:%M:%S").to_string()
        )?;
        writeln!(
            f,
            "Updated: {}",
            self.updated_at.format("%Y/%m/%d %H:%M:%S").to_string()
        )?;
        writeln!(f, "Archived: {}", self.archived)?;
        writeln!(f, "Deleted: {}", self.deleted)?;
        Ok(())
    }
}
