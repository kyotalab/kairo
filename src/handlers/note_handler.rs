use crate::models::cli::note_cli::NoteCommands;
use crate::repository::*;

pub fn handle_note_command(command: NoteCommands) {
    let conn = &mut establish_connection();

    match command {
        NoteCommands::Create {
            arg_title,
            arg_note_type,
            arg_sub_type,
            arg_project_id,
            arg_task_id,
            arg_tags,
        } => {
            match create_note(
                conn,
                arg_title,
                &arg_note_type,
                &arg_sub_type,
                arg_project_id,
                arg_task_id,
                arg_tags,
            ) {
                Ok(note) => println!("{:?}", note),
                Err(e) => eprintln!("Failed to create note: {}", e),
            };
        }
        NoteCommands::List {
            arg_archived,
            arg_deleted,
        } => match list_notes(conn, arg_archived, arg_deleted) {
            Ok(notes) => {
                for note in notes {
                    println!("{:?}", note);
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch notes: {}", e);
            }
        },
        NoteCommands::Get { arg_id } => match get_note_by_id(conn, &arg_id) {
            Ok(Some(note)) => {
                println!("{:?}", note);
            }
            Ok(None) => {
                println!("Note not found");
            }
            Err(e) => {
                println!("Database error: {:?}", e);
            }
        },
        NoteCommands::Update {
            arg_id,
            arg_title,
            arg_note_type,
            arg_sub_type,
            arg_project_id,
            arg_task_id,
        } => {
            match update_note(
                conn,
                &arg_id,
                arg_title,
                arg_note_type,
                arg_sub_type,
                arg_project_id,
                arg_task_id,
            ) {
                Ok(note) => println!("{:?}", note),
                Err(e) => eprintln!("Failed to update note: {}", e),
            }
        }
        NoteCommands::Archive { arg_id } => match archive_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to archive note: {}", e),
        },
        NoteCommands::Delete { arg_id } => match soft_delete_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to delete note: {}", e),
        },
        NoteCommands::Purge { arg_id } => match delete_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to purge note: {}", e),
        },
        NoteCommands::Unarchive { arg_id } => match unarchive_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to un-archive note: {}", e),
        },
        NoteCommands::Restore { arg_id } => match restore_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to restore note: {}", e),
        },
    }
}
