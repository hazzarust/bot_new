mod coinmarketcap;
mod telegram_bot;

use dotenv::dotenv;
use std::env;
use crate::coinmarketcap::hype_price;
use crate::telegram_bot::{Command, answer};
use teloxide::prelude::*;

#[tokio::main]
async fn main() {

    // Loads env variables
    dotenv().ok();
    
    // Check that the RUST_LOG environment variable is set correctly

    if let Ok(log_level) = env::var("RUST_LOG") {
        println!("Logging level: {}", log_level);
    }

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;


}








