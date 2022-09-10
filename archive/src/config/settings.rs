use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

use std::env;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct SpellApi {
    pub(super) url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct SpellDatasource {
    pub(super) remote_type: String,
    pub(super) cache_time: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(super) struct SpellSettings {
    pub(super) spell_api: SpellApi,
    pub(super) spell_datasource: SpellDatasource,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Settings {
    pub(super) debug: bool,
    pub(super) spell_settings: SpellSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let environment = env::var("ENV").unwrap_or_else(|_| "staging".into());
        let fmt = FileFormat::Yaml;
        let s = Config::builder()
            .add_source(File::new(".config/default", fmt).required(true))
            .add_source(File::new(&format!(".config/{}", environment), fmt).required(false))
            .add_source(File::new(".config/secret", fmt).required(false))
            .build()?;

        s.try_deserialize()
    }
}
