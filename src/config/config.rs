use std::error::Error;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::fs::{self, File};
use toml;

use super::template::config_template;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub core_config: CoreConfig,
}

#[derive(Serialize, Deserialize)]
pub struct CoreConfig {
    pub bot_token: String,
    pub chat_id: Vec<u64>,
    pub cache_dir: String,
}

pub fn read(path: String) -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new(&path);
    if !config_path.exists() {
        let template_config = config_template();
        write(&path, template_config)?;
        println!("[I] Config file not found, created template config file at: {}", path);
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