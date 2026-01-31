use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};
use telegram_rs_2::{Bot, Result};
use telegram_rs_2::rt::polling::Polling;
use telegram_rs_2::core::requests::SendMessage;
use serde::Deserialize;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    name: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    humidity: u32,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

struct CacheEntry {
    data: String,
    timestamp: Instant,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Weather Bot Example");

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set");
    let api_key = std::env::var("OPENWEATHER_API_KEY")
        .expect("OPENWEATHER_API_KEY must be set");

    let bot = Bot::new(token);
    let mut polling = Polling::new(bot.clone());

    let cache: Arc<Mutex<HashMap<String, CacheEntry>>> = Arc::new(Mutex::new(HashMap::new()));
    let ttl = Duration::from_secs(900); // 15 minutes

    println!("Bot initialized successfully! Starting polling...");

    while let Some(update) = polling.next_update().await? {
        if let Some(msg) = update.message {
            if let Some(text) = msg.text {
                if text.starts_with("/weather ") {
                    let location = text.trim_start_matches("/weather ").trim();
                    if location.is_empty() {
                        let req = SendMessage {
                            chat_id: msg.chat.id,
                            text: "Please provide a location, e.g., /weather London".to_string(),
                            parse_mode: None,
                            reply_markup: None,
                        };
                        bot.execute(req).await?;
                        continue;
                    }

                    let cache_key = location.to_lowercase();
                    let mut cache_lock = cache.lock().await;
                    if let Some(entry) = cache_lock.get(&cache_key) {
                        if entry.timestamp.elapsed() < ttl {
                            let req = SendMessage {
                                chat_id: msg.chat.id,
                                text: entry.data.clone(),
                                parse_mode: None,
                                reply_markup: None,
                            };
                            bot.execute(req).await?;
                            continue;
                        } else {
                            cache_lock.remove(&cache_key);
                        }
                    }
                    drop(cache_lock);

                    // Fetch from API
                    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", location, api_key);
                    match reqwest::get(&url).await {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                match resp.json::<WeatherResponse>().await {
                                    Ok(weather) => {
                                        let description = format!(
                                            "Weather in {}: {}°, {} (Humidity: {}%)",
                                            weather.name,
                                            weather.main.temp,
                                            weather.weather[0].description,
                                            weather.main.humidity
                                        );
                                        let req = SendMessage {
                                            chat_id: msg.chat.id,
                                            text: description.clone(),
                                            parse_mode: None,
                                            reply_markup: None,
                                        };
                                        bot.execute(req).await?;

                                        // Cache the result
                                        let mut cache_lock = cache.lock().await;
                                        cache_lock.insert(cache_key, CacheEntry {
                                            data: description,
                                            timestamp: Instant::now(),
                                        });
                                    }
                                    Err(_) => {
                                        let req = SendMessage {
                                            chat_id: msg.chat.id,
                                            text: "Failed to parse weather data.".to_string(),
                                            parse_mode: None,
                                            reply_markup: None,
                                        };
                                        bot.execute(req).await?;
                                    }
                                }
                            } else {
                                let req = SendMessage {
                                    chat_id: msg.chat.id,
                                    text: "Location not found or API error.".to_string(),
                                    parse_mode: None,
                                    reply_markup: None,
                                };
                                bot.execute(req).await?;
                            }
                        }
                        Err(_) => {
                            let req = SendMessage {
                                chat_id: msg.chat.id,
                                text: "Failed to fetch weather data.".to_string(),
                                parse_mode: None,
                                reply_markup: None,
                            };
                            bot.execute(req).await?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}