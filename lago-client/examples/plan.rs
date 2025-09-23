use lago_client::LagoClient;
use lago_types::{
    models::{ChargeModel, PlanInterval},
    requests::plan::{
        CreateChargeInput, CreateChargeProperties, CreatePlanInput, CreatePlanRequest,
        GetPlanRequest, ListPlansRequest, UpdatePlanInput, UpdatePlanRequest,
    },
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Lago client
    let client = LagoClient::from_env().expect("Failed to initialize Lago client");

    // Example 1: List all plans
    println!("=== Listing Plans ===");
    let list_request = ListPlansRequest::new();
    let plans = client.list_plans(Some(list_request)).await?;

    println!("Found {} plans", plans.plans.len());
    for plan in &plans.plans {
        println!(
            "- {} ({}): {} {}",
            plan.name, plan.code, plan.amount_cents, plan.amount_currency
        );
    }

    // Example 2: Create a new plan
    println!("\n=== Creating a New Plan ===");

    // Create a standard charge for API calls
    let charge_properties = CreateChargeProperties::Standard {
        amount: "0.10".to_string(), // 10 cents per API call
    };

    let api_charge = CreateChargeInput::new(
        Uuid::new_v4(), // You would use a real billable metric ID here
        ChargeModel::Standard,
        charge_properties,
    )
    .with_invoice_display_name("API Calls".to_string())
    .with_invoiceable(true);

    let plan_input = CreatePlanInput::new(
        "Premium API Plan".to_string(),
        "premium_api_v1".to_string(),
        PlanInterval::Monthly,
        2999, // $29.99 base fee
        "USD".to_string(),
    )
    .with_description("Premium API access with usage-based billing".to_string())
    .with_trial_period(14.0) // 14-day trial
    .with_pay_in_advance(true)
    .with_invoiceable(true)
    .with_charges(vec![api_charge]);

    let create_request = CreatePlanRequest::new(plan_input);

    match client.create_plan(create_request).await {
        Ok(response) => {
            println!(
                "Created plan: {} (ID: {})",
                response.plan.name, response.plan.lago_id
            );
        }
        Err(e) => {
            println!("Failed to create plan: {}", e);
        }
    }

    // Example 3: Get a specific plan
    println!("\n=== Getting a Specific Plan ===");
    let get_request = GetPlanRequest::new("premium_api_v1".to_string());

    match client.get_plan(get_request).await {
        Ok(response) => {
            let plan = response.plan;
            println!("Plan Details:");
            println!("  Name: {}", plan.name);
            println!("  Code: {}", plan.code);
            println!("  Interval: {:?}", plan.interval);
            println!("  Amount: {} {}", plan.amount_cents, plan.amount_currency);
            println!("  Description: {:?}", plan.description);
            println!("  Charges: {}", plan.charges.len());

            for charge in &plan.charges {
                println!(
                    "    - {}: {:?} model",
                    charge.billable_metric_code, charge.charge_model
                );
            }
        }
        Err(e) => {
            println!("Failed to get plan: {}", e);
        }
    }

    // Example 4: Update a plan
    println!("\n=== Updating a Plan ===");
    let update_input = UpdatePlanInput::new()
        .with_name("Premium API Plan v2".to_string())
        .with_description("Updated premium API access with enhanced features".to_string());

    let update_request = UpdatePlanRequest::new(update_input);

    match client.update_plan("premium_api_v1", update_request).await {
        Ok(response) => {
            println!("Updated plan: {}", response.plan.name);
        }
        Err(e) => {
            println!("Failed to update plan: {}", e);
        }
    }

    // Example 5: Working with different charge models
    println!("\n=== Creating a Plan with Graduated Pricing ===");

    use lago_types::requests::plan::CreateGraduatedRange;

    let graduated_ranges = vec![
        CreateGraduatedRange {
            from_value: 0,
            to_value: Some(100),
            flat_amount: "0".to_string(),
            per_unit_amount: "0.05".to_string(), // 5 cents each for first 100
        },
        CreateGraduatedRange {
            from_value: 101,
            to_value: Some(1000),
            flat_amount: "0".to_string(),
            per_unit_amount: "0.03".to_string(), // 3 cents each for 101-1000
        },
        CreateGraduatedRange {
            from_value: 1001,
            to_value: None, // No upper limit
            flat_amount: "0".to_string(),
            per_unit_amount: "0.01".to_string(), // 1 cent each for 1001+
        },
    ];

    let graduated_properties = CreateChargeProperties::Graduated { graduated_ranges };

    let graduated_charge = CreateChargeInput::new(
        Uuid::new_v4(), // You would use a real billable metric ID here
        ChargeModel::Graduated,
        graduated_properties,
    )
    .with_invoice_display_name("Graduated API Calls".to_string())
    .with_invoiceable(true);

    let graduated_plan = CreatePlanInput::new(
        "Graduated API Plan".to_string(),
        "graduated_api_v1".to_string(),
        PlanInterval::Monthly,
        1999, // $19.99 base fee
        "USD".to_string(),
    )
    .with_description("API plan with graduated pricing tiers".to_string())
    .with_charges(vec![graduated_charge]);

    let graduated_request = CreatePlanRequest::new(graduated_plan);

    match client.create_plan(graduated_request).await {
        Ok(response) => {
            println!(
                "Created graduated plan: {} (ID: {})",
                response.plan.name, response.plan.lago_id
            );
        }
        Err(e) => {
            println!("Failed to create graduated plan: {}", e);
        }
    }

    Ok(())
}
