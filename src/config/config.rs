use core::str;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub core_config: CoreConfig,
    pub oss_config: OssConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            core_config: CoreConfig::default(),
            oss_config: OssConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CoreConfig {
    pub bot_token: String,
    pub chat_id: Vec<u64>,
    pub cache_dir: String,
    pub metadata_path: String,
    pub keep_image_cache: bool,
}

impl Default for CoreConfig {
    fn default() -> Self {
        CoreConfig {
            bot_token: "YOUR_BOT_TOKEN".to_string(),
            chat_id: vec![1145141919810],
            cache_dir: "./cache".to_string(),
            metadata_path: "./metadata.json".to_string(),
            keep_image_cache: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OssConfig {
    pub provider: String,
    pub configuration: OssConfigField,
}

#[derive(Serialize, Deserialize)]
pub struct OssConfigField {
    pub qiniu: crate::storage::qiniu::config::Config,
}

impl Default for OssConfig {
    fn default() -> Self {
        OssConfig {
            provider: "qiniu".to_string(),
            configuration: OssConfigField {
                qiniu: crate::storage::qiniu::config::Config::default(),
            },
        }
    }
}

pub fn read(path: String) -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new(&path);
    if !config_path.exists() {
        let template_config = Config::default();
        write(&path, template_config)?;
        println!(
            "[I] Config file not found, created template config file at: {}",
            path
        );
        std::process::exit(0);
    }

    let file_content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&file_content)?;

    Ok(config)
}

pub fn write(path: &String, config: Config) -> Result<(), Box<dyn Error>> {
    let config_str = toml::to_string(&config)?;
    let mut file = File::create(path)?;
    file.write_all(config_str.as_bytes())?;
    Ok(())
}
