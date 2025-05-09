use crate::config::AppConfig;
use config::{Config, ConfigError, File};
use std::path::PathBuf;

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let mut builder = Config::builder();

    // 優先順位: カレントディレクトリ > ユーザー設定 > システム全体
    let config_paths = vec![
        Some(PathBuf::from("./config.toml")),
        dirs::config_dir().map(|p| p.join("kairo/config.toml")),
        Some(PathBuf::from("/etc/kairo/config.toml")),
    ];

    for path_opt in config_paths {
        if let Some(path) = path_opt {
            if path.exists() {
                builder = builder.add_source(File::from(path));
                break;
            }
        }
    }

    builder.build()?.try_deserialize()
}
