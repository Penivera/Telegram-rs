use telegram_rs_2::{Bot, Result};
use telegram_rs_2::rt::polling::Polling;

#[tokio::main]
async fn main() -> Result<()> {
    // This example requires a token
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string());
    let bot = Bot::new(token);

    let mut polling = Polling::new(bot.clone());
    println!("Starting polling...");

    while let Some(update) = polling.next_update().await? {
        println!("Got update: {:?}", update);
        // Handle update
    }

    Ok(())
}
