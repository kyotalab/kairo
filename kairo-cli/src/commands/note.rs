use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[command(about = "Create and manage Zettelkasten notes")]
pub enum NoteCommands {
    #[command(
        about = "Create a new Zettelkasten note.",
        long_about = "Creates a new Zettelkasten note with optional metadata such as type (fleeting or permanent), subtype (log, idea, etc.), tags, and associations with projects or tasks."
    )]
    Create {
        #[arg(short = 't', long = "title")]
        arg_title: String,
        #[arg(short = 'n', long = "note-type")]
        arg_note_type: String,
        #[arg(short = 's', long = "sub-type")]
        arg_sub_type: String,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
        #[arg(long = "tid")]
        arg_task_id: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
    },
    #[command(
        about = "List Zettelkasten notes.",
        long_about = "Displays a list of notes. You can filter by archived or deleted status, associated tags, and sort by creation date."
    )]
    List {
        #[arg(long = "archived")]
        arg_archived: Option<bool>,
        #[arg(long = "deleted")]
        arg_deleted: Option<bool>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
        #[arg(long = "order")]
        arg_order: Option<String>,
    },
    #[command(
        about = "Show a specific note by ID.",
        long_about = "Retrieves and displays the full details of a note, including metadata and associations, based on the provided note ID."
    )]
    Get {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Update an existing note.",
        long_about = "Updates the specified note. You can modify the title, type, subtype, tags, and associated project or task."
    )]
    Update {
        #[arg(long = "id")]
        arg_id: String,
        #[arg(short = 't', long = "title")]
        arg_title: Option<String>,
        #[arg(short = 'n', long = "note-type")]
        arg_note_type: Option<String>,
        #[arg(short = 's', long = "sub-type")]
        arg_sub_type: Option<String>,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
        #[arg(long = "tid")]
        arg_task_id: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
    },
    #[command(
        about = "Archive a note.",
        long_about = "Marks a note as archived. Archived notes are not included in the default note list."
    )]
    Archive {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Soft-delete a note.",
        long_about = "Marks a note as deleted. The note remains in the database and can be restored later."
    )]
    Delete {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Permanently delete a note.",
        long_about = "Removes the note from the database permanently. This action cannot be undone."
    )]
    Purge {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Unarchive a note.",
        long_about = "Restores an archived note back to the active state."
    )]
    Unarchive {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Restore a soft-deleted note.",
        long_about = "Recovers a previously soft-deleted note and makes it active again."
    )]
    Restore {
        #[arg(long = "id")]
        arg_id: String,
    },
}
