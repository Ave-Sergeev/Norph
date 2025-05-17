use anyhow::Result;
use config::{Config, File};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub host: String,
    pub port: i32,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 9090,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Logging {
    pub log_level: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct EmbeddingSettings {
    pub model_path: String,
    pub max_text_size: config_types::ByteSizeConf,
}

impl Default for EmbeddingSettings {
    fn default() -> Self {
        Self {
            model_path: "model".to_string(),
            max_text_size: config_types::ByteSizeConf::of_mebibytes(10),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    pub server: Server,
    pub logging: Logging,
    pub embeddings: EmbeddingSettings,
}

impl Settings {
    pub fn new(location: &str) -> Result<Self> {
        let mut builder = Config::builder();

        if Path::new(location).exists() {
            builder = builder.add_source(File::with_name(location));
        } else {
            log::warn!("Configuration file not found");
        }

        let settings = builder.build()?.try_deserialize()?;

        Ok(settings)
    }

    pub fn json_pretty(&self) -> String {
        to_string_pretty(&self).expect("Failed serialize")
    }
}
