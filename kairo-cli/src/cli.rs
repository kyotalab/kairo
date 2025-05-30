use crate::{
    commands::{Cli, Commands},
    handler::{self},
    kairo_core::config::AppConfig,
};
use diesel::SqliteConnection;

pub fn dispatch(cli: Cli, conn: &mut SqliteConnection, config: &AppConfig) {
    match cli.command {
        Commands::Note { command } => handler::handle_note_command(command, conn, config),
        Commands::Project { command } => handler::handle_project_command(command, conn, config),
        Commands::Task { command } => handler::handle_task_command(command, conn, config),
        Commands::Tag { command } => handler::handle_tag_command(command, conn),
        Commands::Link { command } => handler::handle_link_command(command, conn),
    }
}
