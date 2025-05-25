use core::fmt;

use crate::schema::projects;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
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

impl fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        if let Some(str) = &self.description {
            writeln!(f, "Description: {:?}", str)?;
        } else {
            writeln!(f, "Description: No description")?;
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
