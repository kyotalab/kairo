use crate::models::{Task, TaskPriority};
use crate::repository::*;
use crate::schema::tasks;
use crate::schema::tasks::dsl::*;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::result::Error;

// ==============================
// ▼ Structs / Create
// ==============================
#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub archived: bool,
    pub deleted: bool,
    pub project_id: Option<String>,
}

// ==============================
// ▼ Structs / Update
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct UpdatedTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    pub project_id: Option<String>,
}

// ==============================
// ▼ Structs / Archive
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct ArchivedTask {
    pub archived: bool,
}

// ==============================
// ▼ Structs / SoftDelete
// ==============================
#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct DeletedTask {
    pub deleted: bool,
}

// ==============================
// ▼ Create / Insert
// ==============================
fn generate_task_id(conn: &mut SqliteConnection) -> Result<String, diesel::result::Error> {
    use regex::Regex;

    // タグIDの最大数値部分を取得
    let all_ids: Vec<String> = tasks.select(id).load::<String>(conn)?;

    // 正規表現で "task-001" 形式の数字部分を抽出して最大値を見つける
    let re = Regex::new(r"task-(\d{3})").unwrap();
    let max_num = all_ids
        .iter()
        .filter_map(|task_id| {
            re.captures(task_id)
                .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);

    // 次の番号を3桁ゼロ埋めで生成
    let next_id = format!("task-{:03}", max_num + 1);
    Ok(next_id)
}

pub fn create_task(
    conn: &mut SqliteConnection,
    input_title: String,
    input_description: Option<String>,
    input_task_priority: Option<String>,
    input_due_date: Option<String>,
    input_project_id: Option<String>,
) -> Result<Task, Error> {
    let task_id = generate_task_id(conn)?;
    let validated_task_priority = parse_task_priority(input_task_priority)?;
    let parsed_due_date = parse_due_date(input_due_date)
        .map_err(|e| Error::QueryBuilderError(format!("Invalid due_date: {}", e).into()))?;

    let new_task = NewTask {
        id: task_id,
        title: input_title,
        description: input_description,
        priority: validated_task_priority,
        due_date: parsed_due_date,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        archived: false,
        deleted: false,
        project_id: input_project_id,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Read / Select
// ==============================
pub fn list_tasks(
    conn: &mut SqliteConnection,
    include_archived: Option<bool>,
    include_deleted: Option<bool>,
) -> Result<Vec<Task>, Error> {
    let mut query = tasks.into_boxed();

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
        .select(Task::as_select())
        .order(created_at.desc())
        .load(conn)?)
}

pub fn get_task_by_id(conn: &mut SqliteConnection, task_id: &str) -> Result<Option<Task>, Error> {
    let task = tasks
        .find(task_id)
        .select(Task::as_select())
        .first(conn)
        .optional()?;

    Ok(task)
}

// ==============================
// ▼ Update
// ==============================
pub fn update_task(
    conn: &mut SqliteConnection,
    task_id: &str,
    updated_title: Option<String>,
    updated_description: Option<String>,
    updated_task_priority: Option<String>,
    updated_due_date: Option<String>,
    updated_project_id: Option<String>,
) -> Result<Task, Error> {
    let _exist_task = ensure_task_exists(conn, task_id)?;
    let validated_task_priority = parse_task_priority(updated_task_priority)?;
    let parsed_due_date = parse_due_date(updated_due_date)
        .map_err(|e| Error::QueryBuilderError(format!("Invalid due_date: {}", e).into()))?;

    if let Some(ref pid) = updated_project_id {
        ensure_project_exists(conn, pid)?;
    }

    let updated_task = UpdatedTask {
        title: updated_title,
        description: updated_description,
        priority: validated_task_priority,
        due_date: parsed_due_date,
        updated_at: Utc::now().naive_utc(),
        project_id: updated_project_id,
    };

    diesel::update(tasks.find(task_id))
        .set(updated_task)
        .returning(Task::as_select())
        .get_result(conn)
}

pub fn archive_task(conn: &mut SqliteConnection, task_id: &str) -> Result<Task, Error> {
    let exist_task = ensure_task_exists(conn, task_id)?;

    if exist_task.archived {
        return Err(Error::QueryBuilderError("Task is already archived".into()));
    }

    diesel::update(tasks.find(task_id))
        .set(ArchivedTask { archived: true })
        .returning(Task::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Delete
// ==============================
pub fn soft_delete_task(conn: &mut SqliteConnection, task_id: &str) -> Result<Task, Error> {
    let exist_task = ensure_task_exists(conn, task_id)?;

    if exist_task.deleted {
        return Err(Error::QueryBuilderError("Task is already deleted".into()));
    }

    diesel::update(tasks.find(task_id))
        .set(DeletedTask { deleted: true })
        .returning(Task::as_select())
        .get_result(conn)
}

pub fn delete_task(conn: &mut SqliteConnection, task_id: &str) -> Result<(), Error> {
    let _exist_task = ensure_task_exists(conn, task_id)?;
    diesel::delete(tasks.find(task_id))
        .returning(Task::as_select())
        .get_result(conn)?;

    Ok(())
}

// ==============================
// ▼ Unarchive / Restore
// ==============================
pub fn unarchive_task(conn: &mut SqliteConnection, task_id: &str) -> Result<Task, Error> {
    let archived_task = ensure_task_exists(conn, task_id)?;

    if !archived_task.archived {
        return Err(Error::QueryBuilderError("Task is not archived".into()));
    }

    diesel::update(tasks.find(task_id))
        .set(ArchivedTask { archived: false })
        .returning(Task::as_select())
        .get_result(conn)
}

pub fn restore_task(conn: &mut SqliteConnection, task_id: &str) -> Result<Task, Error> {
    let deleted_task = ensure_task_exists(conn, task_id)?;

    if !deleted_task.deleted {
        return Err(Error::QueryBuilderError("Task is not deleted".into()));
    }

    diesel::update(tasks.find(task_id))
        .set(DeletedTask { deleted: false })
        .returning(Task::as_select())
        .get_result(conn)
}

// ==============================
// ▼ Internal Common Utils
// ==============================
fn ensure_task_exists(conn: &mut SqliteConnection, task_id: &str) -> Result<Task, Error> {
    match get_task_by_id(conn, task_id)? {
        Some(task) => Ok(task),
        None => Err(Error::QueryBuilderError("Task not found".into())),
    }
}

fn parse_task_priority(input: Option<String>) -> Result<Option<TaskPriority>, Error> {
    match input.as_deref().unwrap_or("medium") {
        "low" => Ok(Some(TaskPriority::Low)),
        "medium" => Ok(Some(TaskPriority::Medium)),
        "high" => Ok(Some(TaskPriority::High)),
        "" => Ok(Some(TaskPriority::Medium)),
        other => Err(Error::QueryBuilderError(
            format!("Invalid task_priority: {}", other).into(),
        )),
    }
}

fn parse_due_date(due: Option<String>) -> Result<Option<NaiveDateTime>, chrono::ParseError> {
    match due {
        Some(due_str) => {
            let date = NaiveDate::parse_from_str(&due_str, "%Y-%m-%d")?;
            Ok(date.and_hms_opt(0, 0, 0))
        }
        None => Ok(None),
    }
}
