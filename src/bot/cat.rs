use super::core::MEDIA_GROUP_RESOURCE;
use super::core::CORE_CONFIG;
use teloxide::prelude::*;

pub async fn post_cat(b: Bot, msg: Message) -> ResponseResult<Message> {
    log::debug!("post_cat handler triggered");

    if msg.reply_to_message().is_none() {
        return b
            .send_message(msg.chat.id, "Please reply to a message")
            .await;
    }

    let replied_message = msg.reply_to_message().unwrap();
    match &replied_message.kind {
        teloxide::types::MessageKind::Common(common) => match &common.media_kind {
            teloxide::types::MediaKind::Photo(photo) => {
                let default_caption = "".to_string();
                let caption = photo.caption.as_ref().unwrap_or(&default_caption);

                if photo.media_group_id.is_none() {
                    let file_id = &photo.photo.last().unwrap().file.id;

                    super::net::download_file(&b, file_id).await?;

                    // Use hash as id
                    let target_id = sha256::digest(file_id);

                    super::net::upload_oss(file_id, &target_id).await?;
                    if !CORE_CONFIG.lock().await.keep_image_cache {
                        super::net::delete_file(file_id).await?;
                    }

                    super::metadata::insert_metadata(caption, target_id).await?;
                    super::metadata::update_metadata().await?;

                    return b
                        .send_message(msg.chat.id, "Posted one cat pic!")
                        .reply_to_message_id(msg.id)
                        .await;
                } else {
                    let mut map = MEDIA_GROUP_RESOURCE.lock().await;
                    let photos: Vec<String> = map
                        .get(&photo.media_group_id.as_ref().unwrap().to_string())
                        .unwrap()
                        .to_vec();
                    map.remove(&photo.media_group_id.as_ref().unwrap().to_string());
                    drop(map);

                    for file_id in &photos {
                        super::net::download_file(&b, file_id).await?;

                        // Use hash as id
                        let target_id = sha256::digest(file_id);
                        super::net::upload_oss(file_id, &target_id).await?;
                        if !CORE_CONFIG.lock().await.keep_image_cache {
                            super::net::delete_file(file_id).await?;
                        }

                        super::net::delete_file(file_id).await?;
                        super::metadata::insert_metadata(caption, target_id).await?;
                    }

                    super::metadata::update_metadata().await?;

                    return b
                        .send_message(msg.chat.id, format!("Posted {} cat pics!", &photos.len()))
                        .reply_to_message_id(msg.id)
                        .await;
                }
            }
            _ => {
                return b.send_message(msg.chat.id, "Please reply to a photo").await;
            }
        },
        _ => {
            return b.send_message(msg.chat.id, "Please reply to a photo").await;
        }
    }
}
