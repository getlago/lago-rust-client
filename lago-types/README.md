# Lago Types

A comprehensive type library for the Lago billing system, providing Rust data structures for API requests, responses, and domain models.

## Overview

This crate contains all the type definitions needed to interact with the Lago billing API, including:

- **Models**: Core domain objects like `Customer`, `Invoice`, `UsageThreshold`
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
use lago_types::models::{InvoiceType, InvoiceStatus, InvoicePaymentStatus};

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
            .with_status(payment_status)
            .with_invoice_type(invoice_type)
    );

// Convert to query parameters
let params = request.to_query_params();
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