use std::fmt::Debug;

use crate::command::Command;
use crate::state::BotState;

use teloxide::dispatching::dialogue::Storage;
use teloxide::dispatching::{DpHandlerDescription, HandlerExt, UpdateFilterExt};
use teloxide::dptree::Handler;
use teloxide::prelude::{Message, ResponseResult, Update};

pub type Routes = Handler<'static, ResponseResult<()>, DpHandlerDescription>;

pub fn setup_routes<S>() -> Routes
where
    S: Storage<BotState> + Send + Sync + 'static,
    <S as Storage<BotState>>::Error: Debug + Send,
{
    Update::filter_message()
        .enter_dialogue::<Message, S, BotState>()
        .branch(Command::make_routes())
}
