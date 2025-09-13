use crate::routes::Routes;

use teloxide::dispatching::HandlerExt;
use teloxide::macros::BotCommands;
use teloxide::prelude::ResponseResult;
use teloxide::types::Message;
use teloxide::{Bot, dptree};

mod ball;
mod dice;
mod help;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "посмотреть список всех команд\n")]
    Help,
    #[command(aliases = ["8", "8ball"], description = "спросить всевидящий шар")]
    Ball,
    #[command(
        aliases = ["d20","d"],
        description = "бросить кубик с X-сторонами один или Y раз\n    (с модификатором) – /2d4m3, /d6m2 и т.д."
    )]
    Dice,
}

impl Command {
    pub fn make_routes() -> Routes {
        dptree::entry()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(Command::command_handler),
            )
            .branch(dptree::filter(dice::is_dice).endpoint(dice::answer_by_msg))
    }

    async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => help::answer(bot, msg).await?,
            Command::Ball => ball::answer(bot, msg).await?,
            Command::Dice => dice::answer(bot, msg).await?,
        }

        Ok(())
    }
}
