use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[command(about = "Create and manage tasks (can be linked to notes and projects)")]
pub enum TaskCommands {
    #[command(
        about = "Create a new task.",
        long_about = "Creates a new task with title, description, priority (low/medium/high), due date, and optional project association."
    )]
    Create {
        #[arg(short = 't', long = "title")]
        arg_title: String,
        #[arg(short = 'd', long = "description")]
        arg_description: Option<String>,
        #[arg(short = 'p', long = "priority")]
        arg_priority: Option<String>,
        #[arg(long = "due")]
        arg_due_date: Option<String>,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
    },
    #[command(
        about = "List all tasks.",
        long_about = "Displays all tasks with optional filters such as archived/deleted status, tags, priority, and project ID."
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
        #[arg(long = "priority")]
        arg_priority: Option<String>,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
    },
    #[command(
        about = "Get a task by ID.",
        long_about = "Retrieves and displays a task with the specified ID."
    )]
    Get {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Update an existing task.",
        long_about = "Updates the title, description, priority, due date, or associated project of a task."
    )]
    Update {
        #[arg(long = "id")]
        arg_id: String,
        #[arg(short = 't', long = "title")]
        arg_title: Option<String>,
        #[arg(short = 'd', long = "description")]
        arg_description: Option<String>,
        #[arg(short = 'p', long = "priority")]
        arg_priority: Option<String>,
        #[arg(long = "due")]
        arg_due_date: Option<String>,
        #[arg(long = "pid")]
        arg_project_id: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
    },
    #[command(
        about = "Archive a task.",
        long_about = "Marks a task as archived. Archived tasks are excluded from default listings."
    )]
    Archive {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Soft-delete a task.",
        long_about = "Marks a task as deleted, allowing restoration later."
    )]
    Delete {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Permanently delete a task.",
        long_about = "Deletes a task permanently from the database."
    )]
    Purge {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Unarchive a task.",
        long_about = "Restores an archived task to active status."
    )]
    Unarchive {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Restore a deleted task.",
        long_about = "Restores a previously soft-deleted task."
    )]
    Restore {
        #[arg(long = "id")]
        arg_id: String,
    },
}
