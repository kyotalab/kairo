use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: String,
    pub tag_name: String,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    pub deleted: bool,
}
