use crate::models::{Note, NoteType, SubType};
use crate::schema::notes;
use crate::schema::notes::dsl::*;
use chrono::{NaiveDateTime, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::result::Error;

// Create用
#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote {
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

// Update用
#[derive(AsChangeset)]
#[diesel(table_name = notes)]
pub struct UpdatedNote {
    pub title: String,
    pub note_type: NoteType,
    pub sub_type: Option<SubType>,
    pub updated_at: NaiveDateTime,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

// ==========================================================================-
pub fn create_note_id() -> String {
    Utc::now().format("%Y%m%dT%H%M%S").to_string()
}

fn parse_note_type(input: &str) -> Result<NoteType, Error> {
    match input {
        "fleeting" => Ok(NoteType::Fleeting),
        "permanent" => Ok(NoteType::Permanent),
        other => Err(Error::QueryBuilderError(
            format!("Invalid note_type: {}", other).into(),
        )),
    }
}

fn parse_sub_type(input: &str) -> Result<Option<SubType>, Error> {
    match input {
        "question" => Ok(Some(SubType::Question)),
        "investigation" => Ok(Some(SubType::Investigation)),
        "log" => Ok(Some(SubType::Log)),
        "idea" => Ok(Some(SubType::Idea)),
        "reference" => Ok(Some(SubType::Reference)),
        "literature" => Ok(Some(SubType::Literature)),
        "quote" => Ok(Some(SubType::Quote)),
        "" | "_" => Ok(None),
        other => Err(Error::QueryBuilderError(
            format!("Invalid sub_type: {}", other).into(),
        )),
    }
}

// ==========================================================================-
pub fn create_note(
    conn: &mut SqliteConnection,
    input_title: String,
    input_note_type: &str,
    input_sub_type: &str,
    input_project_id: Option<String>,
    input_task_id: Option<String>,
) -> Result<Note, Error> {
    let validated_note_type = parse_note_type(&input_note_type)?;
    let validated_sub_type = parse_sub_type(&input_sub_type)?;

    let new_note = NewNote {
        id: create_note_id(),
        title: input_title,
        note_type: validated_note_type,
        sub_type: validated_sub_type,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        archived: false,
        deleted: false,
        project_id: input_project_id,
        task_id: input_task_id,
    };

    diesel::insert_into(notes::table)
        .values(&new_note)
        .returning(Note::as_select()) // Diesel 2.x
        .get_result(conn)
}

// ==========================================================================-
pub fn list_notes(
    conn: &mut SqliteConnection,
    include_archived: bool,
    include_deleted: bool,
) -> Result<Vec<Note>, Error> {
    let mut query = notes.into_boxed();

    if !include_archived && !include_deleted {
        query = query.filter(archived.eq(false)).filter(deleted.eq(false));
    } else if include_archived && !include_deleted {
        query = query.filter(archived.eq(true)).filter(deleted.eq(false));
    } else if !include_archived && include_deleted {
        query = query.filter(archived.eq(false)).filter(deleted.eq(true));
    } else {
        return Err(Error::QueryBuilderError(
            "Invalid combination: archived=true AND deleted=true".into(),
        ));
    }

    Ok(query
        .select(Note::as_select())
        .order(created_at.desc())
        .load(conn)?)
}

// ==========================================================================-
pub fn get_note_by_id(conn: &mut SqliteConnection, note_id: &str) -> Result<Option<Note>, Error> {
    let note = notes
        .find(note_id)
        .select(Note::as_select())
        .first(conn)
        .optional()?;

    Ok(note)
}

// ==========================================================================-
pub fn update_note(
    conn: &mut SqliteConnection,
    note_id: &str,
    updated_title: String,
    updated_note_type: &str,
    updated_sub_type: &str,
    updated_project_id: Option<String>,
    updated_task_id: Option<String>,
) -> Result<Note, Error> {
    let validated_note_type = parse_note_type(&updated_note_type)?;
    let validated_sub_type = parse_sub_type(&updated_sub_type)?;

    let updated_note = UpdatedNote {
        title: updated_title,
        note_type: validated_note_type,
        sub_type: validated_sub_type,
        updated_at: Utc::now().naive_utc(),
        project_id: updated_project_id,
        task_id: updated_task_id,
    };

    diesel::update(notes.find(note_id))
        .set(updated_note)
        .returning(Note::as_select())
        .get_result(conn)
}
