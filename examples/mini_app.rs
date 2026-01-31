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
        println!("This example demonstrates Mini App setup with initData validation.");
        
        // TODO: Implement complete example
        println!("Full implementation coming soon...");
        
        // Example of what the full implementation might look like:
        // let raw_init_data = "query_id=123&user=...&auth_date=456&hash=abc";
        // match InitData::parse(raw_init_data) {
        //     Ok(init_data) => {
        //         println!("InitData parsed successfully");
        //         if let Ok(valid) = init_data.validate("bot_token") {
        //             println!("InitData is valid: {}", valid);
        //         }
        //     }
        //     Err(e) => eprintln!("Failed to parse initData: {}", e),
        // }
    }

    #[cfg(not(feature = "mini-app"))]
    {
        println!("Mini App example requires 'mini-app' feature");
        println!("Run with: cargo run --example mini_app --features mini-app");
    }
}
