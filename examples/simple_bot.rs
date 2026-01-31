use telegram_rs_2::Bot;
use telegram_rs_2::rt::polling::Polling;
use telegram_rs_2::core::types::{InlineKeyboardBuilder, InlineKeyboardButton};
use telegram_rs_2::core::requests::{SendMessage, AnswerCallbackQuery};

#[tokio::main]
async fn main() -> telegram_rs_2::Result<()> {
    println!("Simple Bot Example with Buttons");
    
    // Get bot token from environment
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or("8241173620:AAFTsEgLr7ZeXTWotBtGNECcaTUqLOHN8as".to_string());
    
    // Create bot instance
    let bot = Bot::new(token);
    
    // Create polling instance
    let mut polling = Polling::new(bot.clone());
    
    println!("Bot initialized successfully! Starting polling...");
    
    while let Some(update) = polling.next_update().await? {
        if let Some(msg) = update.message {
            if let Some(text) = msg.text {
                if text == "/start" {
                    println!("Sending keyboard to chat: {}", msg.chat.id);
                    let keyboard = InlineKeyboardBuilder::new()
                        .row(vec![
                            InlineKeyboardButton::callback("Button 1", "btn_1"),
                            InlineKeyboardButton::callback("Button 2", "btn_2"),
                        ])
                        .build();

                    let req = SendMessage {
                        chat_id: msg.chat.id,
                        text: "Here is a keyboard!".to_string(),
                        parse_mode: None,
                        reply_markup: Some(keyboard),
                    };
                    
                    if let Err(e) = bot.execute(req).await {
                         eprintln!("Failed to send message: {}", e);
                    }
                }
            }
        }
        
        if let Some(callback) = update.callback_query {
            println!("Received callback: {} from data: {:?}", callback.id, callback.data);
            
            let answer = AnswerCallbackQuery {
                callback_query_id: callback.id,
                text: Some("Button clicked!".to_string()),
                show_alert: Some(false),
                url: None,
                cache_time: None,
            };
            
            if let Err(e) = bot.execute(answer).await {
                 eprintln!("Failed to answer callback: {}", e);
            }
        }
    }
    
    Ok(())
}
