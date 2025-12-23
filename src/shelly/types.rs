use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ShellyRequest {
    pub price: f64,
}
