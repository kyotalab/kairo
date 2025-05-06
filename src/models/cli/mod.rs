pub mod note_cli;
pub mod project_cli;

use clap::{Parser, Subcommand};
use note_cli::NoteCommands;
use project_cli::ProjectCommands;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // ここにProjectCommands, TaskCommandsを追加する!!
    Note {
        #[command(subcommand)]
        command: NoteCommands,
    },
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
}
