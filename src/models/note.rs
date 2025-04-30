use crate::schema::notes;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[DieselType = "Note_type"]
pub enum NoteType {
    #[db_rename = "fleeting"]
    Fleeting,
    #[db_rename = "permanent"]
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
#[DieselType = "Sub_type"]
pub enum SubType {
    #[db_rename = "question"]
    Question,
    #[db_rename = "investigation"]
    Investigation,
    #[db_rename = "log"]
    Log,
    #[db_rename = "idea"]
    Idea,
    #[db_rename = "reference"]
    Reference,
    #[db_rename = "literature"]
    Literature,
    #[db_rename = "quote"]
    Quote,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = notes)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub note_type: NoteType,
    pub sub_type: Option<SubType>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
    pub deleted: bool,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}
