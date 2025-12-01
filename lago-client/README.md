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

### Activity Logs

```rust
use lago_types::{
    filters::activity_log::ActivityLogFilters,
    models::ActivitySource,
    requests::activity_log::{GetActivityLogRequest, ListActivityLogsRequest},
};

// List all activity logs
let activity_logs = client.list_activity_logs(None).await?;

// List activity logs with filters
let request = ListActivityLogsRequest::new().with_filters(
    ActivityLogFilters::new()
        .with_activity_types(vec!["invoice.created".to_string()])
        .with_activity_sources(vec![ActivitySource::Api, ActivitySource::Front])
        .with_user_emails(vec!["admin@example.com".to_string()])
        .with_resource_types(vec!["Invoice".to_string()])
        .with_date_range("2025-01-01".to_string(), "2025-01-31".to_string()),
);
let filtered_logs = client.list_activity_logs(Some(request)).await?;

// Get a specific activity log by activity ID
let request = GetActivityLogRequest::new("activity-id".to_string());
let activity_log = client.get_activity_log(request).await?;
println!("Activity: {} - {:?}",
    activity_log.activity_log.activity_type,
    activity_log.activity_log.activity_source
);
```

### API Logs

```rust
use lago_types::{
    filters::api_log::ApiLogFilters,
    models::{HttpMethod, HttpStatus, StatusOutcome},
    requests::api_log::{GetApiLogRequest, ListApiLogsRequest},
};

// List all API logs
let api_logs = client.list_api_logs(None).await?;

// List API logs with filters
let request = ListApiLogsRequest::new().with_filters(
    ApiLogFilters::new()
        .with_http_methods(vec![HttpMethod::Post, HttpMethod::Put])
        .with_http_statuses(vec![HttpStatus::Outcome(StatusOutcome::Failed)])
        .with_api_version("v1".to_string())
        .with_request_paths(vec!["/invoices".to_string(), "/customers".to_string()])
        .with_date_range("2025-01-01".to_string(), "2025-01-31".to_string()),
);
let filtered_logs = client.list_api_logs(Some(request)).await?;

// Get a specific API log by request ID
let request = GetApiLogRequest::new("request-id".to_string());
let api_log = client.get_api_log(request).await?;
println!("Request: {:?} {} - Status {}",
    api_log.api_log.http_method,
    api_log.api_log.request_path,
    api_log.api_log.http_status
);
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
```

### Applied Coupons

```rust
use lago_types::{
    filters::applied_coupon::AppliedCouponFilter,
    models::{AppliedCouponStatus, PaginationParams},
    requests::applied_coupon::{ApplyCouponInput, ApplyCouponRequest, ListAppliedCouponsRequest},
};

// Apply a coupon to a customer
let apply_input = ApplyCouponInput::new(
    "customer_123".to_string(),
    "WELCOME10".to_string()
);
let request = ApplyCouponRequest::new(apply_input);
let applied = client.apply_coupon(request).await?;

// Apply with a fixed amount discount
let apply_input = ApplyCouponInput::new("customer_123".to_string(), "DISCOUNT50".to_string())
    .with_fixed_amount(5000, "USD".to_string()); // $50.00 discount
let request = ApplyCouponRequest::new(apply_input);
let applied = client.apply_coupon(request).await?;

// Apply with a percentage discount
let apply_input = ApplyCouponInput::new("customer_123".to_string(), "SAVE20".to_string())
    .with_percentage_rate("20".to_string()); // 20% discount
let request = ApplyCouponRequest::new(apply_input);
let applied = client.apply_coupon(request).await?;

// List all applied coupons
let applied_coupons = client.list_applied_coupons(None).await?;

// List with filters
let request = ListAppliedCouponsRequest::new()
    .with_pagination(PaginationParams::default().with_page(1).with_per_page(20))
    .with_filters(
        AppliedCouponFilter::new()
            .with_status(AppliedCouponStatus::Active)
            .with_external_customer_id("customer_123".to_string())
            .with_coupon_codes(vec!["WELCOME10".to_string()])
    );
let filtered = client.list_applied_coupons(Some(request)).await?;
```

### Coupons

```rust
use lago_types::{
    models::{CouponExpiration, CouponFrequency, PaginationParams},
    requests::coupon::{
        CreateCouponInput, CreateCouponRequest, DeleteCouponRequest,
        GetCouponRequest, ListCouponsRequest, UpdateCouponInput, UpdateCouponRequest,
    },
};

// Create a percentage-based coupon
let coupon = CreateCouponInput::percentage(
    "Welcome 10% Discount".to_string(),
    "WELCOME10".to_string(),
    "10".to_string(),
    CouponFrequency::Once,
    CouponExpiration::NoExpiration,
)
.with_reusable(true);
let request = CreateCouponRequest::new(coupon);
let created = client.create_coupon(request).await?;

// Create a fixed amount coupon
let coupon = CreateCouponInput::fixed_amount(
    "Summer $50 Off".to_string(),
    "SUMMER50".to_string(),
    5000, // $50.00 in cents
    "USD".to_string(),
    CouponFrequency::Recurring,
    CouponExpiration::NoExpiration,
)
.with_frequency_duration(3);
let request = CreateCouponRequest::new(coupon);
let created = client.create_coupon(request).await?;

// List all coupons
let coupons = client.list_coupons(None).await?;

// List coupons with pagination
let request = ListCouponsRequest::new()
    .with_pagination(PaginationParams::default().with_per_page(20));
let coupons = client.list_coupons(Some(request)).await?;

// Get a specific coupon
let request = GetCouponRequest::new("WELCOME10".to_string());
let coupon = client.get_coupon(request).await?;

// Update a coupon
let update_input = UpdateCouponInput::new()
    .with_name("Welcome 15% Discount".to_string())
    .with_percentage_rate("15".to_string());
let request = UpdateCouponRequest::new("WELCOME10".to_string(), update_input);
let updated = client.update_coupon(request).await?;

// Delete a coupon
let request = DeleteCouponRequest::new("SUMMER50".to_string());
let deleted = client.delete_coupon(request).await?;
```

### Events

```rust
use lago_types::requests::event::{CreateEventInput, CreateEventRequest, GetEventRequest};
use serde_json::json;

// Create a usage event for a customer
let event_input = CreateEventInput::for_customer(
    "transaction_123".to_string(),
    "customer_456".to_string(),
    "api_calls".to_string(),
)
.with_properties(json!({"calls": 150}))
.with_timestamp(1705312200);

let request = CreateEventRequest::new(event_input);
let created = client.create_event(request).await?;
println!("Created event: {}", created.event.transaction_id);

// Create a usage event for a subscription
let event_input = CreateEventInput::for_subscription(
    "transaction_456".to_string(),
    "subscription_789".to_string(),
    "storage_gb".to_string(),
)
.with_properties(json!({"gb": 50.5}))
.with_precise_total_amount_cents(1234567);

let request = CreateEventRequest::new(event_input);
let created = client.create_event(request).await?;

// Get a specific event by transaction ID
let request = GetEventRequest::new("transaction_123".to_string());
let event = client.get_event(request).await?;
println!("Event code: {}, timestamp: {}", event.event.code, event.event.timestamp);
```

### Plans

```rust
use lago_types::{
    models::{ChargeModel, PaginationParams, PlanInterval},
    requests::plan::{
        CreatePlanChargeInput, CreatePlanInput, CreatePlanRequest, DeletePlanRequest,
        GetPlanRequest, ListPlansRequest, UpdatePlanInput, UpdatePlanRequest,
    },
};

// List all plans
let plans = client.list_plans(None).await?;

// List plans with pagination
let request = ListPlansRequest::new()
    .with_pagination(PaginationParams::default().with_per_page(20));
let plans = client.list_plans(Some(request)).await?;

// Get a specific plan by code
let request = GetPlanRequest::new("starter_plan".to_string());
let plan = client.get_plan(request).await?;

// Create a basic plan
let plan_input = CreatePlanInput::new(
    "Starter Plan".to_string(),
    "starter_plan".to_string(),
    PlanInterval::Monthly,
    9900, // $99.00 in cents
    "USD".to_string(),
)
.with_description("Our starter plan".to_string())
.with_pay_in_advance(true)
.with_trial_period(14.0);

let request = CreatePlanRequest::new(plan_input);
let created = client.create_plan(request).await?;

// Create a plan with usage-based charges
let charge = CreatePlanChargeInput::new(
    "billable_metric_lago_id".to_string(),
    ChargeModel::Standard,
)
.with_invoiceable(true)
.with_properties(serde_json::json!({"amount": "0.01"}));

let plan_input = CreatePlanInput::new(
    "Usage Plan".to_string(),
    "usage_plan".to_string(),
    PlanInterval::Monthly,
    4900,
    "USD".to_string(),
)
.with_charges(vec![charge]);

let request = CreatePlanRequest::new(plan_input);
let created = client.create_plan(request).await?;

// Update a plan
let update_input = UpdatePlanInput::new()
    .with_name("Updated Starter Plan".to_string())
    .with_amount_cents(12900);

let request = UpdatePlanRequest::new("starter_plan".to_string(), update_input);
let updated = client.update_plan(request).await?;

// Delete a plan
let request = DeletePlanRequest::new("starter_plan".to_string());
let deleted = client.delete_plan(request).await?;
```

### Credit Notes

```rust
use lago_types::{
    filters::credit_note::CreditNoteFilter,
    models::{CreditNoteReason, CreditNoteRefundStatus, PaginationParams},
    requests::credit_note::{
        CreateCreditNoteInput, CreateCreditNoteItemInput, CreateCreditNoteRequest,
        GetCreditNoteRequest, ListCreditNotesRequest, UpdateCreditNoteInput, UpdateCreditNoteRequest,
    },
};

// List all credit notes
let credit_notes = client.list_credit_notes(None).await?;

// List credit notes with filters
let request = ListCreditNotesRequest::new()
    .with_pagination(PaginationParams::default().with_page(1).with_per_page(20))
    .with_filters(
        CreditNoteFilter::new()
            .with_external_customer_id("customer_123".to_string())
            .with_reason(CreditNoteReason::Other)
            .with_date_range("2024-01-01".to_string(), "2024-12-31".to_string())
    );
let filtered = client.list_credit_notes(Some(request)).await?;

// Get a specific credit note
let request = GetCreditNoteRequest::new("credit-note-lago-id".to_string());
let credit_note = client.get_credit_note(request).await?;

// Create a credit note
let items = vec![
    CreateCreditNoteItemInput::new("fee_lago_id".to_string(), 1000),
];
let input = CreateCreditNoteInput::new(
    "invoice_lago_id".to_string(),
    CreditNoteReason::Other,
    1000, // credit_amount_cents
    0,    // refund_amount_cents
    items,
)
.with_description("Credit for billing adjustment".to_string());
let request = CreateCreditNoteRequest::new(input);
let created = client.create_credit_note(request).await?;

// Update a credit note's refund status
let update_input = UpdateCreditNoteInput::new()
    .with_refund_status(CreditNoteRefundStatus::Succeeded);
let request = UpdateCreditNoteRequest::new("credit-note-lago-id".to_string(), update_input);
let updated = client.update_credit_note(request).await?;
```

### Customer Usage

```rust
use lago_types::requests::customer_usage::GetCustomerCurrentUsageRequest;

// Get current usage for a customer's subscription
let request = GetCustomerCurrentUsageRequest::new(
    "customer_123".to_string(),
    "subscription_456".to_string(),
);
let usage = client.get_customer_current_usage(request).await?;
println!("Total amount: {} cents", usage.customer_usage.total_amount_cents);
println!("Charges: {:?}", usage.customer_usage.charges_usage.len());

// Get usage without applying taxes
let request = GetCustomerCurrentUsageRequest::new(
    "customer_123".to_string(),
    "subscription_456".to_string(),
)
.with_apply_taxes(false);
let usage = client.get_customer_current_usage(request).await?;
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
- `activity_log.rs` - Activity logs listing and filtering
- `api_log.rs` - API logs listing and filtering
- `billable_metric.rs` - Billable metrics management
- `customer.rs` - Customers management operations
- `invoice.rs` - Invoice operations including preview
- `applied_coupon.rs` - Applied coupons listing and filtering
- `coupon.rs` - Coupon CRUD operations
- `event.rs` - Event creation and retrieval
- `credit_note.rs` - Credit note operations
- `customer_usage.rs` - Customer usage retrieval
- `plan.rs` - Plan CRUD operations
- `subscription.rs` - Subscription CRUD operations

```bash
# Run the basic usage example
cargo run --example basic_usage

# Run the activity logs example
cargo run --example activity_log

# Run the API logs example
cargo run --example api_log

# Run the billable metrics example
cargo run --example billable_metric

# Run the customer management example
cargo run --example customer

# Run the invoice example
cargo run --example invoice

# Run the applied coupons example
cargo run --example applied_coupon

# Run the coupons example
cargo run --example coupon

# Run the events example
cargo run --example event

# Run the credit notes example
cargo run --example credit_note

# Run the customer usage example
cargo run --example customer_usage

# Run the plans example
cargo run --example plan

# Run the subscriptions example
cargo run --example subscription
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