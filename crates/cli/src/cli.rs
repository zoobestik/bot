use std::fmt::Debug;
use std::sync::Arc;

use bot_core::routes::{Routes, setup_routes};
use bot_core::state::BotState;

use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::{SqliteStorage, Storage};

use bot::init_bot;
use config::env::{Error as EnvError, make_config};

pub mod bot;
pub mod config;

/// Run the CLI entrypoint: load configuration, initialize storage and routes,
/// and start the bot.
///
/// This function loads configuration from environment variables (optionally
/// from a local `.env` via `dotenvy`), opens the bot's dialogue storage,
/// builds routes, and launches the bot.
///
/// # Errors
///
/// Returns `Err(String)` when:
/// - the `BOT_TELEGRAM_TOKEN` environment variable is missing or unreadable;
/// - the `SQLite` dialogue storage cannot be opened (e.g., invalid path or I/O error).
pub async fn run() -> Result<(), String> {
    let config = match make_config() {
        Ok(config) => config,
        Err(EnvError::MissingToken) => {
            return Err("BOT_TELEGRAM_TOKEN environment variable is not set".to_string());
        }
    };

    let storage = match SqliteStorage::open(&config.db_file, Json).await {
        Ok(storage) => storage,
        Err(e) => return Err(format!("Storage init issue: {e}")),
    };

    let routes = setup_routes_for(&storage);

    init_bot::<BotState, _, _>(&config.token, storage, routes).await;
    Ok(())
}

pub fn setup_routes_for<S>(_storage: &Arc<S>) -> Routes
where
    S: Storage<BotState> + Send + Sync + 'static,
    <S as Storage<BotState>>::Error: Debug + Send,
{
    setup_routes::<S>()
}
