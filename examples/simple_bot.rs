//! Example: Simple Telegram Bot
//!
//! This example demonstrates how to create a basic Telegram bot using telegram-rs.
//! It shows basic setup and message handling.
//!
//! # Setup
//! 1. Create a bot with BotFather on Telegram to get your bot token
//! 2. Set the TELEGRAM_BOT_TOKEN environment variable
//! 3. Run: cargo run --example simple_bot

use telegram_rs::Bot;
use telegram_rs::rt::polling::Polling;

#[tokio::main]
async fn main() -> telegram_rs::Result<()> {
    println!("Simple Bot Example");
    
    // Get bot token from environment
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or("8241173620:AAFTsEgLr7ZeXTWotBtGNECcaTUqLOHN8as".to_string());
    
    // Create bot instance
    let bot = Bot::new(token);
    
    // Create polling instance
    let mut polling = Polling::new(bot);
    
    println!("Bot initialized successfully! Starting polling...");
    
    while let Some(update) = polling.next_update().await? {
        println!("Received update: {:?}", update);
    }
    
    Ok(())
}
