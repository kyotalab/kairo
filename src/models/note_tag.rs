use diesel::prelude::*;
use diesel::sqlite::Sqlite;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::note_tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NoteTag {
    pub note_id: String,
    pub tag_id: String,
}
