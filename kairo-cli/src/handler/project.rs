use crate::commands::project::ProjectCommands;
use diesel::SqliteConnection;
use kairo_core::{config::AppConfig, usecase::project::*};

pub fn handle_project_command(
    command: ProjectCommands,
    conn: &mut SqliteConnection,
    config: &AppConfig,
) {
    match command {
        ProjectCommands::Create {
            arg_title,
            arg_description,
            arg_tags,
        } => {
            if let Err(e) =
                handle_create_project(config, conn, arg_title, arg_description, arg_tags)
            {
                eprintln!("Failed to create project: {}", e);
            }
        }
        ProjectCommands::List {
            arg_archived,
            arg_deleted,
            arg_tags,
            arg_order,
        } => {
            if let Err(e) =
                handle_list_projects(conn, arg_archived, arg_deleted, arg_tags, arg_order)
            {
                eprintln!("Failed to list projects: {}", e);
            }
        }
        ProjectCommands::Get { arg_id } => {
            if let Err(e) = handle_get_project(conn, arg_id) {
                eprintln!("Failed to get project: {}", e);
            }
        }
        ProjectCommands::Update {
            arg_id,
            arg_title,
            arg_description,
            arg_tags,
        } => {
            if let Err(e) =
                handle_update_project(config, conn, arg_id, arg_title, arg_description, arg_tags)
            {
                eprintln!("Failed to update project: {}", e);
            }
        }
        ProjectCommands::Archive { arg_id } => {
            if let Err(e) = handle_archive_project(conn, arg_id) {
                eprintln!("Failed to archive project: {}", e);
            }
        }
        ProjectCommands::Delete { arg_id } => {
            if let Err(e) = handle_delete_project(conn, arg_id) {
                eprintln!("Failed to delete project: {}", e);
            }
        }
        ProjectCommands::Purge { arg_id } => {
            if let Err(e) = handle_purge_project(conn, arg_id) {
                eprintln!("Failed to purge project: {}", e);
            }
        }
        ProjectCommands::Unarchive { arg_id } => {
            if let Err(e) = handle_archive_project(conn, arg_id) {
                eprintln!("Failed to unarchive project: {}", e);
            }
        }
        ProjectCommands::Restore { arg_id } => {
            if let Err(e) = handle_restore_project(conn, arg_id) {
                eprintln!("Failed to restore project: {}", e);
            }
        }
    }
}
