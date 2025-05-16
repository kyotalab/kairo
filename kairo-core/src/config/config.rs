use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub paths: PathsConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PathsConfig {
    pub db_path: String,
    pub notes_dir: String,
    pub projects_dir: String,
    pub tasks_dir: String,
}
