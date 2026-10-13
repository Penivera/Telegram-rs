//! Example: Telegram Mini App
//!
//! This example demonstrates how to set up a basic Telegram Mini App using telegram-rs.
//! It shows initData validation and basic Mini App utilities.
//!
//! # Setup
//! Run with mini-app feature: cargo run --example mini_app --features mini-app

#[cfg(feature = "mini-app")]
use telegram_rs_2::mini_app::InitData;

fn main() {
    #[cfg(feature = "mini-app")]
    {
        println!("Telegram Mini App Example");
        let raw_init_data = "query_id=sample_query_123&auth_date=1700000000&hash=mock_hash";
        match InitData::parse(raw_init_data) {
            Ok(init_data) => {
                println!(
                    "Parsed InitData: query_id={:?}, auth_date={}",
                    init_data.query_id, init_data.auth_date
                );
            }
            Err(e) => eprintln!("Failed to parse initData: {}", e),
        }
    }

    #[cfg(not(feature = "mini-app"))]
    {
        println!("Mini App example requires 'mini-app' feature");
        println!("Run with: cargo run --example mini_app --features mini-app");
    }
}
