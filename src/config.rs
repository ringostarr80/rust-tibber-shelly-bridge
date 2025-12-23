use std::env;

pub struct Config {
    pub tibber_access_token: String,
    pub tibber_api_url: String,
    pub shelly_api_token: String,
    pub shelly_api_url: String,
    pub max_retries: u8,
    pub retry_delay_seconds: u8,
}

impl Config {
    pub fn load() -> Self {
        let _ = dotenvy::dotenv();

        // Required: Tibber Access Token
        let tibber_access_token = env::var("TIBBER_ACCESS_TOKEN")
            .expect("TIBBER_ACCESS_TOKEN environment variable must be set");

        // Optional: Tibber API URL (default: official API endpoint)
        let tibber_api_url = env::var("TIBBER_API_URL")
            .unwrap_or_else(|_| "https://api.tibber.com/v1-beta/gql".to_string());

        // Required: Shelly API Token
        let shelly_api_token = env::var("SHELLY_API_TOKEN")
            .expect("SHELLY_API_TOKEN environment variable must be set");

        // Required: Shelly API URL
        let shelly_api_url =
            env::var("SHELLY_API_URL").expect("SHELLY_API_URL environment variable must be set");

        // Optional: Max Retries (default: 3)
        let max_retries = env::var("MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);

        // Optional: Retry Delay (default: 5)
        let retry_delay_seconds = env::var("RETRY_DELAY_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        Self {
            tibber_access_token,
            tibber_api_url,
            shelly_api_token,
            shelly_api_url,
            max_retries,
            retry_delay_seconds,
        }
    }
}
