use crate::commands::task::TaskCommands;
use crate::repository::*;

pub fn handle_task_command(command: TaskCommands) {
    let conn = &mut establish_connection();

    match command {
        TaskCommands::Create {
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
        } => match create_task(
            conn,
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
        ) {
            Ok(task) => println!("{:?}", task),
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
