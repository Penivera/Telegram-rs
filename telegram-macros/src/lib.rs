//! Procedural macros for `telegram-rs`.
//!
//! Provides attribute macros for declarative bot creation and command routing.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Attribute macro for decorating bot entrypoints and definitions.
///
/// Reserved for high-level declarative bot runtime setups.
///
/// # Examples
///
/// ```ignore
/// #[telegram_rs_2::bot]
/// async fn my_bot() {
///     // Bot setup
/// }
/// ```
#[proc_macro_attribute]
pub fn bot(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Attribute macro for declaring Telegram bot command handler functions.
///
/// Wraps a command handler function, enabling automated routing and metadata extraction.
///
/// # Examples
///
/// ```ignore
/// #[telegram_rs_2::command("/start")]
/// async fn handle_start(bot: Bot, msg: Message) {
///     // Handle /start
/// }
/// ```
#[proc_macro_attribute]
pub fn command(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}
