use lago_client::LagoClient;
use lago_types::requests::payment::{ListCustomerPaymentsRequest, ListPaymentsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: List all payments
    let list_request = ListPaymentsRequest::new();
    let payments = client.list_payments(Some(list_request)).await?;
    println!("Found {} payments", payments.payments.len());

    for payment in &payments.payments {
        println!(
            "  Payment {}: {} {} - {:?}",
            payment.lago_id, payment.amount_cents, payment.amount_currency, payment.payment_status
        );
    }

    // Example 2: List payments for an existing customer (from first payment)
    if let Some(first_payment) = payments.payments.first() {
        let customer_id = &first_payment.external_customer_id;
        let customer_payments_request = ListCustomerPaymentsRequest::new(customer_id.clone());
        let customer_payments = client
            .list_customer_payments(customer_payments_request)
            .await?;
        println!(
            "Found {} payments for customer {}",
            customer_payments.payments.len(),
            customer_id
        );
    } else {
        println!("No payments found, skipping customer payments lookup");
    }

    // Example 3: List payments filtered by external customer ID (using existing customer)
    if let Some(first_payment) = payments.payments.first() {
        let customer_id = &first_payment.external_customer_id;
        let filtered_request =
            ListPaymentsRequest::new().with_external_customer_id(customer_id.clone());
        let filtered_payments = client.list_payments(Some(filtered_request)).await?;
        println!(
            "Found {} payments for {} (via filter)",
            filtered_payments.payments.len(),
            customer_id
        );
    }

    // Example 4: Create a manual payment (requires valid invoice_id)
    // Uncomment and update invoice_id to test:
    // let payment_input = CreatePaymentInput::new(
    //     "486b147a-02a1-4ccf-8603-f3541fc25e7a".to_string(), // invoice_id
    //     10000, // amount_cents (100.00)
    //     "manual-payment-001".to_string(),
    // )
    // .with_paid_at("2025-01-15".to_string());
    //
    // let create_request = CreatePaymentRequest::new(payment_input);
    // let created_payment = client.create_payment(create_request).await?;
    // println!(
    //     "Created payment: {} - {} {}",
    //     created_payment.payment.lago_id,
    //     created_payment.payment.amount_cents,
    //     created_payment.payment.amount_currency
    // );

    Ok(())
}
