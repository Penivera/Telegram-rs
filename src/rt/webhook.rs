//! Production-grade HTTP Webhook server powered by Actix-Web.
//!
//! Available when the `webhook` Cargo feature is enabled.

use crate::core::types::Update;
use crate::Bot;
use actix_web::{web, App, HttpResponse, HttpServer};

/// Webhook server for receiving push updates from the Telegram servers.
///
/// Built on top of [Actix-Web](https://actix.rs/), this runner listens for incoming POST
/// requests at the configured path and dispatches incoming updates to your async handler.
///
/// # Examples
///
/// ```no_run
/// #[cfg(feature = "webhook")]
/// {
///     use telegram_rs_2::core::types::Update;
///     use telegram_rs_2::rt::webhook::Webhook;
///     use telegram_rs_2::{Bot, Result};
///
///     #[tokio::main]
///     async fn main() -> Result<()> {
///         let bot = Bot::new("TOKEN".to_string());
///         Webhook::new(bot)
///             .port(8443)
///             .path("/webhook".to_string())
///             .run(|update: Update, _bot: Bot| async move {
///                 println!("Webhook update received: {:?}", update);
///             })
///             .await?;
///         Ok(())
///     }
/// }
/// ```
pub struct Webhook {
    bot: Bot,
    path: String,
    port: u16,
}

impl Webhook {
    /// Creates a new `Webhook` server instance bound to port `8080` and route `"/webhook"` by default.
    ///
    /// # Arguments
    /// * `bot` - Instance of [`Bot`].
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            path: "/webhook".to_string(),
            port: 8080,
        }
    }

    /// Sets the HTTP endpoint path that will handle incoming Telegram updates.
    ///
    /// # Arguments
    /// * `path` - The route path, e.g. `"/my_secret_token"`.
    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    /// Sets the TCP port on which the HTTP server will bind (defaults to `8080`).
    ///
    /// # Arguments
    /// * `port` - Port number (e.g. `8443`, `8080`, `443`).
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Starts the Actix-Web HTTP server and blocks until terminated.
    ///
    /// Each incoming valid update is passed to the given asynchronous `handler` function.
    ///
    /// # Arguments
    /// * `handler` - An async callback taking an [`Update`] and a [`Bot`] clone.
    pub async fn run<F, Fut>(self, handler: F) -> std::io::Result<()>
    where
        F: Fn(Update, Bot) -> Fut + Send + Sync + 'static + Clone,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let bot = self.bot.clone();
        let path = self.path.clone();

        HttpServer::new(move || {
            let bot_clone = bot.clone();
            let handler_clone = handler.clone();

            App::new().app_data(web::Data::new(bot_clone)).route(
                &path,
                web::post().to(move |update: web::Json<Update>, bot_data: web::Data<Bot>| {
                    let bot = bot_data.get_ref().clone();
                    let handler = handler_clone.clone();
                    async move {
                        handler(update.into_inner(), bot).await;
                        HttpResponse::Ok().finish()
                    }
                }),
            )
        })
        .bind(("0.0.0.0", self.port))?
        .run()
        .await
    }

    /// Manually parses and processes a raw JSON update payload.
    ///
    /// Useful for integrating with existing web frameworks (Axum, Warp, Rocket)
    /// without using the built-in Actix-Web server runner.
    ///
    /// # Arguments
    /// * `raw_update` - The raw JSON string sent by Telegram.
    /// * `bot` - Instance of [`Bot`].
    /// * `handler` - Async callback to process the parsed update.
    pub async fn process_update<F, Fut>(
        raw_update: &str,
        bot: Bot,
        handler: F,
    ) -> Result<(), crate::Error>
    where
        F: Fn(Update, Bot) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let update: Update = serde_json::from_str(raw_update)?;
        handler(update, bot).await;
        Ok(())
    }
}
