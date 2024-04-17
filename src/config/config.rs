use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::fs::{self, File};
use toml;

use super::template::config_template;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
}

pub fn load_config(path: String) -> Config {
    let config_path = Path::new(&path);
    if !config_path.exists() {
        let template_config = config_template();
        write_config(&path, template_config);
        println!("[I] Config file not found, created template config file at: {}", path);
        std::process::exit(0);
    }
    
    let file_content = fs::read_to_string(path).unwrap();
    let config: Config = toml::from_str(&file_content).unwrap();

    config
}

pub fn write_config(path: &String, config: Config) {
    let config_str = toml::to_string(&config).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(config_str.as_bytes()).unwrap();
}