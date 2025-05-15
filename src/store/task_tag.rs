use crate::{model::TaskTag, schema::task_tags};
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
