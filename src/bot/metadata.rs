use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use teloxide::RequestError;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub data: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub caption: String,
    pub cats: Vec<String>,
}

lazy_static! {
    static ref METADATA: Arc<Mutex<Metadata>> = Arc::new(Mutex::new(Metadata { data: Vec::new() }));
    static ref METADATA_PATH: Arc<Mutex<String>> =
        Arc::new(Mutex::new("metadata.json".to_string()));
}

pub async fn init_metadata(path: &str) {
    let pth = std::path::Path::new(path);

    // Open the file, creating it if it doesn't exist
    let file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&pth)
        .await
        .unwrap();

    // If the file was just created, write the default METADATA value to it
    if file.metadata().await.unwrap().len() == 0 {
        let default_metadata = serde_json::to_string(&*METADATA.lock().await).unwrap();
        tokio::fs::write(&pth, default_metadata).await.unwrap();
    }

    let mut metadata = METADATA.lock().await;
    *metadata = serde_json::from_str(&std::fs::read_to_string(pth).unwrap()).unwrap();
    drop(metadata);

    // Update path
    let mut metadata_path = METADATA_PATH.lock().await;
    *metadata_path = path.to_owned();
    drop(metadata_path);
}

pub async fn insert_metadata(caption: &str, cats: Vec<String>) -> Result<(), RequestError>{
    let mut metadata = METADATA.lock().await;
    metadata.data.push(Field {
        caption: caption.to_string(),
        cats,
    });
    drop(metadata);
    update_metadata().await
}

pub async fn update_metadata() -> Result<(), RequestError> {
    // Write to file
    let metadata = METADATA.lock().await;
    std::fs::write(
        &*METADATA_PATH.lock().await,
        serde_json::to_string(&*metadata).unwrap(),
    )
    .unwrap();
    drop(metadata);

    // Call upload
    let metadata_path = METADATA_PATH.lock().await;
    let storage = super::core::STORAGE.lock().await;
    storage
        .upload(&metadata_path, "metadata.json")
        .map_err(|e| {
            RequestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
    log::debug!("Uploaded metadata");
    drop(storage);
    drop(metadata_path);
    Ok(())
}
