use crate::schema::projects;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
    pub deleted: bool,
}
