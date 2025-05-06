pub mod note_cli;

use note_cli::NoteCommands;

use clap::{Parser, Subcommand};

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
}
