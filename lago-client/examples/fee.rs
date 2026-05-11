use lago_client::LagoClient;
use lago_types::{
    filters::fee::FeeFilters,
    models::FeeType,
    requests::fee::{GetFeeRequest, ListFeesRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: list a few recent fees with default filters
    let list_request = ListFeesRequest::new();
    let fees = client.list_fees(Some(list_request)).await?;
    println!("Found {} fees", fees.fees.len());

    for fee in fees.fees.iter().take(5) {
        println!(
            "  Fee {}: {} {} | {:?} | {:?}",
            fee.lago_id, fee.amount_cents, fee.amount_currency, fee.payment_status, fee.item
        );
    }

    // Example 2: list only subscription fees (typical MRR base)
    let subscription_filters = FeeFilters::new().with_fee_type(FeeType::Subscription);
    let subscription_request = ListFeesRequest::new().with_filters(subscription_filters);
    let subscription_fees = client.list_fees(Some(subscription_request)).await?;
    println!(
        "\nFound {} subscription fees (recurring revenue)",
        subscription_fees.fees.len()
    );

    // Example 3: list usage charges for a specific billable metric (e.g., "seats")
    // — useful when MRR should include certain usage as recurring revenue.
    let seats_filters = FeeFilters::new()
        .with_fee_type(FeeType::Charge)
        .with_billable_metric_code("seats".to_string());
    let seats_request = ListFeesRequest::new().with_filters(seats_filters);
    let seats_fees = client.list_fees(Some(seats_request)).await?;
    println!(
        "\nFound {} charge fees for billable metric 'seats'",
        seats_fees.fees.len()
    );

    // Example 4: fetch a single fee by Lago ID (using first fee from the list, if any)
    if let Some(first) = fees.fees.first() {
        let get_request = GetFeeRequest::new(first.lago_id.to_string());
        let single = client.get_fee(get_request).await?;
        println!(
            "\nRetrieved fee {}: {} {}",
            single.fee.lago_id, single.fee.amount_cents, single.fee.amount_currency
        );
    } else {
        println!("\nNo fees found, skipping single-fee retrieval example");
    }

    Ok(())
}
