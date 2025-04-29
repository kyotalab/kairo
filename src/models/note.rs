use chrono::{DateTime, Utc};
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum NoteType {
    Fleeting,
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum SubType {
    Question,
    Investigation,
    Log,
    Idea,
    Reference,
    Literature,
    Quote,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    pub id: String,
    pub title: String,
    pub note_type: NoteType,
    pub sub_type: Option<SubType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived: bool,
    pub deleted: bool,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

// NoteType
impl ToSql<Text, Sqlite> for NoteType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            NoteType::Fleeting => "fleeting",
            NoteType::Permanent => "permanent",
        };
        out.write_all(value.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for NoteType {
    fn from_sql(bytes: diesel::backend::RawValue<Sqlite>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"fleeting" => Ok(NoteType::Fleeting),
            b"permanent" => Ok(NoteType::Permanent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

// SubType
impl ToSql<Text, Sqlite> for SubType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            SubType::Question => "question",
            SubType::Investigation => "investigation",
            SubType::Log => "log",
            SubType::Idea => "idea",
            SubType::Reference => "reference",
            SubType::Literature => "literature",
            SubType::Quote => "quote",
        };
        out.write_all(value.as_bytes())?;
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for SubType {
    fn from_sql(bytes: diesel::backend::RawValue<Sqlite>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"question" => Ok(SubType::Question),
            b"investigation" => Ok(SubType::Investigation),
            b"log" => Ok(SubType::Log),
            b"idea" => Ok(SubType::Idea),
            b"reference" => Ok(SubType::Reference),
            b"literature" => Ok(SubType::Literature),
            b"quote" => Ok(SubType::Quote),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
