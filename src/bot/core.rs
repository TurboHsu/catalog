use teloxide::prelude::*;

pub async fn start_bot(token: String) {
    log::info!("Starting CatALog bot...");

    let bot = Bot::new(token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}