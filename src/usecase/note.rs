use crate::config::AppConfig;
use crate::markdown::NoteFrontMatter;
use crate::store::note::*;
use crate::store::tag::get_tags_by_note_id;
use crate::util::write_to_markdown;
use anyhow::Ok;
use diesel::SqliteConnection;

pub fn handle_create_note(
    config: &AppConfig,
    conn: &mut SqliteConnection,
    title: String,
    note_type: &str,
    sub_type: &str,
    project_id: Option<String>,
    task_id: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<(), anyhow::Error> {
    let note = create_note(
        conn, title, &note_type, &sub_type, project_id, task_id, tags,
    )?;

    let dir = &config.paths.notes_dir;
    println!("{:?}", note);
    let tags = get_tags_by_note_id(conn, &note.id).unwrap();
    let tags_str = tags.into_iter().map(|t| t.tag_name).collect();

    let front_matter = NoteFrontMatter {
        item: note,
        tags: tags_str,
    };
    if let Err(e) = write_to_markdown(&front_matter, dir) {
        eprintln!("Failed to write note: {}", e)
    }
    println!("Run `kairo tui` to open dashboard");
    Ok(())
}

pub fn handle_list_notes(
    conn: &mut SqliteConnection,
    include_archived: Option<bool>,
    include_deleted: Option<bool>,
) -> Result<(), anyhow::Error> {
    let notes = list_notes(conn, include_archived, include_deleted)?;
    for note in notes {
        println!("{:?}", note);
    }
    Ok(())
}

pub fn handle_get_note(conn: &mut SqliteConnection, note_id: String) -> Result<(), anyhow::Error> {
    let note = get_note_by_id(conn, &note_id)?;
    match note {
        Some(exist) => {
            println!("{:?}", exist);
        }
        None => {
            println!("Note not found");
        }
    }
    Ok(())
}

pub fn handle_update_note(
    conn: &mut SqliteConnection,
    note_id: String,
    title: Option<String>,
    note_type: Option<String>,
    sub_type: Option<String>,
    project_id: Option<String>,
    task_id: Option<String>,
) -> Result<(), anyhow::Error> {
    let note = update_note(
        conn, &note_id, title, note_type, sub_type, project_id, task_id,
    )?;

    println!("Updated note: {:?}", note.id);
    Ok(())
}

pub fn handle_archive_note(
    conn: &mut SqliteConnection,
    note_id: String,
) -> Result<(), anyhow::Error> {
    let note = archive_note(conn, &note_id)?;

    println!("Archived note: {:?}", note.id);
    Ok(())
}

pub fn handle_delete_note(
    conn: &mut SqliteConnection,
    note_id: String,
) -> Result<(), anyhow::Error> {
    let note = soft_delete_note(conn, &note_id)?;

    println!("Deleted note: {:?}", note.id);
    Ok(())
}

pub fn handle_purge_note(
    conn: &mut SqliteConnection,
    note_id: String,
) -> Result<(), anyhow::Error> {
    let _note = delete_note(conn, &note_id)?;

    println!("Purged note: {:?}", note_id);
    Ok(())
}

pub fn handle_unarchive_note(
    conn: &mut SqliteConnection,
    note_id: String,
) -> Result<(), anyhow::Error> {
    let note = unarchive_note(conn, &note_id)?;

    println!("Unarchived note: {:?}", note.id);
    Ok(())
}

pub fn handle_restore_note(
    conn: &mut SqliteConnection,
    note_id: String,
) -> Result<(), anyhow::Error> {
    let note = restore_note(conn, &note_id)?;

    println!("Restored note: {:?}", note.id);
    Ok(())
}
