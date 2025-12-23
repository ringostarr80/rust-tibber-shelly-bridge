mod config;
mod retry;
mod shelly;
mod tibber;

use config::Config;
use retry::retry_with_backoff;
use shelly::client::ShellyClient;
use tibber::client::TibberClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = Config::load();

    let tibber_client = TibberClient::new(config.tibber_access_token, config.tibber_api_url);
    let tibber_response = retry_with_backoff(
        || tibber_client.get_current_price(),
        config.max_retries,
        config.retry_delay_seconds,
    )
    .await?;

    let (price, currency) = tibber_response.get_price();
    log::info!("Current price from Tibber: {} {}", price, currency);

    let shelly_client = ShellyClient::new(config.shelly_api_token, config.shelly_api_url);
    retry_with_backoff(
        || shelly_client.send_price(price),
        config.max_retries,
        config.retry_delay_seconds,
    )
    .await?;

    log::info!("Price successfully sent to Shelly");

    Ok(())
}
