use crate::{model::NoteTag, schema::note_tags};
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
