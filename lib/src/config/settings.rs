use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

use std::env;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SpellApi {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SpellDatasource {
    pub remote_type: String,
    pub cache_time: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SpellSettings {
    pub spell_api: SpellApi,
    pub spell_datasource: SpellDatasource,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Settings {
    pub debug: bool,
    pub spell_settings: SpellSettings,
}

impl Settings {
    pub fn new(folder_path: &String) -> Result<Self, ConfigError> {
        let environment = env::var("ENV").unwrap_or_else(|_| "staging".into());
        let fmt = FileFormat::Yaml;
        let s = Config::builder()
            .add_source(File::new(&format!("{}/default", folder_path), fmt).required(true))
            .add_source(File::new(&format!("{}/{}", folder_path, environment), fmt).required(false))
            .add_source(File::new(&format!("{}/secret", folder_path), fmt).required(false))
            .build()?;

        s.try_deserialize()
    }
}
