use crate::{
    model::NoteTag,
    schema::note_tags::{self, note_id},
};
use diesel::{SqliteConnection, prelude::*, result::Error};

// ==============================
// ▼ Structs / Create
// ==============================
#[derive(Insertable)]
#[diesel(table_name = note_tags)]
pub struct NewNoteTag {
    pub note_id: String,
    pub tag_id: String,
}

// ==============================
// ▼ Create / Insert
// ==============================
pub fn create_note_tag(
    conn: &mut SqliteConnection,
    input_note_id: &str,
    input_tag_id: &str,
) -> Result<NoteTag, Error> {
    let new_note_tag = NewNoteTag {
        note_id: input_note_id.to_string(),
        tag_id: input_tag_id.to_string(),
    };

    diesel::insert_into(note_tags::table)
        .values(&new_note_tag)
        .returning(NoteTag::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn get_note_tags_by_note_id(
    conn: &mut SqliteConnection,
    input_note_id: &str,
) -> Result<Option<Vec<NoteTag>>, Error> {
    let note_tag = note_tags::table
        .filter(note_tags::note_id.eq(input_note_id))
        .select(NoteTag::as_select())
        .load::<NoteTag>(conn)
        .optional()?;

    Ok(note_tag)
}

// ==============================
// ▼ Delete
// ==============================
pub fn delete_note_tag_by_note_id(
    conn: &mut SqliteConnection,
    input_note_id: &str,
) -> Result<(), Error> {
    let _exist_note_tags = ensure_note_tag_exists(conn, input_note_id)?;
    diesel::delete(note_tags::table.filter(note_id.eq(input_note_id))).execute(conn)?;

    Ok(())
}

// ==============================
// ▼ Internal Common Utils
// ==============================
fn ensure_note_tag_exists(
    conn: &mut SqliteConnection,
    input_note_id: &str,
) -> Result<Vec<NoteTag>, Error> {
    match get_note_tags_by_note_id(conn, input_note_id)? {
        Some(note_tag) => Ok(note_tag),
        None => Err(Error::QueryBuilderError("Note_tag not found".into())),
    }
}
