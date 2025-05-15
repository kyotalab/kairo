use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::project_tags)]
pub struct ProjectTag {
    pub project_id: String,
    pub tag_id: String,
}
