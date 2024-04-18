use super::cat;
use crate::{config::config::CoreConfig, storage};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc};
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::sync::Mutex;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]

enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "post cats")]
    Post,
    #[command(description = "return chat id")]
    ChatId,
}

lazy_static! {
    pub static ref MEDIA_GROUP_RESOURCE: Arc<Mutex<HashMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref STORAGE: Mutex<Box<dyn storage::file::ObjectStorage>> =
        Mutex::new(Box::new(storage::file::Default::new()));
}

async fn handle_commands(b: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            b.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Post => {
            cat::post_cat(b, msg).await?;
        }
        Command::ChatId => {
            b.send_message(msg.chat.id, format!("Chat ID: {}", msg.chat.id))
                .await?;
        }
    }
    Ok(())
}

pub async fn start_bot(core_config: CoreConfig, storage: Box<dyn storage::file::ObjectStorage>) {
    log::info!("Starting CatALog bot...");

    let bot = Bot::new(core_config.bot_token);

    // Put config
    let mut download_cache_dir = super::net::CACHE_DIR.lock().await;
    *download_cache_dir = core_config.cache_dir.clone();
    drop(download_cache_dir);

    // Put storage
    let mut storage_ref = STORAGE.lock().await;
    *storage_ref = storage;
    drop(storage_ref);

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter(move |msg: Message| {
                    !core_config.chat_id.contains(&msg.from().unwrap().id.0)
                })
                .endpoint(|msg: Message, bot: Bot| async move {
                    log::debug!("Unauthorized user: {:?}", msg.from().unwrap().id.0);
                    // Detect for /id command
                    if msg.text().unwrap().starts_with("/chatid") {
                        bot.send_message(msg.chat.id, format!("Chat ID: {}", msg.chat.id))
                            .await?;
                    } else {
                        bot.send_message(
                            msg.chat.id,
                            "You do not have enough cat power to use this bot!",
                        )
                        .await?;
                    }
                    respond(())
                }),
        )
        .branch(
            dptree::entry()
                .filter(move |msg: Message| {
                    match &msg.kind {
                        teloxide::types::MessageKind::Common(common) => match &common.media_kind {
                            teloxide::types::MediaKind::Photo(photo) => {
                                if photo.media_group_id.is_some() {
                                    return true;
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    };
                    false
                })
                .endpoint(|msg: Message| async move { handle_group_media(msg).await }),
        )
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_commands),
        );

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            log::debug!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error occurred in dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_group_media(msg: Message) -> ResponseResult<()> {
    let mut map = MEDIA_GROUP_RESOURCE.lock().await;

    map.entry(msg.media_group_id().unwrap().to_string())
        .or_insert_with(Vec::new)
        .push(msg.photo().unwrap().last().unwrap().file.id.to_string());

    log::debug!(
        "received media group {} with photo {}.",
        msg.media_group_id().unwrap(),
        msg.photo().unwrap().first().unwrap().file.id
    );
    Ok(())
}
