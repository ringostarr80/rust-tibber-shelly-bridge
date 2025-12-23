use super::error::TibberError;
use super::types::TibberResponse;

pub struct TibberClient {
    access_token: String,
    api_url: String,
    client: reqwest::Client,
}

impl TibberClient {
    pub fn new(access_token: String, api_url: String) -> Self {
        Self {
            access_token,
            api_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_current_price(&self) -> Result<TibberResponse, TibberError> {
        let query = r#"{
  viewer {
    homes {
      currentSubscription{
        priceInfo{
          current{
            total
            currency
          }
        }
      }
    }
  }
}"#;

        let body = serde_json::json!({
            "query": query
        });

        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        let tibber_response: TibberResponse = serde_json::from_str(&response)?;

        if tibber_response.data.viewer.homes.is_empty() {
            return Err(TibberError::NoHomesFound);
        }

        Ok(tibber_response)
    }
}
