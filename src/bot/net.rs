use std::sync::Arc;
use tokio::sync::Mutex;
use teloxide::{net::Download, prelude::*};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CACHE_DIR: Arc<Mutex<String>> = Arc::new(Mutex::new("".to_string()));
}


pub async fn download_file(b: &Bot, file_id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let origin = b.get_file(file_id).await?;

    let global_cache_dir = CACHE_DIR.lock().await;
    let cache_dir = global_cache_dir.clone();
    drop(global_cache_dir);

    let mut dst = tokio::fs::File::create(format!("{}/{}.png", cache_dir, file_id)).await?;
    b.download_file(&origin.path, &mut dst).await?;
    log::debug!("Downloaded file: {}", file_id);

    Ok(())
}