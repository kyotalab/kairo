use crate::schema::notes;
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

// --- ToSql<Text, Sqlite> 実装 ---
impl ToSql<Text, Sqlite> for NoteType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let value = match self {
            NoteType::Fleeting => "fleeting",
            NoteType::Permanent => "permanent",
        };
        <str as ToSql<Text, Sqlite>>::to_sql(value, out)
    }
}

// --- FromSql<Text, Sqlite> 実装 ---
impl FromSql<Text, Sqlite> for NoteType {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <*const str as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match unsafe { &*s } {
            "fleeting" => Ok(NoteType::Fleeting),
            "permanent" => Ok(NoteType::Permanent),
            other => Err(format!("Unrecognized NoteType variant: {}", other).into()),
        }
    }
}

// --- ToSql<Text, Sqlite> 実装 ---
impl ToSql<Text, Sqlite> for SubType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        let value = match self {
            SubType::Question => "question",
            SubType::Investigation => "investigation",
            SubType::Log => "log",
            SubType::Idea => "idea",
            SubType::Reference => "reference",
            SubType::Literature => "literature",
            SubType::Quote => "quote",
        };
        <str as ToSql<Text, Sqlite>>::to_sql(value, out)
    }
}

// --- FromSql<Text, Sqlite> 実装 ---
impl FromSql<Text, Sqlite> for SubType {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = <*const str as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match unsafe { &*s } {
            "question" => Ok(SubType::Question),
            "investigation" => Ok(SubType::Investigation),
            "log" => Ok(SubType::Log),
            "idea" => Ok(SubType::Idea),
            "reference" => Ok(SubType::Reference),
            "literature" => Ok(SubType::Literature),
            "quote" => Ok(SubType::Quote),
            other => Err(format!("Unrecognized NoteType variant: {}", other).into()),
        }
    }
}
