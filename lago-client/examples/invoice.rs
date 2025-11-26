use lago_client::LagoClient;
use lago_types::requests::invoice::{
    BillingTime, GetInvoiceRequest, InvoicePreviewCoupon, InvoicePreviewCustomer,
    InvoicePreviewInput, InvoicePreviewRequest, InvoicePreviewSubscriptions, ListInvoicesRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: List all invoices
    let list_request = ListInvoicesRequest::new();
    let invoices = client.list_invoices(Some(list_request)).await?;
    println!("Found {} invoices", invoices.invoices.len());

    // Example 2: Get a specific invoice by ID
    let get_request = GetInvoiceRequest::new("invoice_id_here".to_string());
    let invoice = client.get_invoice(get_request).await?;
    println!("Retrieved invoice: {}", invoice.invoice.number);

    // Example 3: Preview an invoice for an existing customer with a new subscription
    let preview_input = InvoicePreviewInput::for_customer("customer_123".to_string())
        .with_plan_code("startup".to_string())
        .with_billing_time(BillingTime::Calendar);

    let preview_request = InvoicePreviewRequest::new(preview_input);
    let preview = client.preview_invoice(preview_request).await?;
    println!(
        "Preview invoice total: {} cents",
        preview.invoice.total_amount_cents
    );

    // Example 4: Preview an invoice with inline customer details
    let customer = InvoicePreviewCustomer::new()
        .with_name("New Customer".to_string())
        .with_currency("USD".to_string())
        .with_address(
            "123 Main St".to_string(),
            None,
            Some("San Francisco".to_string()),
            Some("CA".to_string()),
            Some("US".to_string()),
        );

    let preview_input = InvoicePreviewInput::new(customer)
        .with_plan_code("hobby".to_string())
        .with_billing_time(BillingTime::Anniversary)
        .with_subscription_at("2024-01-01T00:00:00Z".to_string());

    let preview_request = InvoicePreviewRequest::new(preview_input);
    let preview = client.preview_invoice(preview_request).await?;
    println!(
        "Preview invoice for new customer: {} cents",
        preview.invoice.total_amount_cents
    );

    // Example 5: Preview an invoice with coupons
    let coupon = InvoicePreviewCoupon::new("DISCOUNT20".to_string())
        .with_name("20% Discount".to_string())
        .with_percentage("20".to_string());

    let preview_input = InvoicePreviewInput::for_customer("cust_123".to_string())
        .with_plan_code("hobby".to_string())
        .with_coupons(vec![coupon]);

    let preview_request = InvoicePreviewRequest::new(preview_input);
    let preview = client.preview_invoice(preview_request).await?;
    println!(
        "Preview with coupon: {} cents (coupons: {} cents)",
        preview.invoice.total_amount_cents, preview.invoice.coupons_amount_cents
    );

    // Example 6: Preview an invoice for existing subscriptions with plan upgrade
    let subscriptions = InvoicePreviewSubscriptions::new(vec!["sub_123".to_string()])
        .with_plan_code("in_advance".to_string());

    let preview_input =
        InvoicePreviewInput::for_customer("cust_123".to_string()).with_subscriptions(subscriptions);

    let preview_request = InvoicePreviewRequest::new(preview_input);
    let preview = client.preview_invoice(preview_request).await?;
    println!(
        "Preview for plan upgrade: {} cents",
        preview.invoice.total_amount_cents
    );

    Ok(())
}
