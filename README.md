# Lago Rust Client

:warning: This client is under development, it's not ready for production usage.

A Rust client library for interacting with the [Lago](https://www.getlago.com/) API. This library provides a type-safe, async interface for managing customers, subscriptions, invoices, and other billing operations.

## Overview

This repository contains two main crates:

### 🔧 `lago-types`
The types crate provides all the data structures, request/response models, and error types used by the Lago API. It includes:

- **Request Types**: Structures for API requests (create, update, list operations)
- **Response Types**: Structures for API responses with proper deserialization
- **Error Types**: Comprehensive error handling for different API scenarios
- **Shared Types**: Common data structures used across the API

### 🚀 `lago-client`
The client crate provides the main interface for interacting with the Lago API. It includes:

- **HTTP Client**: Async HTTP client with automatic retries and error handling
- **Authentication**: Bearer token authentication
- **Rate Limiting**: Built-in handling of API rate limits
- **Retry Logic**: Configurable retry strategies for failed requests
- **Query Modules**: Organized API operations (invoices, customers, subscriptions, etc.)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lago-client = "0.1.0"
lago-types = "0.1.2"
```

## Quick Start

### Basic Usage

```rust
use lago_client::LagoClient;
use lago_types::requests::invoice::ListInvoicesRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client from environment variables
    let client = LagoClient::from_env()?;
    
    // List invoices
    let request = ListInvoicesRequest::default();
    let invoices = client.list_invoices(Some(request)).await?;
    
    println!("Found {} invoices", invoices.invoices.len());
    
    Ok(())
}
```

### Custom Configuration

```rust
use lago_client::{LagoClient, Config, Region, Credentials, RetryConfig, RetryMode};
use std::time::Duration;

let config = Config::builder()
    .credentials(Credentials::new("your-api-key".to_string()))
    .region(Region::Us)
    .timeout(Duration::from_secs(30))
    .retry_config(
        RetryConfig::builder()
            .mode(RetryMode::Standard)
            .max_attempts(3)
            .initial_delay(Duration::from_millis(100))
            .max_delay(Duration::from_secs(10))
            .backoff_multiplier(2.0)
            .build()
    )
    .build();

let client = LagoClient::new(config);
```

## Configuration

### Environment Variables

The client can be configured using environment variables:

```bash
export LAGO_API_KEY="your-api-key"
export LAGO_REGION="us"  # or "eu"
export LAGO_TIMEOUT_SECS="30"
```

### Regions

The client supports multiple regions:

```rust
use lago_client::{Config, Region};

// US region
let config = Config::builder()
    .region(Region::Us)
    .build();

// EU region
let config = Config::builder()
    .region(Region::Eu)
    .build();

// Custom endpoint
let config = Config::builder()
    .region(Region::Custom("https://api.custom.com".to_string()))
    .build();
```

### Retry Configuration

Configure retry behavior for failed requests:

```rust
use lago_client::{RetryConfig, RetryMode};
use std::time::Duration;

let retry_config = RetryConfig::builder()
    .mode(RetryMode::Standard)      // Standard, Adaptive, or Off
    .max_attempts(5)                // Maximum retry attempts
    .initial_delay(Duration::from_millis(200))  // Initial delay
    .max_delay(Duration::from_secs(30))         // Maximum delay
    .backoff_multiplier(2.0)        // Exponential backoff multiplier
    .build();
```

## API Operations

### Invoices

```rust
use lago_types::requests::invoice::{ListInvoicesRequest, GetInvoiceRequest};

// List invoices with pagination
let request = ListInvoicesRequest::builder()
    .per_page(50)
    .page(1)
    .build();
let invoices = client.list_invoices(Some(request)).await?;

// Get a specific invoice
let request = GetInvoiceRequest::new("invoice-id".to_string());
let invoice = client.get_invoice(request).await?;
```

### Subscriptions

```rust
use lago_types::requests::subscription::{
    CreateSubscriptionInput, CreateSubscriptionRequest, GetSubscriptionRequest,
    ListSubscriptionsRequest, UpdateSubscriptionInput, UpdateSubscriptionRequest,
    DeleteSubscriptionRequest, ListCustomerSubscriptionsRequest,
};
use lago_types::models::{SubscriptionBillingTime, SubscriptionStatus};
use lago_types::filters::subscription::SubscriptionFilters;

// Create a subscription
let input = CreateSubscriptionInput::new(
    "customer_123".to_string(),
    "starter_plan".to_string(),
)
.with_external_id("sub_001".to_string())
.with_name("My Subscription".to_string())
.with_billing_time(SubscriptionBillingTime::Calendar);

let request = CreateSubscriptionRequest::new(input);
let subscription = client.create_subscription(request).await?;

// Get a subscription by external ID
let request = GetSubscriptionRequest::new("sub_001".to_string());
let subscription = client.get_subscription(request).await?;

// List all subscriptions with filters
let filters = SubscriptionFilters::new()
    .with_status(SubscriptionStatus::Active);
let request = ListSubscriptionsRequest::new()
    .with_filters(filters);
let subscriptions = client.list_subscriptions(Some(request)).await?;

// List a customer's subscriptions
let request = ListCustomerSubscriptionsRequest::new("customer_123".to_string());
let subscriptions = client.list_customer_subscriptions(request).await?;

// Update a subscription
let input = UpdateSubscriptionInput::new()
    .with_name("Updated Name".to_string());
let request = UpdateSubscriptionRequest::new("sub_001".to_string(), input);
let subscription = client.update_subscription(request).await?;

// Delete (terminate) a subscription
let request = DeleteSubscriptionRequest::new("sub_001".to_string());
let subscription = client.delete_subscription(request).await?;
```

### Plans

```rust
use lago_types::requests::plan::{
    CreatePlanInput, CreatePlanRequest, GetPlanRequest, ListPlansRequest,
    UpdatePlanInput, UpdatePlanRequest, DeletePlanRequest,
};
// Create a plan
let input = CreatePlanInput::builder()
    .code("starter_plan".to_string())
    .name("Starter Plan".to_string())
    .description(Some("A basic starter plan".to_string()))
    .amount_cents(1000)
    .currency("USD".to_string())
    .interval("month".to_string())
    .trial_period_days(Some(14))
    .build();
let request = CreatePlanRequest::new(input);
let plan = client.create_plan(request).await?;
// List plans with pagination
let request = ListPlansRequest::builder()
    .per_page(50)
    .page(1)
    .build();
let plans = client.list_plans(Some(request)).await?;
// Get a specific plan
let request = GetPlanRequest::new("starter_plan".to_string());
let plan = client.get_plan(request).await?;
// Update a plan
let input = UpdatePlanInput::builder()
    .name(Some("Starter Plan Updated".to_string()))
    .description(Some("Updated description".to_string()))
    .amount_cents(Some(1200))
    .build();
let request = UpdatePlanRequest::new("starter_plan".to_string(), input);
let plan = client.update_plan(request).await?;
// Delete a plan
let request = DeletePlanRequest::new("starter_plan".to_string());
client.delete_plan(request).await?;
```

### Customer Usage

```rust
use lago_types::requests::customer_usage::GetCustomerCurrentUsageRequest;

// Get customer current usage for a subscription
let request = GetCustomerCurrentUsageRequest::new(
    "customer_123".to_string(),
    "subscription_456".to_string(),
);
let usage = client.get_customer_current_usage(request).await?;

println!("Usage period: {} to {}", usage.customer_usage.from_datetime, usage.customer_usage.to_datetime);
println!("Total amount: {} cents", usage.customer_usage.total_amount_cents);

// Iterate through charges
for charge in &usage.customer_usage.charges_usage {
    println!("{}: {} units, {} cents",
        charge.billable_metric.name,
        charge.units,
        charge.amount_cents
    );
}

// Get usage without applying taxes
let request = GetCustomerCurrentUsageRequest::new(
    "customer_123".to_string(),
    "subscription_456".to_string(),
)
.with_apply_taxes(false);
let usage = client.get_customer_current_usage(request).await?;
```

### Payments

```rust
use lago_types::requests::payment::{
    CreatePaymentInput, CreatePaymentRequest, ListCustomerPaymentsRequest, ListPaymentsRequest,
};

// List all payments
let request = ListPaymentsRequest::new();
let payments = client.list_payments(Some(request)).await?;
println!("Found {} payments", payments.payments.len());

// List payments for a specific customer
let request = ListCustomerPaymentsRequest::new("customer_123".to_string());
let payments = client.list_customer_payments(request).await?;

// List payments filtered by external customer ID
let request = ListPaymentsRequest::new()
    .with_external_customer_id("customer_123".to_string());
let payments = client.list_payments(Some(request)).await?;

// Create a manual payment
let input = CreatePaymentInput::new(
    "invoice_id_here".to_string(),
    10000, // amount in cents
    "payment-ref-001".to_string(),
)
.with_paid_at("2025-01-15".to_string());

let request = CreatePaymentRequest::new(input);
let payment = client.create_payment(request).await?;
println!("Created payment: {}", payment.payment.lago_id);
```

## Error Handling

The client provides comprehensive error handling:

```rust
use lago_types::error::LagoError;

match client.list_invoices(None).await {
    Ok(invoices) => {
        println!("Success: {} invoices", invoices.invoices.len());
    }
    Err(LagoError::Unauthorized) => {
        println!("Invalid API key");
    }
    Err(LagoError::RateLimit) => {
        println!("Rate limit exceeded");
    }
    Err(LagoError::Api { status, message }) => {
        println!("API error {}: {}", status, message);
    }
    Err(LagoError::Http(e)) => {
        println!("HTTP error: {}", e);
    }
    Err(LagoError::Serialization(e)) => {
        println!("Serialization error: {}", e);
    }
    Err(LagoError::Configuration(e)) => {
        println!("Configuration error: {}", e);
    }
}
```

## Features

### ✅ Implemented Features

- **Async/Await Support**: Built with Tokio for async operations
- **Automatic Retries**: Configurable retry logic with exponential backoff
- **Rate Limit Handling**: Automatic handling of API rate limits
- **Type Safety**: Full type safety with serde serialization/deserialization
- **Error Handling**: Comprehensive error types and handling
- **Authentication**: Bearer token authentication
- **Timeout Support**: Configurable request timeouts
- **Multiple Regions**: Support for US, EU, and custom endpoints

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Building

```bash
# Build the workspace
cargo build

# Build with release optimizations
cargo build --release
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for your changes
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [API Documentation](https://doc.getlago.com/)
- **Issues**: [GitHub Issues](https://github.com/getlago/lago-rust-client/issues)
- **Community**: [Lago Community](https://www.getlago.com/community)