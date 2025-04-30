use crate::schema::tags;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: String,
    pub tag_name: String,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
}
