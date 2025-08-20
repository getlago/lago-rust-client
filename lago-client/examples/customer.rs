use lago_client::LagoClient;
use lago_types::{
    models::{CustomerType, CustomerFinalizeZeroAmountInvoice},
    requests::customer::{
        CreateCustomerInput, CreateCustomerRequest, CreateCustomerBillingConfiguration,
        CreateCustomerShippingAddress, CreateCustomerMetadata, GetCustomerRequest,
        ListCustomersRequest,
    },
    filters::customer::CustomerFilter,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: Create a new customer
    let new_customer = CreateCustomerInput::new("customer_123".to_string())
        .with_name("Acme Corporation".to_string())
        .with_email("billing@acme.com".to_string())
        .with_address(
            "123 Business St".to_string(),
            Some("Suite 100".to_string()),
            Some("San Francisco".to_string()),
            Some("US".to_string()),
            Some("CA".to_string()),
            Some("94105".to_string()),
        )
        .with_phone("+1-555-0123".to_string())
        .with_url("https://acme.com".to_string())
        .with_legal_info("Acme Corporation Inc.".to_string(), Some("123456789".to_string()))
        .with_currency("USD".to_string())
        .with_customer_type(CustomerType::Company)
        .with_finalize_zero_amount_invoice(CustomerFinalizeZeroAmountInvoice::Finalize)
        .with_net_payment_term(30)
        .with_billing_configuration(CreateCustomerBillingConfiguration {
            invoice_grace_period: Some(3),
            document_locale: Some("en".to_string()),
            payment_provider: None,
            payment_provider_code: None,
            provider_customer_id: None,
            sync_with_provider: None,
            sync: None,
            provider_payment_methods: None,
        })
        .with_shipping_address(CreateCustomerShippingAddress {
            address_line1: Some("456 Shipping Ave".to_string()),
            address_line2: Some("Floor 2".to_string()),
            city: Some("Oakland".to_string()),
            country: Some("US".to_string()),
            state: Some("CA".to_string()),
            zipcode: Some("94607".to_string()),
        })
        .with_metadata(vec![
            CreateCustomerMetadata {
                key: "industry".to_string(),
                value: "technology".to_string(),
                display_in_invoice: false,
            },
            CreateCustomerMetadata {
                key: "plan_type".to_string(),
                value: "enterprise".to_string(),
                display_in_invoice: true,
            },
        ]);

    let create_request = CreateCustomerRequest::new(new_customer);
    let created_customer = client.create_customer(create_request).await?;
    println!("Created customer: {:?}", created_customer.customer.external_id);

    // Example 2: List all customers
    let list_request = ListCustomersRequest::new();
    let customers = client.list_customers(Some(list_request)).await?;
    println!("Found {} customers", customers.customers.len());

    // Example 3: List customers with filters
    let filtered_request = ListCustomersRequest::new()
        .with_filters(
            CustomerFilter::new()
                .with_customer_id("customer_123".to_string())
        );
    let filtered_customers = client.list_customers(Some(filtered_request)).await?;
    println!("Found {} filtered customers", filtered_customers.customers.len());

    // Example 4: Get a specific customer by external ID
    let get_request = GetCustomerRequest::new("customer_123".to_string());
    let customer = client.get_customer(get_request).await?;
    println!("Retrieved customer: {}", customer.customer.external_id);

    // Example 5: Update an existing customer
    let updated_customer = CreateCustomerInput::new("customer_123".to_string())
        .with_name("Acme Corporation Ltd.".to_string()) 
        .with_email("accounts@acme.com".to_string())
        .with_phone("+1-555-0124".to_string());

    let update_request = CreateCustomerRequest::new(updated_customer);
    let updated = client.create_customer(update_request).await?;
    println!("Updated customer: {}", updated.customer.external_id);

    Ok(())
}
