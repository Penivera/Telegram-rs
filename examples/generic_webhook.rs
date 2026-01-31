#[cfg(feature = "webhook")]
use hyper::service::{make_service_fn, service_fn};
#[cfg(feature = "webhook")]
use hyper::{Body, Request, Response, Server};
#[cfg(feature = "webhook")]
use std::convert::Infallible;
#[cfg(feature = "webhook")]
use telegram_rs_2::core::types::Update;
#[cfg(feature = "webhook")]
use telegram_rs_2::rt::webhook::Webhook;
#[cfg(feature = "webhook")]
use telegram_rs_2::{Bot, Result};

// This example uses Hyper directly to show how you can use any web server.
// You just need to get the body as a string and pass it to Webhook::process_update.

#[cfg(feature = "webhook")]
#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string());
    let bot = Bot::new(token);

    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc = make_service_fn(move |_conn| {
        let bot = bot.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let bot = bot.clone();
                async move {
                    if req.method() == hyper::Method::POST && req.uri().path() == "/webhook" {
                        let bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
                        let body_str = String::from_utf8(bytes.to_vec()).unwrap();

                        // Pass the raw update body to the library
                        let result = Webhook::process_update(
                            &body_str,
                            bot,
                            |update: Update, _bot: Bot| async move {
                                println!("Received generic update: {:?}", update);
                            },
                        )
                        .await;

                        match result {
                            Ok(_) => Ok::<_, Infallible>(Response::new(Body::from("OK"))),
                            Err(e) => {
                                eprintln!("Error processing update: {}", e);
                                Ok::<_, Infallible>(Response::new(Body::from("Error")))
                            }
                        }
                    } else {
                        Ok(Response::new(Body::from("Not Found")))
                    }
                }
            }))
        }
    });

    println!("Starting generic webhook server on http://{}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

#[cfg(not(feature = "webhook"))]
fn main() {
    println!("This example requires the 'webhook' feature.");
    println!("Run with: cargo run --example generic_webhook --features webhook");
}
