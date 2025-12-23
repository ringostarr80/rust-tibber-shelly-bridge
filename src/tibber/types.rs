use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum Currency {
    EUR,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::EUR => write!(f, "EUR"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub total: f64,
    pub currency: Currency,
}

#[derive(Debug, Deserialize)]
pub struct PriceInfo {
    pub current: Price,
}

#[derive(Debug, Deserialize)]
pub struct CurrentSubscription {
    #[serde(rename = "priceInfo")]
    pub price_info: PriceInfo,
}

#[derive(Debug, Deserialize)]
pub struct Home {
    #[serde(rename = "currentSubscription")]
    pub current_subscription: CurrentSubscription,
}

#[derive(Debug, Deserialize)]
pub struct Viewer {
    pub homes: Vec<Home>,
}

#[derive(Debug, Deserialize)]
pub struct TibberData {
    pub viewer: Viewer,
}

#[derive(Debug, Deserialize)]
pub struct TibberResponse {
    pub data: TibberData,
}

impl TibberResponse {
    pub fn get_price(&self) -> (f64, Currency) {
        if self.data.viewer.homes.is_empty() {
            panic!("No homes found in Tibber response");
        }

        let price_data = &self.data.viewer.homes[0]
            .current_subscription
            .price_info
            .current;

        (price_data.total, price_data.currency.clone())
    }
}
