//! Asynchronous HTTP transport client for the Telegram Bot API.

use crate::Error;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

/// Internal asynchronous HTTP client managing requests to Telegram Bot API servers.
///
/// Wraps a [`reqwest::Client`] with authentication and URL building logic.
#[derive(Clone, Debug)]
pub struct HttpClient {
    client: Client,
    _token: String,
    api_url: String,
}

impl HttpClient {
    /// Creates a new `HttpClient` for the official Telegram Bot API endpoint.
    ///
    /// # Arguments
    /// * `token` - Telegram Bot token.
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            api_url: format!("https://api.telegram.org/bot{}/", token),
            _token: token,
        }
    }

    /// Creates a new `HttpClient` targeting a custom API base URL.
    ///
    /// # Arguments
    /// * `token` - Telegram Bot token.
    /// * `api_url` - Custom base URL including the bot token path.
    pub fn with_api_url(token: String, api_url: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
            _token: token,
        }
    }

    /// Performs an asynchronous HTTP POST request to a Telegram method endpoint.
    ///
    /// Serializes the provided `body` to JSON, executes the request, and validates the
    /// standard Telegram API JSON envelope `{ "ok": bool, "result": T, ... }`.
    ///
    /// # Arguments
    /// * `method` - Method name (e.g. `"sendMessage"`).
    /// * `body` - Serializable payload.
    pub async fn post<T: DeserializeOwned>(
        &self,
        method: &str,
        body: &impl serde::Serialize,
    ) -> Result<T, Error> {
        let url = format!("{}{}", self.api_url, method);
        let resp = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(Error::Network)?;

        self.handle_response(resp).await
    }

    /// Performs an asynchronous HTTP GET request to a Telegram method endpoint.
    ///
    /// # Arguments
    /// * `method` - Method name (e.g. `"getMe"`).
    pub async fn get<T: DeserializeOwned>(&self, method: &str) -> Result<T, Error> {
        let url = format!("{}{}", self.api_url, method);
        let resp = self.client.get(&url).send().await.map_err(Error::Network)?;

        self.handle_response(resp).await
    }

    /// Parses and validates the standard Telegram `{ "ok": true, "result": ... }` envelope.
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        let _status = response.status();
        let text = response.text().await.map_err(Error::Network)?;

        #[derive(serde::Deserialize)]
        struct ApiResponse<T> {
            ok: bool,
            result: Option<T>,
            description: Option<String>,
            error_code: Option<i32>,
        }

        let api_resp: ApiResponse<T> = serde_json::from_str(&text).map_err(Error::Serialization)?;

        if api_resp.ok {
            Ok(api_resp.result.ok_or_else(|| {
                Error::Other("Missing result field in successful response".to_string())
            })?)
        } else {
            Err(Error::Api {
                error_code: api_resp.error_code,
                description: api_resp
                    .description
                    .unwrap_or_else(|| "Unknown API error".to_string()),
            })
        }
    }
}
