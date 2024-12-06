use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;

mod coinmarketcap;

#[tokio::main]
async fn main() {

    // Loads env variables

    dotenv().ok();

    // Check that the RUST_LOG environment variable is set correctly

    if let Ok(log_level) = env::var("RUST_LOG") {
        println!("Logging level: {}", log_level);
    }

    // Optional: Print the loaded token to verify it's being read correctly
    
    if let Ok(teloxide_token) = env::var("TELOXIDE_TOKEN") {
        println!("Loaded TELOXIDE_TOKEN: {}", teloxide_token);
    } else {
        println!("TELOXIDE_TOKEN is not set!");
    }

    if let Ok(cmk_token) = env::var("COINMARKETCAP_TOKEN"){
        println!("Loaded COINMARKETCAP_TOKEN token {}", cmk_token)
    }else{
        println!("COINMARKETCAP_TOKEN is not set!")
    }

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    //takes in my bot from .env, 
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;





}
