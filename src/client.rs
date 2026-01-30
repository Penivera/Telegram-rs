use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use crate::Error;

#[derive(Clone, Debug)]
pub struct HttpClient {
    client: Client,
    _token: String,
    api_url: String,
}

impl HttpClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            api_url: format!("https://api.telegram.org/bot{}/", token),
            _token: token,
        }
    }

    pub fn with_api_url(token: String, api_url: String) -> Self {
        Self {
            client: Client::new(),
            api_url,
            _token: token,
        }
    }

    pub async fn post<T: DeserializeOwned>(&self, method: &str, body: &impl serde::Serialize) -> Result<T, Error> {
        let url = format!("{}{}", self.api_url, method);
        let resp = self.client.post(&url)
            .json(body)
            .send()
            .await
            .map_err(Error::Network)?; // TODO: Better error conversion

        self.handle_response(resp).await
    }

    pub async fn get<T: DeserializeOwned>(&self, method: &str) -> Result<T, Error> {
        let url = format!("{}{}", self.api_url, method);
        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(Error::Network)?;

        self.handle_response(resp).await
    }

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
            Ok(api_resp.result.ok_or_else(|| Error::Other("Missing result field in successful response".to_string()))?)
        } else {
            Err(Error::Api {
                error_code: api_resp.error_code,
                description: api_resp.description.unwrap_or_else(|| "Unknown API error".to_string()),
            })
        }
    }
}
