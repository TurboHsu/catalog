use super::config::{Config, CoreConfig};

pub fn config_template() -> Config {
    Config {
        core_config: CoreConfig {
            bot_token: "YOUR_BOT_TOKEN".to_string(),
            chat_id: vec![1145141919810],
            cache_dir: "./cache".to_string(),
        }
    }
}