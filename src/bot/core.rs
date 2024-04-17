use std::sync::Mutex;

use lazy_static::lazy_static;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::config::{config::CoreConfig, template::config_template};

use super::cat;

lazy_static!{
     pub static ref CORE_CONFIG: Mutex<CoreConfig> = Mutex::new(config_template().core_config);
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
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
            b.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Post => {
            cat::post_cat(b, msg).await?;
        }
        Command::ChatId => {
            b.send_message(msg.chat.id, format!("Chat ID: {}", msg.chat.id)).await?;
        }
    }
    Ok(())
}

pub async fn start_bot(core_config: CoreConfig) {
    log::info!("Starting CatALog bot...");

    *CORE_CONFIG.lock().unwrap() = core_config;

    let bot = Bot::new(CORE_CONFIG.lock().unwrap().bot_token.clone());

    Command::repl(bot, handle_commands).await;
}
