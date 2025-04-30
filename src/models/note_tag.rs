use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::note_tags)]
pub struct NoteTag {
    pub note_id: String,
    pub tag_id: String,
}
