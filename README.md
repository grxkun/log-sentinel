# Log Sentinel Indexer

A modular, asynchronous Rust SDK for integrating with the Log Sentinel data platform. Designed for seamless data platform interactions, featuring clean trait-based architecture for database integration, historical backfill, creator management, and verification services.

## Features

- **Database Integration**: Connect to PostgreSQL databases with simulated client support.
- **Historical Backfill**: Automated backfill operations from configurable start points.
- **Configuration Management**: Load settings from environment variables and `.env` files using `dotenvy`.
- **Creator Access**: Fetch creator profiles and balances via API simulations.
- **Verification Services**: Check address verification status and retrieve verified addresses.
- **Asynchronous Operations**: Built on `tokio` for high-performance async handling.
- **Error Handling**: Comprehensive error management with `anyhow`.
- **Serialization**: JSON and chrono support with `serde` and `serde_json`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
log-sentinel = "0.1.0"
```

## Usage

Set up your environment variables:

```bash
export API_BASE_URL="https://api.logsentinel.com"
export API_KEY="your_api_key"
export DATABASE_URL="postgres://user:pass@localhost/db"
export BACKFILL_START_POINT="0"
```

Or create a `.env` file with the same variables.

Then, in your Rust code:

```rust
use log_sentinel::DataPlatformClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = DataPlatformClient::new()?;
    client.initialize(true).await?;

    // Access features
    let creator = client.creator().get_profile("creator_id").await?;
    let verified = client.verification().check_verification("address").await?;

    Ok(())
}
```

## License

This project is open source and available under the MIT License.