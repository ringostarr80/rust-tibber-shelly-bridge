use std::future::Future;
use tokio::time::{Duration, sleep};

pub async fn retry_with_backoff<F, Fut, T, E>(
    operation: F,
    max_retries: u8,
    delay_seconds: u8,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    // 1. try
    let mut last_error = match operation().await {
        Ok(result) => return Ok(result),
        Err(e) => Some(e),
    };

    // retries
    for attempt in 1..=max_retries {
        log::warn!(
            "Retry attempt {}/{} after error: {}",
            attempt,
            max_retries,
            last_error.as_ref().unwrap()
        );

        sleep(Duration::from_secs(delay_seconds as u64)).await;

        last_error = match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => Some(e),
        };
    }

    // All retries failed, return last error
    Err(last_error.unwrap())
}
