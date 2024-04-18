use lazy_static::lazy_static;
use std::sync::Arc;
use teloxide::{net::Download, prelude::*, RequestError};
use tokio::sync::Mutex;

lazy_static! {
    pub static ref CACHE_DIR: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
}

pub async fn download_file(b: &Bot, file_id: &String) -> Result<(), RequestError> {
    let origin = b.get_file(file_id).await?;

    let global_cache_dir = CACHE_DIR.lock().await;
    let cache_dir = global_cache_dir.clone();
    drop(global_cache_dir);

    // Create the directory if it doesn't exist
    tokio::fs::create_dir_all(&cache_dir).await?;

    let mut dst = tokio::fs::File::create(format!("{}/{}.png", cache_dir, file_id)).await?;
    b.download_file(&origin.path, &mut dst).await?;
    log::debug!("Downloaded file: {}", file_id);

    Ok(())
}

pub async fn upload_oss(file_id: &String) -> Result<(), RequestError> {
    let global_cache_dir = CACHE_DIR.lock().await;
    let cache_dir = global_cache_dir.clone();
    drop(global_cache_dir);

    let storage = super::core::STORAGE.lock().await;
    storage
        .upload(
            &format!("{}/{}.png", cache_dir, file_id),
            &format!("{}.png", file_id),
        )
        .map_err(|e| {
            RequestError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
    log::debug!("Uploaded file: {}", file_id);
    drop(storage);

    Ok(())
}

pub async fn delete_file(file_id: &String) -> Result<(), RequestError> {
    let global_cache_dir = CACHE_DIR.lock().await;
    let cache_dir = global_cache_dir.clone();
    drop(global_cache_dir);

    tokio::fs::remove_file(format!("{}/{}.png", cache_dir, file_id)).await?;
    log::debug!("Deleted file: {}", file_id);

    Ok(())
}
