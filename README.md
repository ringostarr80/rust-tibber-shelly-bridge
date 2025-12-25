[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=ringostarr80_rust-tibber-shelly-bridge&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=ringostarr80_rust-tibber-shelly-bridge)

# rust-tibber-shelly-bridge

## What does it do?

This tool connects to the Tibber API, fetches the current electricity price for your home, and then sends it to the Shelly Cloud API. Handy if you want to control your Shelly devices based on current electricity prices!

## Features

- 🔄 Automatic retry logic for connection issues
- 📝 Proper logging
- ⚙️ Configurable via environment variables
- 🚀 Single-threaded async runtime for minimal overhead
- 🛡️ Clean error handling

## Installation

### Prerequisites

- Rust (best installed via [rustup](https://rustup.rs/))
- A Tibber account with API token
- Access to the Shelly Cloud API

### Build

```bash
git clone https://github.com/ringostarr80/rust-tibber-shelly-bridge.git
cd rust-tibber-shelly-bridge
cargo build --release
```

The binary will be located at `target/release/rust-tibber-shelly-bridge`.

## Configuration

The tool uses environment variables for configuration. The easiest way is to copy the `.env_sample` file to `.env` and edit it to your needs.

### Environment Variables in Detail

| Variable              | Required | Default | Description |
|-----------------------|----------|---------|-------------|
| `TIBBER_ACCESS_TOKEN` | ✅       | -       | Your Tibber API token |
| `TIBBER_API_URL`      | ❌        | `https://api.tibber.com/v1-beta/gql` | Tibber GraphQL endpoint |
| `SHELLY_API_TOKEN`    | ✅       | -       | Your Shelly API token |
| `SHELLY_API_URL`      | ✅       | -       | Shelly API endpoint |
| `MAX_RETRIES`         | ❌        | `3`     | Number of retry attempts |
| `RETRY_DELAY_SECONDS` | ❌        | `5`     | Wait time between retries |

## Usage

```bash
# With .env file
cargo run

# With custom log level
RUST_LOG=debug cargo run
```

## Example Output

```
[2025-12-23T21:27:32Z INFO  rust_tibber_shelly_bridge] Current price from Tibber: 0.2866 EUR
[2025-12-23T21:27:32Z INFO  rust_tibber_shelly_bridge] Price successfully sent to Shelly
```
