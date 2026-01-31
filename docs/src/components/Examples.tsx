import React from 'react';

const Examples: React.FC = () => {
  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-4">Examples</h1>
      <p className="mb-4">Below is an example of how to use the Weather Bot:</p>
      <pre className="bg-gray-200 p-4 rounded">
        <code>
{`use telegram_rs::{Bot, Result};
use telegram_rs::rt::polling::Polling;
use telegram_rs::core::requests::SendMessage;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("TELEGRAM_BOT_TOKEN")?;
    let bot = Bot::new(token);
    let mut polling = Polling::new(bot.clone());

    while let Some(update) = polling.next_update().await? {
        if let Some(message) = update.message {
            if let Some(text) = message.text {
                if text.starts_with("/weather") {
                    let location = text[9..].trim();
                    let response = format!("Fetching weather for: {}", location);
                    let req = SendMessage {
                        chat_id: message.chat.id,
                        text: response,
                        parse_mode: None,
                        reply_markup: None,
                    };
                    bot.execute(req).await?;
                }
            }
        }
    }

    Ok(())
}`}
        </code>
      </pre>
      <p className="mt-4">To run this example, make sure to set the following environment variables:</p>
      <ul className="list-disc list-inside">
        <li><code>TELEGRAM_BOT_TOKEN</code>: Your Telegram bot token</li>
        <li><code>OPENWEATHER_API_KEY</code>: Your OpenWeatherMap API key</li>
      </ul>
      <p className="mt-4">Run the bot using the following command:</p>
      <pre className="bg-gray-200 p-4 rounded">
        <code>cargo run --example weather_bot</code>
      </pre>
    </div>
  );
};

export default Examples;