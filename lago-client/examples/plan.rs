use lago_client::LagoClient;
use lago_types::{
    error::LagoError,
    models::{ChargeModel, PaginationParams, PlanInterval},
    requests::plan::{
        CreatePlanChargeInput, CreatePlanInput, CreatePlanRequest, DeletePlanRequest,
        GetPlanRequest, ListPlansRequest, UpdatePlanInput, UpdatePlanRequest,
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

    // Configuration - update these to match your Lago setup
    let plan_code = format!("example_plan_{}", timestamp);

    // Track the created plan's code for use later
    let mut created_plan_code: Option<String> = None;

    // Example 1: List all plans
    println!("--- Example 1: List all plans ---");
    let list_request =
        ListPlansRequest::new().with_pagination(PaginationParams::new().with_per_page(10));

    match client.list_plans(Some(list_request)).await {
        Ok(response) => {
            println!("Found {} plans", response.plans.len());
            for plan in &response.plans {
                println!(
                    "  - {} ({}): {} {} / {:?}",
                    plan.name, plan.code, plan.amount_cents, plan.amount_currency, plan.interval
                );
            }
            println!(
                "Page {}/{} (total: {})",
                response.meta.current_page, response.meta.total_pages, response.meta.total_count
            );
        }
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Get a specific plan
    println!("\n--- Example 2: Get an existing plan ---");

    // First, list plans to find an existing one
    let list_request =
        ListPlansRequest::new().with_pagination(PaginationParams::new().with_per_page(1));

    match client.list_plans(Some(list_request)).await {
        Ok(response) => {
            if let Some(existing_plan) = response.plans.first() {
                println!("Found existing plan: {}", existing_plan.code);

                // Fetch full plan details
                let get_request = GetPlanRequest::new(existing_plan.code.clone());
                match client.get_plan(get_request).await {
                    Ok(plan_response) => {
                        let plan = &plan_response.plan;
                        println!("\nFetched plan details:");
                        println!("  Lago ID: {}", plan.lago_id);
                        println!("  Name: {}", plan.name);
                        println!("  Code: {}", plan.code);
                        println!("  Interval: {:?}", plan.interval);
                        println!("  Amount: {} {}", plan.amount_cents, plan.amount_currency);
                        println!("  Pay in Advance: {}", plan.pay_in_advance);
                        if let Some(trial) = plan.trial_period {
                            println!("  Trial Period: {} days", trial);
                        }
                        if let Some(charges) = &plan.charges {
                            println!("  Charges: {} charge(s)", charges.len());
                        }
                    }
                    Err(e) => println!("Error fetching plan: {}", e),
                }
            } else {
                println!("No existing plans found");
            }
        }
        Err(e) => println!("Error listing plans: {}", e),
    }

    // Example 3: Create a new plan
    println!("\n--- Example 3: Create a new plan ---");

    let create_input = CreatePlanInput::new(
        "Example Plan".to_string(),
        plan_code.clone(),
        PlanInterval::Monthly,
        9900, // $99.00
        "USD".to_string(),
    )
    .with_description("An example plan created via the Lago Rust client".to_string())
    .with_pay_in_advance(true)
    .with_trial_period(14.0);

    let create_request = CreatePlanRequest::new(create_input);

    match client.create_plan(create_request).await {
        Ok(response) => {
            println!("Created plan:");
            println!("  Lago ID: {}", response.plan.lago_id);
            println!("  Name: {}", response.plan.name);
            println!("  Code: {}", response.plan.code);
            println!("  Interval: {:?}", response.plan.interval);
            println!(
                "  Amount: {} {}",
                response.plan.amount_cents, response.plan.amount_currency
            );
            created_plan_code = Some(response.plan.code.clone());
        }
        Err(LagoError::Api {
            status: 422,
            message,
        }) => {
            println!("Plan creation failed (validation error): {}", message);
        }
        Err(e) => {
            println!("Error creating plan: {}", e);
        }
    }

    // Example 4: Create a plan with charges
    println!("\n--- Example 4: Create a plan with charges (if billable metric exists) ---");

    // First, check if we have any billable metrics
    match client.list_billable_metrics(None).await {
        Ok(metrics_response) => {
            if let Some(metric) = metrics_response.billable_metrics.first() {
                let plan_with_charges_code = format!("plan_with_charges_{}", timestamp);

                let charge =
                    CreatePlanChargeInput::new(metric.lago_id.to_string(), ChargeModel::Standard)
                        .with_invoiceable(true)
                        .with_pay_in_advance(false)
                        .with_properties(serde_json::json!({
                            "amount": "0.01"
                        }));

                let create_input = CreatePlanInput::new(
                    "Plan with Charges".to_string(),
                    plan_with_charges_code.clone(),
                    PlanInterval::Monthly,
                    4900, // $49.00 base
                    "USD".to_string(),
                )
                .with_description("A plan with usage-based charges".to_string())
                .with_charges(vec![charge]);

                let create_request = CreatePlanRequest::new(create_input);

                match client.create_plan(create_request).await {
                    Ok(response) => {
                        println!("Created plan with charges:");
                        println!("  Code: {}", response.plan.code);
                        if let Some(charges) = &response.plan.charges {
                            println!("  Charges: {} charge(s)", charges.len());
                        }
                        // Clean up - delete this plan
                        let delete_request = DeletePlanRequest::new(plan_with_charges_code);
                        let _ = client.delete_plan(delete_request).await;
                        println!("  (Plan deleted for cleanup)");
                    }
                    Err(e) => {
                        println!("Error creating plan with charges: {}", e);
                    }
                }
            } else {
                println!("No billable metrics found, skipping plan with charges example");
            }
        }
        Err(e) => {
            println!("Could not list billable metrics: {}", e);
        }
    }

    // Example 5: Update a plan
    println!("\n--- Example 5: Update a plan ---");

    if let Some(code) = &created_plan_code {
        let update_input = UpdatePlanInput::new()
            .with_name("Updated Example Plan".to_string())
            .with_description(format!("Updated at {}", timestamp))
            .with_amount_cents(12900); // $129.00

        let update_request = UpdatePlanRequest::new(code.clone(), update_input);

        match client.update_plan(update_request).await {
            Ok(response) => {
                println!("Updated plan:");
                println!("  Name: {}", response.plan.name);
                println!(
                    "  Amount: {} {}",
                    response.plan.amount_cents, response.plan.amount_currency
                );
                if let Some(desc) = &response.plan.description {
                    println!("  Description: {}", desc);
                }
            }
            Err(e) => println!("Error updating plan: {}", e),
        }
    } else {
        println!("No plan was created in Example 3, skipping update");
    }

    // Example 6: Delete a plan
    println!("\n--- Example 6: Delete a plan ---");

    if let Some(code) = created_plan_code {
        let delete_request = DeletePlanRequest::new(code.clone());

        match client.delete_plan(delete_request).await {
            Ok(response) => {
                println!("Deleted plan:");
                println!("  Code: {}", response.plan.code);
                println!("  Name: {}", response.plan.name);
            }
            Err(LagoError::Api { status: 404, .. }) => {
                println!("Plan not found");
            }
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("No plan was created, skipping deletion");
    }

    println!("\n--- Example completed! ---");
    Ok(())
}
