use crate::commands::{Cli, Commands};
use crate::config::AppConfig;
use crate::handlers::{self};
use diesel::SqliteConnection;

pub fn dispatch(cli: Cli, conn: &mut SqliteConnection, config: &AppConfig) {
    match cli.command {
        Commands::Note { command } => handlers::handle_note_command(command, conn, config),
        Commands::Project { command } => handlers::handle_project_command(command, conn, config),
        Commands::Task { command } => handlers::handle_task_command(command, conn, config),
        Commands::Tag { command } => handlers::handle_tag_command(command, conn),
        Commands::Link { command } => handlers::handle_link_command(command, conn),
    }
}
