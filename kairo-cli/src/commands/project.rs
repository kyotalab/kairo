use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[command(about = "Create and manage projects (used to group notes and tasks)")]
pub enum ProjectCommands {
    #[command(
        about = "Create a new project.",
        long_about = "Creates a new project with a title and optional description. Projects can be linked to notes and tasks."
    )]
    Create {
        #[arg(short = 't', long = "title")]
        arg_title: String,
        #[arg(short = 'd', long = "description")]
        arg_description: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
    },
    #[command(
        about = "List all projects.",
        long_about = "Displays all projects with optional filters for archived and deleted status."
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
        about = "Get a project by ID.",
        long_about = "Retrieves and displays a project with the specified ID, including metadata and status."
    )]
    Get {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Update an existing project.",
        long_about = "Updates the title and/or description of the specified project."
    )]
    Update {
        #[arg(long = "id")]
        arg_id: String,
        #[arg(short = 't', long = "title")]
        arg_title: Option<String>,
        #[arg(short = 'd', long = "description")]
        arg_description: Option<String>,
        #[arg(long = "tag")]
        arg_tags: Option<Vec<String>>,
    },
    #[command(
        about = "Archive a project.",
        long_about = "Marks a project as archived. Archived projects are not shown by default."
    )]
    Archive {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Soft-delete a project.",
        long_about = "Marks a project as deleted without permanently removing it from the database."
    )]
    Delete {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Permanently delete a project.",
        long_about = "Removes the project permanently from the database. Cannot be undone."
    )]
    Purge {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Unarchive a project.",
        long_about = "Reverts the archived status of a project, making it active again."
    )]
    Unarchive {
        #[arg(long = "id")]
        arg_id: String,
    },
    #[command(
        about = "Restore a deleted project.",
        long_about = "Restores a previously soft-deleted project."
    )]
    Restore {
        #[arg(long = "id")]
        arg_id: String,
    },
}
