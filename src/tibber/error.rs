use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TibberError {
    RequestFailed(reqwest::Error),
    JsonParseFailed(serde_json::Error),
    NoHomesFound,
}

impl fmt::Display for TibberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TibberError::RequestFailed(err) => write!(f, "HTTP request failed: {}", err),
            TibberError::JsonParseFailed(err) => write!(f, "JSON parsing failed: {}", err),
            TibberError::NoHomesFound => write!(f, "No homes found in response"),
        }
    }
}

impl Error for TibberError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TibberError::RequestFailed(err) => Some(err),
            TibberError::JsonParseFailed(err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for TibberError {
    fn from(err: reqwest::Error) -> Self {
        TibberError::RequestFailed(err)
    }
}

impl From<serde_json::Error> for TibberError {
    fn from(err: serde_json::Error) -> Self {
        TibberError::JsonParseFailed(err)
    }
}
