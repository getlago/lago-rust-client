# Lago Client

A Rust client library for interacting with the [Lago](https://getlago.com) billing API.

## Features

- **Async/Await Support**: Built with `tokio` and `reqwest` for modern async Rust
- **Automatic Retries**: Configurable retry logic with exponential backoff
- **Multiple Regions**: Support for US, EU, and custom API endpoints
- **Flexible Configuration**: Environment variables or programmatic configuration
- **Type Safety**: Strongly typed requests and responses using `lago-types`
- **Authentication**: Secure API key-based authentication

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lago-client = "0.1.0"
```

## Quick Start

### Using Environment Variables

Set the required environment variables:

```bash
export LAGO_API_KEY="your-api-key"
export LAGO_REGION="us"  # or "eu" or custom URL
```

Then create a client:

```rust
use lago_client::LagoClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;
    
    // Use the client to make API calls
    let invoices = client.list_invoices(None).await?;
    println!("Found {} invoices", invoices.invoices.len());
    
    Ok(())
}
```

### Programmatic Configuration

```rust
use lago_client::{LagoClient, Config, Credentials, Region};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .credentials(Credentials::new("your-api-key"))
        .region(Region::Us)
        .timeout(Duration::from_secs(30))
        .build();
    
    let client = LagoClient::new(config);
    
    // Use the client
    let invoices = client.list_invoices(None).await?;
    
    Ok(())
}
```

## Configuration

### Regions

The client supports multiple regions:

- `Region::Us` - United States (default)
- `Region::Eu` - European Union
- `Region::Custom(url)` - Custom API endpoint

### Retry Configuration

Configure retry behavior for failed requests:

```rust
use lago_client::{Config, RetryConfig, RetryMode};
use std::time::Duration;

let retry_config = RetryConfig::builder()
    .mode(RetryMode::Standard)
    .max_attempts(3)
    .initial_delay(Duration::from_millis(100))
    .max_delay(Duration::from_secs(30))
    .backoff_multiplier(2.0)
    .build();

let config = Config::builder()
    .retry_config(retry_config)
    .build();
```

Retry modes:
- `RetryMode::Off` - No retries
- `RetryMode::Standard` - Standard exponential backoff
- `RetryMode::Adaptive` - Adaptive retry behavior

## API Operations

### Invoices

```rust
use lago_types::requests::invoice::{ListInvoicesRequest, GetInvoiceRequest};

// List invoices with optional filters
let request = ListInvoicesRequest::builder()
    .limit(10)
    .build();
let invoices = client.list_invoices(Some(request)).await?;

// Get a specific invoice
let request = GetInvoiceRequest::new("invoice-id");
let invoice = client.get_invoice(request).await?;
```

## Error Handling

The client uses the `lago-types` error system:

```rust
use lago_types::error::LagoError;

match client.list_invoices(None).await {
    Ok(invoices) => println!("Success: {} invoices", invoices.invoices.len()),
    Err(LagoError::Unauthorized) => println!("Invalid API key"),
    Err(LagoError::RateLimit) => println!("Rate limit exceeded"),
    Err(LagoError::Api { status, message }) => {
        println!("API error {}: {}", status, message);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `LAGO_API_KEY` | API key for authentication | Required |
| `LAGO_REGION` | API region (`us`, `eu`, or custom URL) | `us` |
| `LAGO_API_URL` | Custom API endpoint URL | - |

## Examples

See the `examples/` directory for complete usage examples:

- `basic_usage.rs` - Basic client usage
- `custom_configuration.rs` - Advanced configuration options

## Release

Before publishing a release 

```shell
cargo check
cargo test
cargo doc --no-deps --open
cargo package
```

Run the release 

```shell
cargo login API_KEY
cargo publish
```

## License

This project is licensed under the same license as the parent Lago Rust Client.