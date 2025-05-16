use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::task_tags)]
pub struct TaskTag {
    pub task_id: String,
    pub tag_id: String,
}
