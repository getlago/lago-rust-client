use lago_client::LagoClient;
use lago_types::{
    error::LagoError,
    requests::event::{CreateEventInput, CreateEventRequest, GetEventRequest},
};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

/// Helper function to create an event or fetch it if it already exists
async fn create_or_get_event(
    client: &LagoClient,
    event_input: CreateEventInput,
) -> Result<String, Box<dyn std::error::Error>> {
    let transaction_id = event_input.transaction_id.clone();
    let request = CreateEventRequest::new(event_input);

    match client.create_event(request).await {
        Ok(response) => {
            println!("Created event: {}", response.event.transaction_id);
            Ok(response.event.transaction_id)
        }
        Err(LagoError::Api { status: 422, .. }) => {
            // Event already exists, fetch it instead
            println!("Event {} already exists, fetching it...", transaction_id);
            let get_request = GetEventRequest::new(transaction_id.clone());
            let event = client.get_event(get_request).await?;
            println!(
                "Retrieved existing event: transaction_id={}, code={}",
                event.event.transaction_id, event.event.code
            );
            Ok(event.event.transaction_id)
        }
        Err(e) => Err(e.into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Generate unique transaction IDs using timestamp to avoid conflicts
    let unique_id = timestamp;

    // Example 1: Create a usage event for a customer (with unique ID)
    println!("\n--- Example 1: Create event for customer ---");
    let transaction_id_1 = format!("txn_customer_{}", unique_id);
    let event_input = CreateEventInput::for_customer(
        transaction_id_1.clone(),
        "customer_123".to_string(),
        "api_calls".to_string(),
    )
    .with_properties(json!({"calls": 150}))
    .with_timestamp(timestamp);

    create_or_get_event(&client, event_input).await?;

    // Example 2: Create a usage event for a subscription (with unique ID)
    println!("\n--- Example 2: Create event for subscription ---");
    let transaction_id_2 = format!("txn_subscription_{}", unique_id);
    let subscription_event = CreateEventInput::for_subscription(
        transaction_id_2,
        "subscription_456".to_string(),
        "storage_gb".to_string(),
    )
    .with_properties(json!({"gb": 50.5}));

    create_or_get_event(&client, subscription_event).await?;

    // Example 3: Create an event with precise total amount (with unique ID)
    println!("\n--- Example 3: Create event with precise amount ---");
    let transaction_id_3 = format!("txn_precise_{}", unique_id);
    let precise_event = CreateEventInput::for_customer(
        transaction_id_3,
        "customer_789".to_string(),
        "compute_hours".to_string(),
    )
    .with_precise_total_amount_cents(1234567);

    create_or_get_event(&client, precise_event).await?;

    // Example 4: Retrieve the first event we created
    println!("\n--- Example 4: Retrieve event by transaction ID ---");
    let get_request = GetEventRequest::new(transaction_id_1);
    match client.get_event(get_request).await {
        Ok(event) => println!(
            "Retrieved event: transaction_id={}, code={}, timestamp={}",
            event.event.transaction_id, event.event.code, event.event.timestamp
        ),
        Err(e) => println!("Failed to retrieve event: {}", e),
    }

    // Example 5: Demonstrate idempotency - creating same event twice
    println!("\n--- Example 5: Idempotency test (same transaction_id) ---");
    let idempotent_id = format!("txn_idempotent_{}", unique_id);

    // First creation
    let first_event = CreateEventInput::for_customer(
        idempotent_id.clone(),
        "customer_123".to_string(),
        "api_calls".to_string(),
    )
    .with_properties(json!({"calls": 100}));

    create_or_get_event(&client, first_event).await?;

    // Second creation with same transaction_id (should fetch existing)
    let second_event = CreateEventInput::for_customer(
        idempotent_id,
        "customer_123".to_string(),
        "api_calls".to_string(),
    )
    .with_properties(json!({"calls": 200})); // Different properties

    create_or_get_event(&client, second_event).await?;

    println!("\n--- All examples completed successfully! ---");
    Ok(())
}
