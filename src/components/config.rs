use super::utils::get_default_config;
use serde::{Deserialize, Serialize};
use std::path::Path;
//use toml::de::ValueDeserializer;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    name: String,
    rootfs_url: String,
    fs_dir: String,
}

#[allow(dead_code)]
impl Config {
    pub async fn new() -> Self {
        let path = Path::new("config/config.toml");
        if !path.exists() {
            get_default_config()
                .await
                .expect("Failed to get default config");
        }
        let config = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config).unwrap()
    }

    pub fn from(path: &Path) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config).unwrap()
    }

    pub fn fs_dir(&self) -> &str {
        &self.fs_dir
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rootfs_url(&self) -> &str {
        &self.rootfs_url
    }
}
