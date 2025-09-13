use teloxide::prelude::Requester;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;
use teloxide::{Bot, RequestError};

use super::Command;

pub async fn answer(bot: Bot, msg: Message) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
