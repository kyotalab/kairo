use crate::config::AppConfig;
use config::{Config, ConfigError, File};
use etcetera::{BaseStrategy, choose_base_strategy};

pub fn load_config() -> Result<AppConfig, ConfigError> {
    // Documentでconfigパスを限定することが前提

    let strategy = choose_base_strategy().expect("Unable to find the config directory!");
    let mut path = strategy.config_dir();
    path.push("kairo");
    path.push("config.toml");

    if !path.exists() {
        eprintln!("No config file found at: {}", path.display());
    }

    let builder = Config::builder().add_source(File::from(path));

    builder.build()?.try_deserialize()
}
