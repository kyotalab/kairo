use crate::{
    config::AppConfig,
    markdown::{TaskContent, TaskFrontMatter},
    store::*,
    util::{parse_markdown, print_tasks_as_table, write_to_markdown},
};
use anyhow::Ok;
use diesel::SqliteConnection;

pub fn handle_create_task(
    config: &AppConfig,
    conn: &mut SqliteConnection,
    title: String,
    description: Option<String>,
    priority: Option<String>,
    due_date: Option<String>,
    project_id: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<(), anyhow::Error> {
    let task = create_task(
        conn,
        title,
        description,
        priority,
        due_date,
        project_id,
        tags,
    )?;

    let dir = &config.paths.tasks_dir;
    println!("{:?}", task);
    let tags = get_tags_by_task_id(conn, &task.id).unwrap();
    let tags_str = tags.into_iter().map(|t| t.tag_name).collect();

    let front_matter = TaskFrontMatter {
        item: task,
        tags: tags_str,
    };

    let task_content = TaskContent {
        front_matter,
        body: None,
    };

    if let Err(e) = write_to_markdown(&task_content, dir) {
        eprintln!("Failed to write task: {}", e)
    }
    println!("Run `kairo tui` to open dashboard");
    Ok(())
}

pub fn handle_list_tasks(
    conn: &mut SqliteConnection,
    include_archived: Option<bool>,
    include_deleted: Option<bool>,
    include_tags: Option<Vec<String>>,
    include_order: Option<String>,
    include_priority: Option<String>,
    include_project_id: Option<String>,
) -> Result<(), anyhow::Error> {
    let tasks = list_tasks(
        conn,
        include_archived,
        include_deleted,
        include_tags,
        include_order,
        include_priority,
        include_project_id,
    )?;
    // for task in tasks {
    //     println!("{:?}", task);
    // }
    print_tasks_as_table(&tasks);
    Ok(())
}

pub fn handle_get_task(conn: &mut SqliteConnection, task_id: String) -> Result<(), anyhow::Error> {
    let task = get_task_by_id(conn, &task_id)?;
    match task {
        Some(exist) => {
            println!("{exist}");
        }
        None => {
            println!("task not found");
        }
    }
    Ok(())
}

pub fn handle_update_task(
    config: &AppConfig,
    conn: &mut SqliteConnection,
    task_id: String,
    title: Option<String>,
    description: Option<String>,
    priority: Option<String>,
    due_date: Option<String>,
    project_id: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<(), anyhow::Error> {
    let updated_task = update_task(
        conn,
        &task_id,
        title,
        description,
        priority,
        due_date,
        project_id,
        tags,
    )?;

    let dir = &config.paths.tasks_dir;
    println!("{:?}", &updated_task);
    let tags = get_tags_by_task_id(conn, &updated_task.id).unwrap();
    let tags_str: Vec<_> = tags.into_iter().map(|t| t.tag_name).collect();

    let contents = parse_markdown(&updated_task, dir)?;
    let body = Some(contents.1);

    let task_front_matter = TaskFrontMatter {
        item: updated_task.clone(),
        tags: tags_str,
    };

    let task_content = TaskContent {
        front_matter: task_front_matter,
        body,
    };

    if let Err(e) = write_to_markdown(&task_content, dir) {
        eprintln!("Failed to write note: {}", e)
    }

    // println!("Updated note: {:?}", updated_task.id);
    // println!("Run `kairo tui` to open dashboard");
    Ok(())
}

pub fn handle_archive_task(
    conn: &mut SqliteConnection,
    task_id: String,
) -> Result<(), anyhow::Error> {
    let task = archive_task(conn, &task_id)?;

    println!("Archived task: {:?}", task.id);
    Ok(())
}

pub fn handle_delete_task(
    conn: &mut SqliteConnection,
    task_id: String,
) -> Result<(), anyhow::Error> {
    let task = soft_delete_task(conn, &task_id)?;

    println!("Deleted task: {:?}", task.id);
    Ok(())
}

pub fn handle_purge_task(
    conn: &mut SqliteConnection,
    task_id: String,
) -> Result<(), anyhow::Error> {
    let _task = delete_task(conn, &task_id)?;

    println!("Purged task: {:?}", task_id);
    Ok(())
}

pub fn handle_unarchive_task(
    conn: &mut SqliteConnection,
    task_id: String,
) -> Result<(), anyhow::Error> {
    let task = unarchive_task(conn, &task_id)?;

    println!("Unarchived task: {:?}", task.id);
    Ok(())
}

pub fn handle_restore_task(
    conn: &mut SqliteConnection,
    task_id: String,
) -> Result<(), anyhow::Error> {
    let task = restore_task(conn, &task_id)?;

    println!("Restored task: {:?}", task.id);
    Ok(())
}
