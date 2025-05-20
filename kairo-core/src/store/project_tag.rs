use crate::{
    model::ProjectTag,
    schema::project_tags::{self, project_id},
};
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

// ==============================
// ▼ Read / Select
// ==============================
pub fn get_project_tags_by_project_id(
    conn: &mut SqliteConnection,
    input_project_id: &str,
) -> Result<Option<Vec<ProjectTag>>, Error> {
    let project_tag = project_tags::table
        .filter(project_tags::project_id.eq(input_project_id))
        .select(ProjectTag::as_select())
        .load::<ProjectTag>(conn)
        .optional()?;

    Ok(project_tag)
}

// ==============================
// ▼ Delete
// ==============================
pub fn delete_project_tag_by_project_id(
    conn: &mut SqliteConnection,
    input_project_id: &str,
) -> Result<(), Error> {
    let _exist_project_tags = ensure_project_tag_exists(conn, input_project_id)?;
    diesel::delete(project_tags::table.filter(project_id.eq(input_project_id))).execute(conn)?;

    Ok(())
}

// ==============================
// ▼ Internal Common Utils
// ==============================
fn ensure_project_tag_exists(
    conn: &mut SqliteConnection,
    input_project_id: &str,
) -> Result<Vec<ProjectTag>, Error> {
    match get_project_tags_by_project_id(conn, input_project_id)? {
        Some(project_tag) => Ok(project_tag),
        None => Err(Error::QueryBuilderError("Project_tag not found".into())),
    }
}
