use lago_client::LagoClient;
use lago_types::{
    error::LagoError,
    filters::subscription::SubscriptionFilters,
    models::{PaginationParams, SubscriptionStatus},
    requests::customer_usage::GetCustomerCurrentUsageRequest,
    requests::subscription::ListSubscriptionsRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: Find an active subscription and get its current usage
    println!("--- Example 1: Get current usage for an existing subscription ---");

    // First, find an active subscription
    let list_request = ListSubscriptionsRequest::new()
        .with_filters(SubscriptionFilters::new().with_status(SubscriptionStatus::Active))
        .with_pagination(PaginationParams::new().with_per_page(1));

    match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => {
            if let Some(subscription) = response.subscriptions.first() {
                println!(
                    "Found active subscription: {} (customer: {})",
                    subscription.external_id, subscription.external_customer_id
                );

                // Now get the current usage for this subscription
                let usage_request = GetCustomerCurrentUsageRequest::new(
                    subscription.external_customer_id.clone(),
                    subscription.external_id.clone(),
                );

                match client.get_customer_current_usage(usage_request).await {
                    Ok(usage_response) => {
                        let usage = &usage_response.customer_usage;
                        println!("\nCustomer Usage:");
                        println!("  From: {}", usage.from_datetime);
                        println!("  To: {}", usage.to_datetime);
                        println!("  Issuing Date: {}", usage.issuing_date);
                        println!("  Currency: {}", usage.currency);
                        println!("  Amount (cents): {}", usage.amount_cents);
                        println!("  Taxes (cents): {}", usage.taxes_amount_cents);
                        println!("  Total (cents): {}", usage.total_amount_cents);
                        if let Some(invoice_id) = &usage.lago_invoice_id {
                            println!("  Invoice ID: {}", invoice_id);
                        }

                        println!("\n  Charges Usage ({} charges):", usage.charges_usage.len());
                        for charge in &usage.charges_usage {
                            println!(
                                "    - Metric: {} ({})",
                                charge.billable_metric.name, charge.billable_metric.code
                            );
                            println!("      Units: {}", charge.units);
                            println!("      Events: {}", charge.events_count);
                            println!(
                                "      Amount: {} {}",
                                charge.amount_cents, charge.amount_currency
                            );
                            println!("      Charge Model: {}", charge.charge.charge_model);
                            if let Some(display_name) = &charge.charge.invoice_display_name {
                                println!("      Display Name: {}", display_name);
                            }
                        }
                    }
                    Err(LagoError::Api {
                        status: 404,
                        message,
                    }) => {
                        println!("Customer or subscription not found: {}", message);
                    }
                    Err(e) => {
                        println!("Error getting customer usage: {}", e);
                    }
                }
            } else {
                println!("No active subscriptions found in your Lago account");
            }
        }
        Err(e) => println!("Error listing subscriptions: {}", e),
    }

    // Example 2: Get usage without applying taxes
    println!("\n--- Example 2: Get usage without taxes for an existing subscription ---");

    let list_request = ListSubscriptionsRequest::new()
        .with_filters(SubscriptionFilters::new().with_status(SubscriptionStatus::Active))
        .with_pagination(PaginationParams::new().with_per_page(1));

    match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => {
            if let Some(subscription) = response.subscriptions.first() {
                let usage_request = GetCustomerCurrentUsageRequest::new(
                    subscription.external_customer_id.clone(),
                    subscription.external_id.clone(),
                )
                .with_apply_taxes(false);

                match client.get_customer_current_usage(usage_request).await {
                    Ok(usage_response) => {
                        let usage = &usage_response.customer_usage;
                        println!(
                            "Customer Usage for {} (without taxes):",
                            subscription.external_id
                        );
                        println!("  Amount (cents): {}", usage.amount_cents);
                        println!(
                            "  Taxes (cents): {} (should be 0)",
                            usage.taxes_amount_cents
                        );
                        println!("  Total (cents): {}", usage.total_amount_cents);
                    }
                    Err(LagoError::Api { status: 404, .. }) => {
                        println!("Customer or subscription not found");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            } else {
                println!("No active subscriptions found");
            }
        }
        Err(e) => println!("Error listing subscriptions: {}", e),
    }

    // Example 3: Get usage for multiple subscriptions of the same customer
    println!("\n--- Example 3: Get usage for all subscriptions of a customer ---");

    let list_request = ListSubscriptionsRequest::new()
        .with_filters(SubscriptionFilters::new().with_status(SubscriptionStatus::Active))
        .with_pagination(PaginationParams::new().with_per_page(10));

    match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => {
            if let Some(first_sub) = response.subscriptions.first() {
                let customer_id = &first_sub.external_customer_id;

                // Find all subscriptions for this customer
                let customer_subs: Vec<_> = response
                    .subscriptions
                    .iter()
                    .filter(|s| &s.external_customer_id == customer_id)
                    .collect();

                println!(
                    "Found {} active subscriptions for customer {}",
                    customer_subs.len(),
                    customer_id
                );

                for sub in customer_subs {
                    let usage_request = GetCustomerCurrentUsageRequest::new(
                        sub.external_customer_id.clone(),
                        sub.external_id.clone(),
                    );

                    match client.get_customer_current_usage(usage_request).await {
                        Ok(usage_response) => {
                            let usage = &usage_response.customer_usage;
                            println!(
                                "\n  Subscription: {} (plan: {})",
                                sub.external_id, sub.plan_code
                            );
                            println!("    Total: {} cents", usage.total_amount_cents);
                            println!("    Charges: {}", usage.charges_usage.len());
                        }
                        Err(e) => {
                            println!("  Error for subscription {}: {}", sub.external_id, e);
                        }
                    }
                }
            } else {
                println!("No active subscriptions found");
            }
        }
        Err(e) => println!("Error listing subscriptions: {}", e),
    }

    println!("\n--- Examples completed! ---");
    Ok(())
}
