use crate::models::Project;
use crate::schema::projects;
use crate::schema::projects::dsl::*;
use chrono::{NaiveDateTime, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::result::Error;

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

    // 正規表現で "t-001" 形式の数字部分を抽出して最大値を見つける
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

    diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn list_projects(
    conn: &mut SqliteConnection,
    include_archived: Option<bool>,
    include_deleted: Option<bool>,
) -> Result<Vec<Project>, Error> {
    let mut query = projects.into_boxed();

    match (
        include_archived.unwrap_or(false),
        include_deleted.unwrap_or(false),
    ) {
        (false, false) => {
            // 通常表示（Activeなノートのみ）
            query = query.filter(archived.eq(false)).filter(deleted.eq(false));
        }
        (true, false) => {
            // アーカイブ済のみ
            query = query.filter(archived.eq(true)).filter(deleted.eq(false));
        }
        (false, true) => {
            // 削除済のみ
            query = query.filter(archived.eq(false)).filter(deleted.eq(true));
        }
        (true, true) => {
            // 禁止：両方trueは「ありえない状態」
            return Err(Error::QueryBuilderError(
                "Invalid combination: archived=true AND deleted=true".into(),
            ));
        }
    }

    Ok(query
        .select(Project::as_select())
        .order(created_at.desc())
        .load(conn)?)
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
) -> Result<Project, Error> {
    let _exist_project = ensure_project_exists(conn, project_id)?;

    let updated_project = UpdatedProject {
        title: updated_title,
        description: updated_description,
        updated_at: Utc::now().naive_utc(),
    };

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
// ▼ Internal Common Utils
// ==============================
fn ensure_project_exists(conn: &mut SqliteConnection, project_id: &str) -> Result<Project, Error> {
    match get_project_by_id(conn, project_id)? {
        Some(project) => Ok(project),
        None => Err(Error::QueryBuilderError("Project not found".into())),
    }
}
