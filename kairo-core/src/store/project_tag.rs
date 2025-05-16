use crate::{model::ProjectTag, schema::project_tags};
use diesel::{SqliteConnection, prelude::*, result::Error};

// ==============================
// ▼ Structs / Create
// ==============================
#[derive(Insertable)]
#[diesel(table_name = project_tags)]
pub struct NewProjectTag {
    pub project_id: String,
    pub tag_id: String,
}

// ==============================
// ▼ Create / Insert
// ==============================
pub fn create_project_tag(
    conn: &mut SqliteConnection,
    input_project_id: &str,
    input_tag_id: &str,
) -> Result<ProjectTag, Error> {
    let new_project_tag = NewProjectTag {
        project_id: input_project_id.to_string(),
        tag_id: input_tag_id.to_string(),
    };

    diesel::insert_into(project_tags::table)
        .values(&new_project_tag)
        .returning(ProjectTag::as_select())
        .get_result(conn)
}
