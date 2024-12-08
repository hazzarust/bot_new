use teloxide::{prelude::*, utils::command::BotCommands};
use crate::coinmarketcap::hype_price;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "display price of ticker")]
    Ticker(String),
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        },
        Command::Ticker(ticker) => {
            // Assuming `hype_price` takes the ticker symbol as input.
            match hype_price(&ticker).await {
                Ok(price) => {
                    bot.send_message(msg.chat.id, format!("The price of {} is ${}", ticker, price)).await?;
                },
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error fetching price for {}: {}", ticker, e)).await?;
                },
            }
        },
    };

    // Ensure this function returns `ResponseResult<()>`
    Ok(())
}
