use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sqlite::Sqlite;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived: bool,
    pub deleted: bool,
}
