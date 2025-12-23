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
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_price(&self, price: f64) -> Result<(), ShellyError> {
        let url = format!("{}/{}", self.api_url, self.api_token);

        let request_body = ShellyRequest { price };

        let response = self.client.post(&url).json(&request_body).send().await?;

        if !response.status().is_success() {
            return Err(ShellyError::RequestFailed(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            )));
        }

        Ok(())
    }
}
