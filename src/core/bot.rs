use crate::client::HttpClient;
use crate::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Bot {
    pub(crate) client: Arc<HttpClient>,
}

impl Bot {
    pub fn new(token: String) -> Self {
        Self {
            client: Arc::new(HttpClient::new(token)),
        }
    }

    pub fn with_api_url(token: String, api_url: String) -> Self {
        Self {
            client: Arc::new(HttpClient::with_api_url(token, api_url)),
        }
    }

    pub async fn execute<M>(&self, method: M) -> Result<M::Response, Error>
    where
        M: crate::core::requests::Method + serde::Serialize,
    {
        self.client.post(M::NAME, &method).await
    }
}
