use std::fmt::Debug;
use std::sync::Arc;

use teloxide::dispatching::UpdateHandler;
use teloxide::dispatching::dialogue::Storage;
use teloxide::prelude::Dispatcher;
use teloxide::{Bot, dptree};

pub async fn init_bot<State, T, U>(token: &str, storage: Arc<T>, routes: UpdateHandler<U>)
where
    State: Send + Sync + 'static,
    T: Storage<State> + Send + Sync + 'static,
    U: Debug + Send + Sync + 'static,
{
    let bot = Bot::new(token);

    let dispatcher = Dispatcher::builder(bot, routes)
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![storage]);

    dispatcher.build().dispatch().await;
}
