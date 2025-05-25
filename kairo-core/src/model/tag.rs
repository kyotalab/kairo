use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::fmt;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: String,
    pub tag_name: String,
    pub created_at: NaiveDateTime,
    pub deleted: bool,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TagID: {}", self.id)?;
        writeln!(f, "TagName: {}", self.tag_name)?;

        Ok(())
    }
}
