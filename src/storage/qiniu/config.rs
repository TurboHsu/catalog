use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub domain: String,
    pub use_https: bool,
    pub store_path: String,
}