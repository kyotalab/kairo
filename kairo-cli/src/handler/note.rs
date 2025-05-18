use crate::commands::note::NoteCommands;
use diesel::SqliteConnection;
use kairo_core::{config::AppConfig, usecase::note::*};

pub fn handle_note_command(command: NoteCommands, conn: &mut SqliteConnection, config: &AppConfig) {
    match command {
        NoteCommands::Create {
            arg_title,
            arg_note_type,
            arg_sub_type,
            arg_project_id,
            arg_task_id,
            arg_tags,
        } => {
            if let Err(e) = handle_create_note(
                config,
                conn,
                arg_title,
                &arg_note_type,
                &arg_sub_type,
                arg_project_id,
                arg_task_id,
                arg_tags,
            ) {
                eprintln!("Failed to create note: {}", e);
            }
        }
        NoteCommands::List {
            arg_archived,
            arg_deleted,
            arg_tags, // TODO note list --tagで指定のタグを含むノートを表示できるようにする。
            arg_order,
        } => {
            if let Err(e) = handle_list_notes(conn, arg_archived, arg_deleted, arg_tags, arg_order)
            {
                eprintln!("Failed to list notes: {}", e);
            }
        }
        NoteCommands::Get { arg_id } => {
            if let Err(e) = handle_get_note(conn, arg_id) {
                eprintln!("Failed to get note: {}", e);
            }
        }
        NoteCommands::Update {
            arg_id,
            arg_title,
            arg_note_type,
            arg_sub_type,
            arg_project_id,
            arg_task_id,
            arg_tags,
        } => {
            if let Err(e) = handle_update_note(
                config,
                conn,
                arg_id,
                arg_title,
                arg_note_type,
                arg_sub_type,
                arg_project_id,
                arg_task_id,
                arg_tags,
            ) {
                eprintln!("Failed to update note: {}", e);
            }
        }
        NoteCommands::Archive { arg_id } => {
            if let Err(e) = handle_archive_note(conn, arg_id) {
                eprintln!("Failed to archive note: {}", e);
            }
        }
        NoteCommands::Delete { arg_id } => {
            if let Err(e) = handle_delete_note(conn, arg_id) {
                eprintln!("Failed to delete note: {}", e);
            }
        }
        NoteCommands::Purge { arg_id } => {
            if let Err(e) = handle_purge_note(conn, arg_id) {
                eprintln!("Failed to purge note: {}", e);
            }
        }
        NoteCommands::Unarchive { arg_id } => {
            if let Err(e) = handle_unarchive_note(conn, arg_id) {
                eprintln!("Failed to unarchive note: {}", e);
            }
        }
        NoteCommands::Restore { arg_id } => {
            if let Err(e) = handle_restore_note(conn, arg_id) {
                eprintln!("Failed to restore note: {}", e);
            }
        }
    }
}
