use crate::{
    model::{Note, NoteType, SubType},
    schema::{
        note_tags,
        notes::{self, dsl::*},
        tags,
    },
    store::*,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{SqliteConnection, prelude::*, result::Error};

use super::get_tag_by_name;

// ==============================
// ▼ Structs / Create
// ==============================
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

// ==============================
// ▼ Structs / Update
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = notes)]
pub struct UpdatedNote {
    pub title: Option<String>,
    pub note_type: NoteType,
    pub sub_type: Option<SubType>,
    pub updated_at: NaiveDateTime,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
}

// ==============================
// ▼ Structs / Archive
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = notes)]
pub struct ArchivedNote {
    pub archived: bool,
}

// ==============================
// ▼ Structs / SoftDelete
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = notes)]
pub struct SoftDeletedNote {
    pub deleted: bool,
}

// ==============================
// ▼ Create / Insert
// ==============================
fn generate_note_id() -> String {
    Utc::now().format("%Y%m%dT%H%M%S").to_string()
}

pub fn create_note(
    conn: &mut SqliteConnection,
    input_title: String,
    input_note_type: &str,
    input_sub_type: &str,
    input_project_id: Option<String>,
    input_task_id: Option<String>,
    input_tag_names: Option<Vec<String>>,
) -> Result<Note, Error> {
    let validated_note_type = parse_note_type(&input_note_type)?;
    let validated_sub_type = parse_sub_type(&input_sub_type)?;
    if let Some(ref pid) = input_project_id {
        ensure_project_exists(conn, pid)?;
    }

    let new_note = NewNote {
        id: generate_note_id(),
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

    // Note を保存
    let note = diesel::insert_into(notes::table)
        .values(&new_note)
        .returning(Note::as_select())
        .get_result(conn)?;

    // Tag と NoteTag の保存処理
    if let Some(tag_names) = input_tag_names {
        for name in tag_names {
            // タグ取得または作成
            let tag = match get_tag_by_name(conn, name.clone()) {
                Ok(Some(existing)) => existing,
                Ok(None) => create_tag(conn, name.clone())?,
                Err(e) => return Err(e),
            };

            // note_tag を作成
            create_note_tag(conn, &note.id, &tag.id)?;
        }
    }

    Ok(note)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn list_notes(
    conn: &mut SqliteConnection,
    include_archived: Option<bool>,
    include_deleted: Option<bool>,
    include_tags: Option<Vec<String>>,
    include_order: Option<String>,
) -> Result<Vec<Note>, Error> {
    let archived_flag = include_archived.unwrap_or(false);
    let deleted_flag = include_deleted.unwrap_or(false);

    if archived_flag && deleted_flag {
        return Err(Error::QueryBuilderError(
            "Invalid combination: archived=true AND deleted=true".into(),
        ));
    }

    // タグフィルターがある場合：JOIN込みクエリで返す
    if let Some(tags_filter) = include_tags {
        let tag_filtered_query = notes
            .inner_join(note_tags::table.on(notes::id.eq(note_tags::note_id)))
            .inner_join(tags::table.on(tags::id.eq(note_tags::tag_id)))
            .filter(tags::tag_name.eq_any(tags_filter))
            .filter(archived.eq(archived_flag))
            .filter(deleted.eq(deleted_flag))
            .select(Note::as_select())
            .distinct();

        let ordered_query = match include_order.as_deref() {
            Some("asc") => tag_filtered_query.order(created_at.asc()).into_boxed(),
            Some("desc") => tag_filtered_query.order(created_at.desc()).into_boxed(),
            _ => tag_filtered_query.order(created_at.desc()).into_boxed(), // デフォルトは降順
        };

        return Ok(ordered_query.load::<Note>(conn)?);
    }

    // タグフィルターがない場合
    let base_query = notes
        .filter(archived.eq(archived_flag))
        .filter(deleted.eq(deleted_flag))
        .select(Note::as_select());

    let ordered_query = match include_order.as_deref() {
        Some("asc") => base_query.order(created_at.asc()).into_boxed(),
        Some("desc") => base_query.order(created_at.desc()).into_boxed(),
        _ => base_query.order(created_at.desc()).into_boxed(), // デフォルト: desc
    };

    Ok(ordered_query.load::<Note>(conn)?)
}

pub fn get_note_by_id(conn: &mut SqliteConnection, note_id: &str) -> Result<Option<Note>, Error> {
    let note = notes
        .find(note_id)
        .select(Note::as_select())
        .first(conn)
        .optional()?;

    Ok(note)
}

// ==============================
// ▼ Update
// ==============================
pub fn update_note(
    conn: &mut SqliteConnection,
    note_id: &str,
    updated_title: Option<String>,
    updated_note_type: Option<String>,
    updated_sub_type: Option<String>,
    updated_project_id: Option<String>,
    updated_task_id: Option<String>,
) -> Result<Note, Error> {
    let exist_note = ensure_note_exists(conn, note_id)?;

    let validated_note_type = match updated_note_type {
        Some(ref exist) => parse_note_type(exist)?,
        None => exist_note.note_type,
    };

    let validated_sub_type = match updated_sub_type {
        Some(ref exist) => parse_sub_type(exist)?,
        None => exist_note.sub_type,
    };

    if let Some(ref pid) = updated_project_id {
        ensure_project_exists(conn, pid)?;
    }

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

pub fn archive_note(conn: &mut SqliteConnection, note_id: &str) -> Result<Note, Error> {
    let exist_note = ensure_note_exists(conn, note_id)?;

    if exist_note.archived {
        return Err(Error::QueryBuilderError("Note is already archived".into()));
    }

    diesel::update(notes.find(note_id))
        .set(ArchivedNote { archived: true })
        .returning(Note::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Delete
// ==============================
pub fn soft_delete_note(conn: &mut SqliteConnection, note_id: &str) -> Result<Note, Error> {
    let exist_note = ensure_note_exists(conn, note_id)?;

    if exist_note.deleted {
        return Err(Error::QueryBuilderError("Note is already deleted".into()));
    }

    diesel::update(notes.find(note_id))
        .set(SoftDeletedNote { deleted: true })
        .returning(Note::as_select())
        .get_result(conn)
}

pub fn delete_note(conn: &mut SqliteConnection, note_id: &str) -> Result<(), Error> {
    let _exist_note = ensure_note_exists(conn, note_id)?;
    diesel::delete(notes.find(note_id))
        .returning(Note::as_select())
        .get_result(conn)?;

    Ok(())
}

// ==============================
// ▼ Unarchive / Restore
// ==============================
pub fn unarchive_note(conn: &mut SqliteConnection, note_id: &str) -> Result<Note, Error> {
    let archived_note = ensure_note_exists(conn, note_id)?;

    if !archived_note.archived {
        return Err(Error::QueryBuilderError("Note is not archived".into()));
    }

    diesel::update(notes.find(note_id))
        .set(ArchivedNote { archived: false })
        .returning(Note::as_select())
        .get_result(conn)
}

pub fn restore_note(conn: &mut SqliteConnection, note_id: &str) -> Result<Note, Error> {
    let deleted_note = ensure_note_exists(conn, note_id)?;

    if !deleted_note.deleted {
        return Err(Error::QueryBuilderError("Note is not deleted".into()));
    }

    diesel::update(notes.find(note_id))
        .set(SoftDeletedNote { deleted: false })
        .returning(Note::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Internal Common Utils
// ==============================
fn ensure_note_exists(conn: &mut SqliteConnection, note_id: &str) -> Result<Note, Error> {
    match get_note_by_id(conn, note_id)? {
        Some(note) => Ok(note),
        None => Err(Error::QueryBuilderError("Note not found".into())),
    }
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
