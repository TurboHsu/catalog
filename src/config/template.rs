use super::config::{Config, CoreConfig, OssConfig, OssConfigField};

pub fn config_template() -> Config {
    Config {
        core_config: CoreConfig {
            bot_token: "YOUR_BOT_TOKEN".to_string(),
            chat_id: vec![1145141919810],
            cache_dir: "./cache".to_string(),
        },
        oss_config: OssConfig {
            provider: "qiniu".to_string(),
            configuration: OssConfigField {
                qiniu: crate::storage::qiniu::config::Config {
                    store_path: "cat".to_string(),
                    access_key: "YOUR_ACCESS_KEY".to_string(),
                    secret_key: "YOUR_SECRET_KEY".to_string(),
                    bucket: "YOUR_BUCKET_NAME".to_string(),
                    domain: "YOUR_DOMAIN_NAME".to_string(),
                    use_https: true,
                },
            },
        },
    }
}