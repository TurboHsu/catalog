use teloxide::prelude::*;

pub async fn post_cat(b: Bot, msg: Message) -> ResponseResult<Message> {
    if msg.reply_to_message().is_none() {
        return b.send_message(msg.chat.id, "Please reply to a message").await;
    }

    let replied_message = msg.reply_to_message().unwrap();
    match &replied_message.kind {
        teloxide::types::MessageKind::Common(common) => {
            match &common.media_kind {
                teloxide::types::MediaKind::Photo(photo) => {
                    let photo_len = photo.photo.len();
                    b.send_message(msg.chat.id, format!("You have sent {} photos", photo_len)).await?;
                },
                _ => ()
            }
        },
        _ => ()
    }

    


    println!("{:?}", replied_message.kind);
    b.send_message(msg.chat.id, "You are my true dad!").await
}