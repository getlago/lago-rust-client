use lago_client::LagoClient;
use lago_types::requests::customer_usage::GetCustomerCurrentUsageRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: Get customer current usage (basic)
    let request = GetCustomerCurrentUsageRequest::new(
        "customer_123".to_string(),
        "subscription_456".to_string(),
    );
    let usage = client.get_customer_current_usage(request).await?;
    println!("Customer usage for period:");
    println!(
        "  From: {} to {}",
        usage.customer_usage.from_datetime, usage.customer_usage.to_datetime
    );
    println!("  Currency: {}", usage.customer_usage.currency);
    println!("  Amount: {} cents", usage.customer_usage.amount_cents);
    println!("  Taxes: {} cents", usage.customer_usage.taxes_amount_cents);
    println!("  Total: {} cents", usage.customer_usage.total_amount_cents);

    // Example 2: Get customer current usage with taxes disabled
    let request_no_taxes = GetCustomerCurrentUsageRequest::new(
        "customer_123".to_string(),
        "subscription_456".to_string(),
    )
    .with_apply_taxes(false);
    let usage_no_taxes = client.get_customer_current_usage(request_no_taxes).await?;
    println!("\nCustomer usage without taxes applied:");
    println!(
        "  Total: {} cents",
        usage_no_taxes.customer_usage.total_amount_cents
    );

    // Example 3: Iterate through charges usage
    println!("\nCharges breakdown:");
    for charge in &usage.customer_usage.charges_usage {
        println!(
            "  - {} ({}): {} units, {} cents",
            charge.billable_metric.name,
            charge.billable_metric.code,
            charge.units,
            charge.amount_cents
        );
        println!("    Events count: {}", charge.events_count);
        println!("    Charge model: {}", charge.charge.charge_model);

        // Print grouped usage if present
        if !charge.grouped_usage.is_empty() {
            println!("    Grouped usage:");
            for group in &charge.grouped_usage {
                println!(
                    "      - {} units, {} cents ({} events)",
                    group.units, group.amount_cents, group.events_count
                );
            }
        }

        // Print filters if present
        if !charge.filters.is_empty() {
            println!("    Filters:");
            for filter in &charge.filters {
                println!(
                    "      - {:?}={:?}: {} units, {} cents",
                    filter.key, filter.value, filter.units, filter.amount_cents
                );
            }
        }
    }

    Ok(())
}
