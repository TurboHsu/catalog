use teloxide::prelude::*;

pub async fn post_cat(b: Bot, msg: Message) -> ResponseResult<Message> {
    let valid_chat_id = super::core::CORE_CONFIG.lock().unwrap().chat_id.clone();
    if !valid_chat_id.contains(&msg.chat.id.0) {
        return b.send_message(msg.chat.id, "You are not authorized to use this bot").await;
    }

    b.send_message(msg.chat.id, "You are my true dad!").await
}