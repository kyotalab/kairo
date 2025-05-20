use crate::{
    model::Project,
    schema::{
        project_tags,
        projects::{self, dsl::*},
        tags,
    },
    store::*,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{SqliteConnection, prelude::*, result::Error};

// ==============================
// ▼ Structs / Create
// ==============================
#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
    pub deleted: bool,
}

// ==============================
// ▼ Structs / Update
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = projects)]
pub struct UpdatedProject {
    pub title: Option<String>,
    pub description: Option<String>,
    pub updated_at: NaiveDateTime,
}

// ==============================
// ▼ Structs / Archive
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = projects)]
pub struct ArchivedProject {
    pub archived: bool,
}

// ==============================
// ▼ Structs / SoftDelete
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = projects)]
pub struct DeletedProject {
    pub deleted: bool,
}

// ==============================
// ▼ Create / Insert
// ==============================
fn generate_project_id(conn: &mut SqliteConnection) -> Result<String, diesel::result::Error> {
    use regex::Regex;

    // タグIDの最大数値部分を取得
    let all_ids: Vec<String> = projects.select(id).load::<String>(conn)?;

    // 正規表現で "p-001" 形式の数字部分を抽出して最大値を見つける
    let re = Regex::new(r"p-(\d{3})").unwrap();
    let max_num = all_ids
        .iter()
        .filter_map(|project_id| {
            re.captures(project_id)
                .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);

    // 次の番号を3桁ゼロ埋めで生成
    let next_id = format!("p-{:03}", max_num + 1);
    Ok(next_id)
}

pub fn create_project(
    conn: &mut SqliteConnection,
    input_title: String,
    input_description: Option<String>,
    input_tag_names: Option<Vec<String>>,
) -> Result<Project, Error> {
    let project_id = generate_project_id(conn)?;

    let new_project = NewProject {
        id: project_id,
        title: input_title,
        description: input_description,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        archived: false,
        deleted: false,
    };

    let project = diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_select())
        .get_result(conn)?;

    // Tag と ProjectTag の保存処理
    if let Some(tag_names) = input_tag_names {
        for name in tag_names {
            // タグ取得または作成
            let tag = match get_tag_by_name(conn, name.clone()) {
                Ok(Some(existing)) => existing,
                Ok(None) => create_tag(conn, name.clone())?,
                Err(e) => return Err(e),
            };

            // project_tag を作成
            create_project_tag(conn, &project.id, &tag.id)?;
        }
    }

    Ok(project)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn list_projects(
    conn: &mut SqliteConnection,
    include_archived: Option<bool>,
    include_deleted: Option<bool>,
    include_tags: Option<Vec<String>>,
    include_order: Option<String>,
) -> Result<Vec<Project>, Error> {
    let archived_flag = include_archived.unwrap_or(false);
    let deleted_flag = include_deleted.unwrap_or(false);

    if archived_flag && deleted_flag {
        return Err(Error::QueryBuilderError(
            "Invalid combination: archived=true AND deleted=true".into(),
        ));
    }

    // タグフィルターがある場合：JOIN込みクエリで返す
    if let Some(tags_filter) = include_tags {
        let tag_filtered_query = projects
            .inner_join(project_tags::table.on(projects::id.eq(project_tags::project_id)))
            .inner_join(tags::table.on(tags::id.eq(project_tags::tag_id)))
            .filter(tags::tag_name.eq_any(tags_filter))
            .filter(archived.eq(archived_flag))
            .filter(deleted.eq(deleted_flag))
            .select(Project::as_select())
            .distinct();

        let ordered_query = match include_order.as_deref() {
            Some("asc") => tag_filtered_query.order(created_at.asc()).into_boxed(),
            Some("desc") => tag_filtered_query.order(created_at.desc()).into_boxed(),
            _ => tag_filtered_query.order(created_at.desc()).into_boxed(), // デフォルトは降順
        };

        return Ok(ordered_query.load::<Project>(conn)?);
    }

    // タグフィルターがない場合
    let base_query = projects
        .filter(archived.eq(archived_flag))
        .filter(deleted.eq(deleted_flag))
        .select(Project::as_select());

    let ordered_query = match include_order.as_deref() {
        Some("asc") => base_query.order(created_at.asc()).into_boxed(),
        Some("desc") => base_query.order(created_at.desc()).into_boxed(),
        _ => base_query.order(created_at.desc()).into_boxed(), // デフォルト: desc
    };

    Ok(ordered_query.load::<Project>(conn)?)
}

pub fn get_project_by_id(
    conn: &mut SqliteConnection,
    project_id: &str,
) -> Result<Option<Project>, Error> {
    let project = projects
        .find(project_id)
        .select(Project::as_select())
        .first(conn)
        .optional()?;

    Ok(project)
}

// ==============================
// ▼ Update
// ==============================
pub fn update_project(
    conn: &mut SqliteConnection,
    project_id: &str,
    updated_title: Option<String>,
    updated_description: Option<String>,
    updated_tags: Option<Vec<String>>,
) -> Result<Project, Error> {
    let _exist_project = ensure_project_exists(conn, project_id)?;

    let updated_project = UpdatedProject {
        title: updated_title,
        description: updated_description,
        updated_at: Utc::now().naive_utc(),
    };

    match updated_tags {
        None => {
            // 何もしない
        }
        Some(ref tags) => {
            delete_project_tag_by_project_id(conn, project_id)?;

            if !tags.is_empty() {
                for tag_name in tags {
                    // タグ取得または作成
                    let tag = match get_tag_by_name(conn, tag_name.clone()) {
                        Ok(Some(existing)) => existing,
                        Ok(None) => create_tag(conn, tag_name.clone())?,
                        Err(e) => return Err(e),
                    };

                    // project_tag を作成
                    create_project_tag(conn, &project_id, &tag.id)?;
                }
            }
        }
    }

    diesel::update(projects.find(project_id))
        .set(updated_project)
        .returning(Project::as_select())
        .get_result(conn)
}

pub fn archive_project(conn: &mut SqliteConnection, project_id: &str) -> Result<Project, Error> {
    let exist_project = ensure_project_exists(conn, project_id)?;

    if exist_project.archived {
        return Err(Error::QueryBuilderError(
            "Project is already archived".into(),
        ));
    }

    diesel::update(projects.find(project_id))
        .set(ArchivedProject { archived: true })
        .returning(Project::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Delete
// ==============================
pub fn soft_delete_project(
    conn: &mut SqliteConnection,
    project_id: &str,
) -> Result<Project, Error> {
    let exist_project = ensure_project_exists(conn, project_id)?;

    if exist_project.deleted {
        return Err(Error::QueryBuilderError(
            "Project is already deleted".into(),
        ));
    }

    diesel::update(projects.find(project_id))
        .set(DeletedProject { deleted: true })
        .returning(Project::as_select())
        .get_result(conn)
}

pub fn delete_project(conn: &mut SqliteConnection, project_id: &str) -> Result<(), Error> {
    let _exist_project = ensure_project_exists(conn, project_id)?;
    diesel::delete(projects.find(project_id))
        .returning(Project::as_select())
        .get_result(conn)?;

    Ok(())
}

// ==============================
// ▼ Unarchive / Restore
// ==============================
pub fn unarchive_project(conn: &mut SqliteConnection, project_id: &str) -> Result<Project, Error> {
    let archived_project = ensure_project_exists(conn, project_id)?;

    if !archived_project.archived {
        return Err(Error::QueryBuilderError("Project is not archived".into()));
    }

    diesel::update(projects.find(project_id))
        .set(ArchivedProject { archived: false })
        .returning(Project::as_select())
        .get_result(conn)
}

pub fn restore_project(conn: &mut SqliteConnection, project_id: &str) -> Result<Project, Error> {
    let deleted_project = ensure_project_exists(conn, project_id)?;

    if !deleted_project.deleted {
        return Err(Error::QueryBuilderError("Project is not deleted".into()));
    }

    diesel::update(projects.find(project_id))
        .set(DeletedProject { deleted: false })
        .returning(Project::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Internal Common Utils
// ==============================
pub fn ensure_project_exists(
    conn: &mut SqliteConnection,
    project_id: &str,
) -> Result<Project, Error> {
    match get_project_by_id(conn, project_id)? {
        Some(project) => Ok(project),
        None => Err(Error::QueryBuilderError("Project not found".into())),
    }
}
