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
lago-client = "0.1.4"
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
let request = ListInvoicesRequest::new();
let invoices = client.list_invoices(Some(request)).await?;

// Get a specific invoice
let request = GetInvoiceRequest::new("invoice-id".to_string());
let invoice = client.get_invoice(request).await?;
```

### Invoice Preview

Preview an invoice before creating it:

```rust
use lago_types::requests::invoice::{
    BillingTime, InvoicePreviewInput, InvoicePreviewRequest,
    InvoicePreviewCustomer, InvoicePreviewCoupon, InvoicePreviewSubscriptions,
};

// Preview for an existing customer with a new subscription
let preview_input = InvoicePreviewInput::for_customer("customer_123".to_string())
    .with_plan_code("startup".to_string())
    .with_billing_time(BillingTime::Calendar);

let request = InvoicePreviewRequest::new(preview_input);
let preview = client.preview_invoice(request).await?;
println!("Preview total: {} cents", preview.invoice.total_amount_cents);

// Preview with inline customer details
let customer = InvoicePreviewCustomer::new()
    .with_name("New Customer".to_string())
    .with_currency("USD".to_string());

let preview_input = InvoicePreviewInput::new(customer)
    .with_plan_code("enterprise".to_string())
    .with_subscription_at("2024-01-01T00:00:00Z".to_string());

let request = InvoicePreviewRequest::new(preview_input);
let preview = client.preview_invoice(request).await?;

// Preview with coupons
let coupon = InvoicePreviewCoupon::new("DISCOUNT20".to_string())
    .with_percentage("20".to_string());

let preview_input = InvoicePreviewInput::for_customer("customer_123".to_string())
    .with_plan_code("startup".to_string())
    .with_coupons(vec![coupon]);

let request = InvoicePreviewRequest::new(preview_input);
let preview = client.preview_invoice(request).await?;

// Preview for existing subscriptions with plan upgrade
let subscriptions = InvoicePreviewSubscriptions::new(vec!["sub_123".to_string()])
    .with_plan_code("enterprise".to_string());

let preview_input = InvoicePreviewInput::for_customer("customer_123".to_string())
    .with_subscriptions(subscriptions);

let request = InvoicePreviewRequest::new(preview_input);
let preview = client.preview_invoice(request).await?;
```

### Billable Metrics

```rust
use lago_types::{
    models::{BillableMetricAggregationType, BillableMetricFilter},
    requests::billable_metric::{CreateBillableMetricInput, CreateBillableMetricRequest},
};

// Create a billable metric
let metric = CreateBillableMetricInput::new(
    "Storage Usage".to_string(),
    "storage_gb".to_string(),
    BillableMetricAggregationType::SumAgg,
)
.with_description("Tracks storage usage".to_string())
.with_field_name("gb_used".to_string());

let request = CreateBillableMetricRequest::new(metric);
let created = client.create_billable_metric(request).await?;

// List billable metrics
let metrics = client.list_billable_metrics(None).await?;

// Get specific billable metric
let metric = client.get_billable_metric(
    GetBillableMetricRequest::new("storage_gb".to_string())
).await?;
```

### Customers

```rust
use lago_types::{
    models::{CustomerType, CustomerPaymentProvider},
    requests::customer::{CreateCustomerInput, CreateCustomerRequest},
};

// Create or Update a customer
let customer = CreateCustomerInput::new("customer_123".to_string())
    .with_name("Acme Corp".to_string())
    .with_email("billing@acme.com".to_string())
    .with_customer_type(CustomerType::Company)
    .with_currency("USD".to_string());

let request = CreateCustomerRequest::new(customer);
let created = client.create_customer(request).await?;

// List customers
let customers = client.list_customers(None).await?;

// Get specific customer
let customer = client.get_customer(
    GetCustomerRequest::new("customer_123".to_string())
).await?;

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
- `billable_metric.rs` - Billable metrics management
- `customer.rs` - Customers management operations
- `invoice.rs` - Invoice operations including preview

```bash
# Run the basic usage example
cargo run --example basic_usage

# Run the billable metrics example
cargo run --example billable_metric

# Run the customer management example
cargo run --example customer

# Run the invoice example
cargo run --example invoice
```

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