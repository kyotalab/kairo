use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;

use crate::config::AppConfig;

pub fn establish_connection(config: &AppConfig) -> SqliteConnection {
    // dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_url = &config.paths.db_path;
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
