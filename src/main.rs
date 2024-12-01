use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {

    // Loads env variables

    dotenv().ok();

    // Check that the RUST_LOG environment variable is set correctly

    if let Ok(log_level) = env::var("RUST_LOG") {
        println!("Logging level: {}", log_level);
    }

    // Optional: Print the loaded token to verify it's being read correctly
    
    if let Ok(token) = env::var("TELOXIDE_TOKEN") {
        println!("Loaded TELOXIDE_TOKEN: {}", token);
    } else {
        println!("TELOXIDE_TOKEN is not set!");
    }

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
