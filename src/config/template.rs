use super::config::Config;

pub fn config_template() -> Config {
    Config {
        bot_token: String::from("YOUR_BOT_TOKEN"),
    }
}