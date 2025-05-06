use clap::Parser;
use kairo::cli;
use kairo::models::Cli;

fn main() {
    let cli = Cli::parse();
    cli::dispatch(cli);
}
