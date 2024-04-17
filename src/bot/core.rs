use teloxide::{prelude::*, utils::command::BotCommands};

use crate::config::config::CoreConfig;

use super::cat;

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

pub async fn start_bot(core_config: CoreConfig) {
    log::info!("Starting CatALog bot...");

    let bot = Bot::new(core_config.bot_token);

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter(move |msg: Message| {
                    !core_config.chat_id.contains(&msg.from().unwrap().id.0)
                })
                .endpoint(|msg: Message, bot: Bot| async move {
                    bot.send_message(
                        msg.chat.id,
                        "You do not have enough cat power to use this bot!",
                    )
                    .await?;
                    respond(())
                }),
        )
        .branch(
            dptree::entry()
                .filter(move |msg: Message| {
                    let mut flag: bool = false;
                    match &msg.kind {
                        teloxide::types::MessageKind::Common(common) => match &common.media_kind {
                            teloxide::types::MediaKind::Photo(photo) => {
                                if photo.media_group_id.is_some() {
                                    flag = true;
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    };
                    flag
                })
                .endpoint(|msg: Message, bot: Bot| async move { handle_group_media(msg, bot) }),
        )
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_commands),
        );

    Dispatcher::builder(bot, handler)
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error occurred in dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn handle_group_media(msg: Message, bot: Bot) -> ResponseResult<()> {
    println!("{:?}", msg);
    Ok(())
}
