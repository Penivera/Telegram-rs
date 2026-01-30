#[cfg(feature = "webhook")]
use telegram_rs::{Bot, Update};
#[cfg(feature = "webhook")]
use telegram_rs::rt::webhook::Webhook;

#[cfg(feature = "webhook")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string());
    let bot = Bot::new(token);

    println!("Starting webhook...");
    Webhook::new(bot)
        .port(8080)
        .run(|update: Update, _bot: Bot| async move {
            println!("Got update via webhook: {:?}", update);
        })
        .await
}

#[cfg(not(feature = "webhook"))]
fn main() {
    println!("Webhook feature not enabled");
}
