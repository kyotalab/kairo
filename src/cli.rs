use crate::handlers::{self};
use crate::models::{Cli, Commands};

pub fn dispatch(cli: Cli) {
    match cli.command {
        Commands::Note { command } => handlers::handle_note_command(command),
        Commands::Project { command } => handlers::handle_project_command(command),
    }
}
