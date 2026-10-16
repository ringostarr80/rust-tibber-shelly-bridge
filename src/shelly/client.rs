use super::error::ShellyError;
use super::types::ShellyRequest;

pub struct ShellyClient {
    api_token: String,
    api_url: String,
    client: reqwest::Client,
}

impl ShellyClient {
    pub fn new(api_token: String, api_url: String) -> Self {
        Self {
            api_token,
            api_url,
            // The API token is part of the URL, so never send it unencrypted
            client: reqwest::Client::builder()
                .https_only(true)
                .build()
                .expect("failed to build HTTP client"),
        }
    }

    pub async fn send_price(&self, price: f64) -> Result<(), ShellyError> {
        let url = format!("{}/{}", self.api_url, self.api_token);

        let request_body = ShellyRequest { price };

        // Strip the URL from errors so the API token does not end up in logs
        self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(reqwest::Error::without_url)?;

        Ok(())
    }
}
