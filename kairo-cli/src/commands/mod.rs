pub mod linked_note;
pub mod note;
pub mod project;
pub mod tag;
pub mod task;

use clap::{Parser, Subcommand};
use linked_note::LinkCommands;
use note::NoteCommands;
use project::ProjectCommands;
use tag::TagCommands;
use task::TaskCommands;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Note {
        #[command(subcommand)]
        command: NoteCommands,
    },
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    Tag {
        #[command(subcommand)]
        command: TagCommands,
    },
    Link {
        #[command(subcommand)]
        command: LinkCommands,
    },
}
