use std::error::Error;

use bot_cli::cli::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(run().await?)
}

// @todo: add tests for main.rs with mocking run function
