# Lago Types

A comprehensive type library for the Lago billing system, providing Rust data structures for API requests, responses, and domain models.

## Overview

This crate contains all the type definitions needed to interact with the Lago billing API, including:

- **Models**: Core domain objects like `Customer`, `Invoice`, `Coupon`, `AppliedCoupon`, `ActivityLogObject`, `ApiLogObject`, `UsageThreshold`
- **Requests**: Structured request types for API operations
- **Responses**: Typed responses from API endpoints
- **Filters**: Query parameter builders for list operations
- **Error handling**: Common error types and API error responses

## Features

- **Type Safety**: Strongly typed structs and enums for all API interactions
- **Serialization**: Full serde support for JSON serialization/deserialization
- **String Parsing**: Built-in `FromStr` implementations for enums to parse from string representations
- **Filtering**: Composable filter builders for list queries
- **Pagination**: Built-in pagination support with metadata
- **Documentation**: Comprehensive documentation for all public APIs

## String to Enum Conversion

The library provides convenient `FromStr` implementations for all enums, allowing easy conversion from string representations to typed enum values. This is particularly useful when working with external APIs or user input.


### Usage Examples

```rust
use std::str::FromStr;
use lago_types::models::{InvoiceType};

// Using FromStr::from_str()
let invoice_type = InvoiceType::from_str("subscription").unwrap();
assert_eq!(invoice_type, InvoiceType::Subscription);

// Error handling for invalid values
let result: Result<InvoiceType, _> = "invalid_type".parse();
assert!(result.is_err());
```

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
lago-types = { path = "../lago-types" }
```

### Basic Example

```rust
use std::str::FromStr;
use lago_types::{
    models::{Customer, Invoice, InvoicePaymentStatus, InvoiceType},
    requests::ListInvoicesRequest,
    filters::InvoiceFilters,
    models::PaginationParams,
};

// Parse enum values from strings
let status: InvoiceStatus = "finalized".parse().unwrap();
let payment_status: InvoicePaymentStatus = "pending".parse().unwrap();
let invoice_type = InvoiceType::from_str("subscription").unwrap();

// Create a request to list invoices with filters
let request = ListInvoicesRequest::new()
    .with_pagination(
        PaginationParams::new()
            .with_page(1)
            .with_per_page(20)
    )
    .with_filters(
        InvoiceFilters::new()
            .with_customer_id("customer_123".to_string())
            .with_status(status)
            .with_payment_status(payment_status)
            .with_invoice_type(invoice_type)
    );

// Convert to query parameters
let params = request.to_query_params();
```

### Invoice Preview

Preview an invoice before creating it:

```rust
use lago_types::requests::invoice::{
    BillingTime, InvoicePreviewInput, InvoicePreviewRequest,
    InvoicePreviewCustomer, InvoicePreviewCoupon, InvoicePreviewSubscriptions,
};

// Preview for an existing customer with a new subscription
let preview = InvoicePreviewInput::for_customer("customer_123".to_string())
    .with_plan_code("startup".to_string())
    .with_billing_time(BillingTime::Calendar);

let request = InvoicePreviewRequest::new(preview);

// Preview with inline customer details
let customer = InvoicePreviewCustomer::new()
    .with_name("New Customer".to_string())
    .with_currency("USD".to_string());

let preview = InvoicePreviewInput::new(customer)
    .with_plan_code("enterprise".to_string())
    .with_subscription_at("2024-01-01T00:00:00Z".to_string());

// Preview with coupons
let coupon = InvoicePreviewCoupon::new("DISCOUNT20".to_string())
    .with_percentage("20".to_string());

let preview = InvoicePreviewInput::for_customer("customer_123".to_string())
    .with_plan_code("startup".to_string())
    .with_coupons(vec![coupon]);

// Preview for existing subscriptions with plan upgrade
let subscriptions = InvoicePreviewSubscriptions::new(vec!["sub_123".to_string()])
    .with_plan_code("enterprise".to_string());

let preview = InvoicePreviewInput::for_customer("customer_123".to_string())
    .with_subscriptions(subscriptions);


### Activity Logs

Fetch activity logs:

```rust
use std::str::FromStr;
use lago_types::{
    models::{ActivitySource, PaginationParams},
    requests::activity_log::{ListActivityLogsRequest, GetActivityLogRequest},
    filters::activity_log::ActivityLogFilters,
};

// Parse activity source from string
let source = ActivitySource::from_str("api").unwrap();

// Create a request to list activity logs with filters
let request = ListActivityLogsRequest::new()
    .with_pagination(
        PaginationParams::new()
            .with_page(1)
            .with_per_page(50)
    )
    .with_filters(
        ActivityLogFilters::new()
            .with_activity_types(vec!["invoice.created".to_string()])
            .with_activity_sources(vec![ActivitySource::Api, ActivitySource::Front])
            .with_user_emails(vec!["admin@example.com".to_string()])
            .with_resource_types(vec!["Invoice".to_string()])
            .with_date_range("2025-01-01".to_string(), "2025-01-31".to_string())
    );

// Get a specific activity log by activity ID
let get_request = GetActivityLogRequest::new("activity-uuid".to_string());
```

### API Logs

Fetch API logs:

```rust
use std::str::FromStr;
use lago_types::{
    models::{HttpMethod, HttpStatus, StatusOutcome, PaginationParams},
    requests::api_log::{ListApiLogsRequest, GetApiLogRequest},
    filters::api_log::ApiLogFilters,
};

// Parse HTTP method from string
let method = HttpMethod::from_str("post").unwrap();

// Create a request to list API logs with filters
let request = ListApiLogsRequest::new()
    .with_pagination(
        PaginationParams::new()
            .with_page(1)
            .with_per_page(50)
    )
    .with_filters(
        ApiLogFilters::new()
            .with_http_methods(vec![HttpMethod::Post, HttpMethod::Put])
            .with_http_statuses(vec![
                HttpStatus::Outcome(StatusOutcome::Failed),
                HttpStatus::Code(500)
            ])
            .with_api_version("v1".to_string())
            .with_date_range("2025-01-01".to_string(), "2025-01-31".to_string())
    );

// Get a specific API log by request ID
let get_request = GetApiLogRequest::new("request-uuid".to_string());
```

### Applied Coupons

Work with applied coupons:

```rust
use lago_types::{
    filters::applied_coupon::AppliedCouponFilter,
    models::{AppliedCouponStatus, AppliedCouponFrequency, PaginationParams},
    requests::applied_coupon::{ApplyCouponInput, ApplyCouponRequest, ListAppliedCouponsRequest},
};

// Create input to apply a coupon to a customer
let apply_input = ApplyCouponInput::new(
    "customer_123".to_string(),
    "WELCOME10".to_string()
);

// Apply with a fixed amount discount
let apply_input = ApplyCouponInput::new("customer_123".to_string(), "DISCOUNT50".to_string())
    .with_fixed_amount(5000, "USD".to_string())
    .with_frequency(AppliedCouponFrequency::Once);

// Apply with a percentage discount
let apply_input = ApplyCouponInput::new("customer_123".to_string(), "SAVE20".to_string())
    .with_percentage_rate("20".to_string())
    .with_frequency(AppliedCouponFrequency::Recurring)
    .with_frequency_duration(3); // Apply for 3 billing periods

let request = ApplyCouponRequest::new(apply_input);

// Create a request to list applied coupons with filters
let list_request = ListAppliedCouponsRequest::new()
    .with_pagination(
        PaginationParams::new()
            .with_page(1)
            .with_per_page(50)
    )
    .with_filters(
        AppliedCouponFilter::new()
            .with_status(AppliedCouponStatus::Active)
            .with_external_customer_id("customer_123".to_string())
            .with_coupon_codes(vec!["WELCOME10".to_string(), "SAVE20".to_string()])
    );

// Convert to query parameters
let params = list_request.to_query_params();
```

### Coupons

Work with coupon definitions:

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

// Create a fixed amount coupon with recurring application
let coupon = CreateCouponInput::fixed_amount(
    "Summer $50 Off".to_string(),
    "SUMMER50".to_string(),
    5000, // $50.00 in cents
    "USD".to_string(),
    CouponFrequency::Recurring,
    CouponExpiration::NoExpiration,
)
.with_frequency_duration(3) // Apply for 3 billing periods
.with_limited_plans(vec!["startup".to_string(), "professional".to_string()]);
let request = CreateCouponRequest::new(coupon);

// List coupons with pagination
let list_request = ListCouponsRequest::new()
    .with_pagination(
        PaginationParams::new()
            .with_page(1)
            .with_per_page(20)
    );
let params = list_request.to_query_params();

// Get a specific coupon
let get_request = GetCouponRequest::new("WELCOME10".to_string());

// Update an existing coupon
let update_input = UpdateCouponInput::new()
    .with_name("Welcome 15% Discount".to_string())
    .with_percentage_rate("15".to_string())
    .with_reusable(false);
let update_request = UpdateCouponRequest::new("WELCOME10".to_string(), update_input);

// Delete a coupon
let delete_request = DeleteCouponRequest::new("SUMMER50".to_string());
```

## Module Structure

- `models/` - Core domain models and data structures
- `requests/` - Request types for API operations
- `responses/` - Response types from API endpoints
- `filters/` - Filter builders for list operations
- `error.rs` - Error types and handling

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

This project is licensed under the MIT License - see the LICENSE file for details.