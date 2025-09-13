use bot_lib::dice::{Dice, DiceError};

use teloxide::prelude::{Message, Requester};
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{Bot, RequestError};

pub fn is_dice(msg: Message) -> bool {
    dice_parse_command(&msg).is_ok()
}

pub async fn send_dice(bot: Bot, msg: Message, dice: Dice) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, dice.throw().to_string())
        .reply_to(msg.id)
        .await?;
    Ok(())
}

pub async fn answer(bot: Bot, msg: Message) -> Result<(), RequestError> {
    send_dice(bot, msg, Dice::new_once(20)).await
}

pub async fn answer_by_msg(bot: Bot, msg: Message) -> Result<(), RequestError> {
    match dice_parse_command(&msg) {
        Ok(dice) => {
            send_dice(bot, msg, dice).await?;
        }
        Err(_) => {
            bot.send_message(
                msg.chat.id,
                "Некорректный формат кубика. Используйте нотацию вроде: d6, 2d10, 2d4m3.",
            )
            .reply_to(msg.id)
            .await?;
        }
    };

    Ok(())
}

fn dice_parse_command(msg: &Message) -> Result<Dice, DiceError> {
    let text = msg.text().unwrap_or_default();
    let text = text.trim();

    if text.is_empty() || !text.starts_with('/') {
        return Err(DiceError::InvalidText);
    }

    Dice::parse_text(&text[1..])
}
