use clap::Parser;
use kairo::cli;
use kairo::commands::Cli;
use kairo::store::db::establish_connection;
use kairo::util::load_config;

fn main() {
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("❌ Failed to load config: {}", e);
        std::process::exit(1);
    });

    let conn = &mut establish_connection(&config);

    let cli = Cli::parse();
    cli::dispatch(cli, conn, &config);
}
