use std::env::var;

use super::CliConfig;

#[derive(Debug)]
pub enum Error {
    MissingToken,
}

/// Build a `CliConfig` from environment variables.
///
/// Loads variables from a local `.env` file (if present) using `dotenvy`,
/// then reads:
/// - `BOT_TELEGRAM_TOKEN` (required)
/// - `BOT_DB_FILE` (optional, defaults to `"bot.db"`)
///
/// # Errors
///
/// Returns `Error::MissingToken` if the `BOT_TELEGRAM_TOKEN` environment
/// variable is not set or is otherwise unavailable.
pub fn make_config() -> Result<CliConfig, Error> {
    dotenvy::dotenv().ok();

    Ok(CliConfig {
        token: var("BOT_TELEGRAM_TOKEN").map_err(|_| Error::MissingToken)?,
        db_file: var("BOT_DB_FILE").unwrap_or_else(|_| String::from("bot.db")),
    })
}
