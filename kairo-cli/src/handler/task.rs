use crate::commands::task::TaskCommands;
use diesel::SqliteConnection;
use kairo_core::{config::AppConfig, usecase::task::*};

pub fn handle_task_command(command: TaskCommands, conn: &mut SqliteConnection, config: &AppConfig) {
    match command {
        TaskCommands::Create {
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
            arg_tags,
        } => {
            if let Err(e) = handle_create_task(
                config,
                conn,
                arg_title,
                arg_description,
                arg_priority,
                arg_due_date,
                arg_project_id,
                arg_tags,
            ) {
                eprintln!("Failed to create task: {}", e);
            }
        }
        TaskCommands::List {
            arg_archived,
            arg_deleted,
            arg_tags,
            arg_order,
        } => {
            if let Err(e) = handle_list_tasks(conn, arg_archived, arg_deleted, arg_tags, arg_order)
            {
                eprintln!("Failed to list tasks: {}", e);
            }
        }
        TaskCommands::Get { arg_id } => {
            if let Err(e) = handle_get_task(conn, arg_id) {
                eprintln!("Failed to get task: {}", e);
            }
        }
        TaskCommands::Update {
            arg_id,
            arg_title,
            arg_description,
            arg_priority,
            arg_due_date,
            arg_project_id,
        } => {
            if let Err(e) = handle_update_task(
                conn,
                arg_id,
                arg_title,
                arg_description,
                arg_priority,
                arg_due_date,
                arg_project_id,
            ) {
                eprintln!("Failed to update task: {}", e);
            }
        }
        TaskCommands::Archive { arg_id } => {
            if let Err(e) = handle_archive_task(conn, arg_id) {
                eprintln!("Failed to archive task: {}", e);
            }
        }
        TaskCommands::Delete { arg_id } => {
            if let Err(e) = handle_delete_task(conn, arg_id) {
                eprintln!("Failed to delete task: {}", e);
            }
        }
        TaskCommands::Purge { arg_id } => {
            if let Err(e) = handle_purge_task(conn, arg_id) {
                eprintln!("Failed to purge task: {}", e);
            }
        }
        TaskCommands::Unarchive { arg_id } => {
            if let Err(e) = handle_unarchive_task(conn, arg_id) {
                eprintln!("Failed to unarchive task: {}", e);
            }
        }
        TaskCommands::Restore { arg_id } => {
            if let Err(e) = handle_restore_task(conn, arg_id) {
                eprintln!("Failed to restore task: {}", e);
            }
        }
    }
}
