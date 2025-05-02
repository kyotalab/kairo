use clap::Parser;
use kairo::models::{Cli, Commands};
use kairo::repository::*;

fn main() {
    let conn = &mut establish_connection();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            arg_title,
            arg_note_type,
            arg_sub_type,
            arg_project_id,
            arg_task_id,
        } => {
            match create_note(
                conn,
                arg_title,
                &arg_note_type,
                &arg_sub_type,
                arg_project_id,
                arg_task_id,
            ) {
                Ok(note) => println!("{:?}", note),
                Err(e) => eprintln!("Failed to create note: {}", e),
            };
        }
        Commands::List {
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
        Commands::Get { arg_id } => match get_note_by_id(conn, &arg_id) {
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
        Commands::Update {
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
        Commands::Archive { arg_id } => match archive_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to archive note: {}", e),
        },
        Commands::Delete { arg_id } => match soft_delete_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to delete note: {}", e),
        },
        Commands::Purge { arg_id } => match delete_note(conn, &arg_id) {
            Ok(note) => println!("{:?}", note),
            Err(e) => eprintln!("Failed to purge note: {}", e),
        },
    }
}
