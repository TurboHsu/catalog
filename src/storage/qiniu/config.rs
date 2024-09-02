use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub domain: String,
    pub use_https: bool,
    pub store_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            store_path: "cat".to_string(),
            access_key: "YOUR_ACCESS_KEY".to_string(),
            secret_key: "YOUR_SECRET_KEY".to_string(),
            bucket: "YOUR_BUCKET_NAME".to_string(),
            domain: "YOUR_DOMAIN_NAME".to_string(),
            use_https: true,
        }
    }
}
