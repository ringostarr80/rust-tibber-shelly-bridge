use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ShellyError {
    RequestFailed(reqwest::Error),
    JsonParseFailed(serde_json::Error),
}

impl fmt::Display for ShellyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellyError::RequestFailed(err) => write!(f, "HTTP request failed: {}", err),
            ShellyError::JsonParseFailed(err) => write!(f, "JSON parsing failed: {}", err),
        }
    }
}

impl Error for ShellyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ShellyError::RequestFailed(err) => Some(err),
            ShellyError::JsonParseFailed(err) => Some(err),
        }
    }
}

impl From<reqwest::Error> for ShellyError {
    fn from(err: reqwest::Error) -> Self {
        ShellyError::RequestFailed(err)
    }
}

impl From<serde_json::Error> for ShellyError {
    fn from(err: serde_json::Error) -> Self {
        ShellyError::JsonParseFailed(err)
    }
}
