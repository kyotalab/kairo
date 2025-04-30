use crate::models::{Note, NoteType, SubType};
use crate::schema::notes;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote {
    pub id: String,
    pub title: String,
    pub note_type: NoteType,
    pub sub_type: Option<SubType>,

    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,

    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,

    pub archived: bool,
    pub deleted: bool,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

pub fn create_note_id() -> String {
    Utc::now().format("%Y%m%dT%H%M%S").to_string()
}

pub fn create_note(
    conn: &mut SqliteConnection,
    title: String,
    note_type_str: &str,
    sub_type_str: &str,
    project_id: Option<String>,
    task_id: Option<String>,
) -> Result<Note, Error> {
    let note_type = match note_type_str {
        "fleeting" => NoteType::Fleeting,
        "permanent" => NoteType::Permanent,
        other => {
            return Err(Error::QueryBuilderError(
                format!("Invalid note_type: {}", other).into(),
            ));
        }
    };

    let sub_type = match sub_type_str {
        "question" => Some(SubType::Question),
        "investigation" => Some(SubType::Investigation),
        "log" => Some(SubType::Log),
        "idea" => Some(SubType::Idea),
        "reference" => Some(SubType::Reference),
        "literature" => Some(SubType::Literature),
        "quote" => Some(SubType::Quote),
        "" | "_" => None, // 入力なしならNone
        other => {
            return Err(Error::QueryBuilderError(
                format!("Invalid sub_type: {}", other).into(),
            ));
        }
    };

    let new_note = NewNote {
        id: create_note_id(),
        title,
        note_type,
        sub_type,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        archived: false,
        deleted: false,
        project_id,
        task_id,
    };

    diesel::insert_into(notes::table)
        .values(&new_note)
        .returning(Note::as_select()) // Diesel 2.x
        .get_result(conn)
}
