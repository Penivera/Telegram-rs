use crate::Bot;
use crate::core::types::Update;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Arc;

pub struct Webhook {
    bot: Bot,
    path: String,
    port: u16,
}

impl Webhook {
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            path: "/webhook".to_string(),
            port: 8080,
        }
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

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
            
            App::new()
                .app_data(web::Data::new(bot_clone))
                .route(&path, web::post().to(move |update: web::Json<Update>, bot_data: web::Data<Bot>| {
                    let bot = bot_data.get_ref().clone();
                    let handler = handler_clone.clone();
                    async move {
                        handler(update.into_inner(), bot).await;
                        HttpResponse::Ok().finish()
                    }
                }))
        })
        .bind(("0.0.0.0", self.port))?
        .run()
        .await
    }
}
