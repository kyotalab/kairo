use clap::Parser;
use kairo::cli;
use kairo::commands::Cli;

fn main() {
    let cli = Cli::parse();
    cli::dispatch(cli);
}
