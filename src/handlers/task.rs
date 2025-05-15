use crate::commands::task::TaskCommands;
use crate::config::AppConfig;
use crate::frontmatters::TaskFrontMatter;
use crate::repository::*;
use crate::utils::write_to_markdown;
use diesel::SqliteConnection;

pub fn handle_task_command(command: TaskCommands, conn: &mut SqliteConnection, config: &AppConfig) {
    match command {
        TaskCommands::Create {
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
            arg_tags,
        } => match create_task(
            conn,
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
            arg_tags,
        ) {
            Ok(task) => {
                let dir = &config.paths.tasks_dir;
                println!("{:?}", task);

                let tags = get_tags_by_task_id(conn, &task.id).unwrap();
                let tags_str = tags.into_iter().map(|t| t.tag_name).collect();

                let front_matter = TaskFrontMatter {
                    item: task,
                    tags: tags_str,
                };
                if let Err(e) = write_to_markdown(&front_matter, dir) {
                    eprintln!("Failed to write task: {}", e)
                }
                println!("Run `kairo tui` to open dashboard")
            }
            Err(e) => eprintln!("Failed to create task: {}", e),
        },
        TaskCommands::List {
            arg_archived,
            arg_deleted,
        } => match list_tasks(conn, arg_archived, arg_deleted) {
            Ok(tasks) => {
                for task in tasks {
                    println!("{:?}", task);
                }
            }
            Err(e) => eprintln!("Failed to fetch tasks: {}", e),
        },
        TaskCommands::Get { arg_id } => match get_task_by_id(conn, &arg_id) {
            Ok(Some(task)) => {
                println!("{:?}", task);
            }
            Ok(None) => {
                println!("Task not found");
            }
            Err(e) => {
                println!("Database error: {:?}", e);
            }
        },
        TaskCommands::Update {
            arg_id,
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
        } => match update_task(
            conn,
            &arg_id,
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
        ) {
            Ok(task) => println!("{:?}", task),
            Err(e) => eprintln!("Failed to update task: {}", e),
        },
        TaskCommands::Archive { arg_id } => match archive_task(conn, &arg_id) {
            Ok(task) => println!("{:?}", task),
            Err(e) => eprintln!("Failed to archive task: {}", e),
        },
        TaskCommands::Delete { arg_id } => match soft_delete_task(conn, &arg_id) {
            Ok(task) => println!("{:?}", task),
            Err(e) => eprintln!("Failed to delete task: {}", e),
        },
        TaskCommands::Purge { arg_id } => match delete_task(conn, &arg_id) {
            Ok(task) => println!("{:?}", task),
            Err(e) => eprintln!("Failed to purge task: {}", e),
        },
        TaskCommands::Unarchive { arg_id } => match unarchive_task(conn, &arg_id) {
            Ok(task) => println!("{:?}", task),
            Err(e) => eprintln!("Failed to un-archive task: {}", e),
        },
        TaskCommands::Restore { arg_id } => match restore_task(conn, &arg_id) {
            Ok(task) => println!("{:?}", task),
            Err(e) => eprintln!("Failed to restore task: {}", e),
        },
    }
}
