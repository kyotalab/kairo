use clap::Parser;
use kairo_cli::{cli, commands::Cli};
use kairo_core::{store::db::establish_connection, util::load_config};

fn main() {
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("❌ Failed to load config: {}", e);
        std::process::exit(1);
    });

    let conn = &mut establish_connection(&config);

    let cli = Cli::parse();
    cli::dispatch(cli, conn, &config);
}
