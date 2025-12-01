use lago_client::LagoClient;
use lago_types::{
    error::LagoError,
    filters::subscription::SubscriptionFilters,
    models::{PaginationParams, SubscriptionBillingTime, SubscriptionStatus},
    requests::subscription::{
        CreateSubscriptionInput, CreateSubscriptionRequest, DeleteSubscriptionRequest,
        GetSubscriptionRequest, ListCustomerSubscriptionsRequest, ListSubscriptionsRequest,
        UpdateSubscriptionInput, UpdateSubscriptionRequest,
    },
};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Generate unique IDs using timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Track the created subscription's external_id for use in Example 7
    let mut created_subscription_id: Option<String> = None;

    // Example 1: Create a subscription using existing customer and a compatible plan
    println!("--- Example 1: Create a subscription ---");

    // Find an existing subscription to get a customer and a plan with matching currency
    let list_request = ListSubscriptionsRequest::new()
        .with_filters(SubscriptionFilters::new().with_status(SubscriptionStatus::Active))
        .with_pagination(PaginationParams::new().with_per_page(1));

    let existing_sub_info = match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => response
            .subscriptions
            .first()
            .map(|s| (s.external_customer_id.clone(), s.plan_code.clone())),
        Err(_) => None,
    };

    if let Some((customer_id, plan_code)) = existing_sub_info {
        let external_subscription_id = format!("sub_{}", timestamp);
        println!("Using existing customer: {}", customer_id);
        println!(
            "Using plan from existing subscription: {} (ensures currency match)",
            plan_code
        );

        let create_input = CreateSubscriptionInput::new(customer_id.clone(), plan_code.clone())
            .with_external_id(external_subscription_id.clone())
            .with_name("My Subscription".to_string())
            .with_billing_time(SubscriptionBillingTime::Calendar);

        let create_request = CreateSubscriptionRequest::new(create_input);

        match client.create_subscription(create_request).await {
            Ok(response) => {
                println!("Created subscription:");
                println!("  Lago ID: {}", response.subscription.lago_id);
                println!("  External ID: {}", response.subscription.external_id);
                println!("  Plan Code: {}", response.subscription.plan_code);
                println!("  Status: {:?}", response.subscription.status);
                println!("  Billing Time: {:?}", response.subscription.billing_time);
                created_subscription_id = Some(response.subscription.external_id.clone());
            }
            Err(LagoError::Api {
                status: 422,
                message,
            }) => {
                println!(
                    "Subscription creation failed (validation error): {}",
                    message
                );
            }
            Err(e) => {
                println!("Error creating subscription: {}", e);
            }
        }
    } else {
        println!("Could not find existing subscription to get customer/plan. Skipping creation.");
        println!("Make sure you have at least one active subscription in your Lago account.");
    }

    // Example 2: Get a subscription by external ID (using the created one or find existing)
    println!("\n--- Example 2: Get a subscription ---");

    let subscription_to_get = if let Some(ref sub_id) = created_subscription_id {
        Some(sub_id.clone())
    } else {
        // Find an existing subscription
        let list_request = ListSubscriptionsRequest::new()
            .with_pagination(PaginationParams::new().with_per_page(1));
        match client.list_subscriptions(Some(list_request)).await {
            Ok(response) => response.subscriptions.first().map(|s| s.external_id.clone()),
            Err(_) => None,
        }
    };

    if let Some(external_subscription_id) = subscription_to_get {
        let get_request = GetSubscriptionRequest::new(external_subscription_id.clone());

        match client.get_subscription(get_request).await {
            Ok(response) => {
                println!("Retrieved subscription:");
                println!("  External ID: {}", response.subscription.external_id);
                println!("  Status: {:?}", response.subscription.status);
                println!(
                    "  Customer: {}",
                    response.subscription.external_customer_id
                );
                if let Some(name) = &response.subscription.name {
                    println!("  Name: {}", name);
                }
            }
            Err(LagoError::Api { status: 404, .. }) => {
                println!("Subscription not found");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    } else {
        println!("No subscription available to retrieve");
    }

    // Example 3: List all subscriptions
    println!("\n--- Example 3: List all subscriptions ---");
    let list_request = ListSubscriptionsRequest::new()
        .with_pagination(PaginationParams::new().with_per_page(10).with_page(1));

    match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => {
            println!("Found {} subscriptions", response.subscriptions.len());
            for sub in &response.subscriptions {
                println!(
                    "  - {} ({}): {:?}",
                    sub.external_id, sub.plan_code, sub.status
                );
            }
            println!(
                "Page {}/{} (total: {})",
                response.meta.current_page, response.meta.total_pages, response.meta.total_count
            );
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 4: List subscriptions with filters
    println!("\n--- Example 4: List active subscriptions ---");
    let filters = SubscriptionFilters::new().with_status(SubscriptionStatus::Active);

    let filtered_request = ListSubscriptionsRequest::new()
        .with_filters(filters)
        .with_pagination(PaginationParams::new().with_per_page(5));

    match client.list_subscriptions(Some(filtered_request)).await {
        Ok(response) => {
            println!(
                "Found {} active subscriptions",
                response.subscriptions.len()
            );
            for sub in &response.subscriptions {
                println!(
                    "  - {} (customer: {})",
                    sub.external_id, sub.external_customer_id
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 5: Fetch an existing subscription from an existing customer
    println!("\n--- Example 5: Fetch existing subscription from existing customer ---");

    // First, list all subscriptions to find an existing one
    let list_request = ListSubscriptionsRequest::new()
        .with_filters(SubscriptionFilters::new().with_status(SubscriptionStatus::Active))
        .with_pagination(PaginationParams::new().with_per_page(1));

    match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => {
            if let Some(existing_sub) = response.subscriptions.first() {
                println!("Found existing subscription: {}", existing_sub.external_id);
                println!("  Customer: {}", existing_sub.external_customer_id);

                // Now fetch the full subscription details
                let get_request = GetSubscriptionRequest::new(existing_sub.external_id.clone());
                match client.get_subscription(get_request).await {
                    Ok(sub_response) => {
                        let sub = &sub_response.subscription;
                        println!("\nFetched subscription details:");
                        println!("  Lago ID: {}", sub.lago_id);
                        println!("  External ID: {}", sub.external_id);
                        println!("  Customer ID: {}", sub.external_customer_id);
                        println!("  Plan Code: {}", sub.plan_code);
                        println!("  Status: {:?}", sub.status);
                        println!("  Billing Time: {:?}", sub.billing_time);
                        if let Some(name) = &sub.name {
                            println!("  Name: {}", name);
                        }
                        if let Some(started_at) = &sub.started_at {
                            println!("  Started At: {}", started_at);
                        }
                        if let Some(plan) = &sub.plan {
                            println!("  Plan Name: {}", plan.name);
                            println!(
                                "  Plan Amount: {} {}",
                                plan.amount_cents, plan.amount_currency
                            );
                        }
                    }
                    Err(e) => println!("Error fetching subscription: {}", e),
                }
            } else {
                println!("No existing subscriptions found");
            }
        }
        Err(e) => println!("Error listing subscriptions: {}", e),
    }

    // Example 5b: List subscriptions for an existing customer
    println!("\n--- Example 5b: List subscriptions for existing customer ---");

    // First find an existing customer with subscriptions
    let list_request =
        ListSubscriptionsRequest::new().with_pagination(PaginationParams::new().with_per_page(1));

    match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => {
            if let Some(existing_sub) = response.subscriptions.first() {
                let existing_customer_id = &existing_sub.external_customer_id;
                println!("Found existing customer: {}", existing_customer_id);

                let customer_subs_request =
                    ListCustomerSubscriptionsRequest::new(existing_customer_id.clone())
                        .with_pagination(PaginationParams::new().with_per_page(10));

                match client
                    .list_customer_subscriptions(customer_subs_request)
                    .await
                {
                    Ok(response) => {
                        println!(
                            "Customer {} has {} subscriptions",
                            existing_customer_id,
                            response.subscriptions.len()
                        );
                        for sub in &response.subscriptions {
                            println!(
                                "  - {} (plan: {}, status: {:?})",
                                sub.external_id, sub.plan_code, sub.status
                            );
                        }
                    }
                    Err(e) => println!("Error listing customer subscriptions: {}", e),
                }
            } else {
                println!("No existing subscriptions found to get customer from");
            }
        }
        Err(e) => println!("Error listing subscriptions: {}", e),
    }

    // Example 6: Update an existing subscription
    println!("\n--- Example 6: Update an existing subscription ---");

    // Find an existing subscription to update
    let list_request = ListSubscriptionsRequest::new()
        .with_filters(SubscriptionFilters::new().with_status(SubscriptionStatus::Active))
        .with_pagination(PaginationParams::new().with_per_page(1));

    match client.list_subscriptions(Some(list_request)).await {
        Ok(response) => {
            if let Some(existing_sub) = response.subscriptions.first() {
                println!(
                    "Found existing subscription to update: {}",
                    existing_sub.external_id
                );
                println!("  Current name: {:?}", existing_sub.name);

                let update_input =
                    UpdateSubscriptionInput::new().with_name(format!("Updated at {}", timestamp));

                let update_request =
                    UpdateSubscriptionRequest::new(existing_sub.external_id.clone(), update_input);

                match client.update_subscription(update_request).await {
                    Ok(response) => {
                        println!("Updated subscription:");
                        println!("  External ID: {}", response.subscription.external_id);
                        if let Some(name) = &response.subscription.name {
                            println!("  New Name: {}", name);
                        }
                    }
                    Err(e) => println!("Error updating subscription: {}", e),
                }
            } else {
                println!("No existing active subscriptions found to update");
            }
        }
        Err(e) => println!("Error listing subscriptions: {}", e),
    }

    // Example 7: Delete (terminate) the subscription created in Example 1
    println!("\n--- Example 7: Delete a subscription ---");

    if let Some(sub_id) = created_subscription_id {
        println!("Terminating subscription created in Example 1: {}", sub_id);
        let delete_request = DeleteSubscriptionRequest::new(sub_id);

        match client.delete_subscription(delete_request).await {
            Ok(response) => {
                println!("Terminated subscription:");
                println!("  External ID: {}", response.subscription.external_id);
                println!("  Status: {:?}", response.subscription.status);
            }
            Err(LagoError::Api { status: 404, .. }) => {
                println!("Subscription not found");
            }
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("No subscription was created in Example 1, skipping deletion");
    }

    println!("\n--- Example completed! ---");
    Ok(())
}
