use crate::{
    model::TaskTag,
    schema::task_tags::{self, task_id},
};
use diesel::{SqliteConnection, prelude::*, result::Error};

// ==============================
// ▼ Structs / Create
// ==============================
#[derive(Insertable)]
#[diesel(table_name = task_tags)]
pub struct NewTaskTag {
    pub task_id: String,
    pub tag_id: String,
}

// ==============================
// ▼ Create / Insert
// ==============================
pub fn create_task_tag(
    conn: &mut SqliteConnection,
    input_task_id: &str,
    input_tag_id: &str,
) -> Result<TaskTag, Error> {
    let new_task_tag = NewTaskTag {
        task_id: input_task_id.to_string(),
        tag_id: input_tag_id.to_string(),
    };

    diesel::insert_into(task_tags::table)
        .values(&new_task_tag)
        .returning(TaskTag::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn get_task_tags_by_task_id(
    conn: &mut SqliteConnection,
    input_task_id: &str,
) -> Result<Option<Vec<TaskTag>>, Error> {
    let task_tag = task_tags::table
        .filter(task_tags::task_id.eq(input_task_id))
        .select(TaskTag::as_select())
        .load::<TaskTag>(conn)
        .optional()?;

    Ok(task_tag)
}

// ==============================
// ▼ Delete
// ==============================
pub fn delete_task_tag_by_task_id(
    conn: &mut SqliteConnection,
    input_task_id: &str,
) -> Result<(), Error> {
    let _exist_task_tags = ensure_task_tag_exists(conn, input_task_id)?;
    diesel::delete(task_tags::table.filter(task_id.eq(input_task_id))).execute(conn)?;

    Ok(())
}

// ==============================
// ▼ Internal Common Utils
// ==============================
fn ensure_task_tag_exists(
    conn: &mut SqliteConnection,
    input_task_id: &str,
) -> Result<Vec<TaskTag>, Error> {
    match get_task_tags_by_task_id(conn, input_task_id)? {
        Some(task_tag) => Ok(task_tag),
        None => Err(Error::QueryBuilderError("Task_tag not found".into())),
    }
}
