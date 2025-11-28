use lago_client::LagoClient;
use lago_types::{
    models::{CouponExpiration, CouponFrequency, PaginationParams},
    requests::coupon::{
        CreateCouponInput, CreateCouponRequest, DeleteCouponRequest, GetCouponRequest,
        ListCouponsRequest, UpdateCouponInput, UpdateCouponRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Create a percentage-based coupon
    let percentage_coupon = CreateCouponInput::percentage(
        "Welcome 10% Discount".to_string(),
        "WELCOME10".to_string(),
        "10".to_string(),
        CouponFrequency::Once,
        CouponExpiration::NoExpiration,
    )
    .with_reusable(true);

    let create_request = CreateCouponRequest::new(percentage_coupon);
    match client.create_coupon(create_request).await {
        Ok(response) => {
            println!("Created percentage coupon: {}", response.coupon.code);
            println!("  Name: {}", response.coupon.name);
            println!("  Type: {:?}", response.coupon.coupon_type);
            println!(
                "  Percentage Rate: {}%",
                response.coupon.percentage_rate.as_deref().unwrap_or("N/A")
            );
        }
        Err(e) => println!("Coupon may already exist or error: {e}"),
    }

    // Create a fixed amount coupon
    let fixed_coupon = CreateCouponInput::fixed_amount(
        "Summer $50 Off".to_string(),
        "SUMMER50".to_string(),
        5000, // $50.00 in cents
        "USD".to_string(),
        CouponFrequency::Recurring,
        CouponExpiration::NoExpiration,
    )
    .with_frequency_duration(3) // Apply for 3 billing periods
    .with_reusable(true);

    let create_request = CreateCouponRequest::new(fixed_coupon);
    match client.create_coupon(create_request).await {
        Ok(response) => {
            println!("\nCreated fixed amount coupon: {}", response.coupon.code);
            println!("  Name: {}", response.coupon.name);
            println!("  Type: {:?}", response.coupon.coupon_type);
            println!(
                "  Amount: {} {}",
                response.coupon.amount_cents.unwrap_or(0),
                response.coupon.amount_currency.as_deref().unwrap_or("N/A")
            );
            println!("  Frequency: {:?}", response.coupon.frequency);
            println!(
                "  Duration: {} periods",
                response.coupon.frequency_duration.unwrap_or(0)
            );
        }
        Err(e) => println!("Coupon may already exist or error: {e}"),
    }

    // List all coupons
    println!("\n--- Listing all coupons ---");
    let list_request = ListCouponsRequest::new();
    let coupons = client.list_coupons(Some(list_request)).await?;
    println!(
        "Found {} coupons (page {}/{})",
        coupons.coupons.len(),
        coupons.meta.current_page,
        coupons.meta.total_pages
    );

    for coupon in &coupons.coupons {
        println!(
            "  - {} ({}): {:?}",
            coupon.name, coupon.code, coupon.coupon_type
        );
    }

    // List coupons with pagination
    let paginated_request =
        ListCouponsRequest::new().with_pagination(PaginationParams::default().with_per_page(5));
    let paginated_coupons = client.list_coupons(Some(paginated_request)).await?;
    println!(
        "\nPaginated results: {} coupons on page {}",
        paginated_coupons.coupons.len(),
        paginated_coupons.meta.current_page
    );

    // Get a specific coupon
    println!("\n--- Getting specific coupon ---");
    let get_request = GetCouponRequest::new("WELCOME10".to_string());
    match client.get_coupon(get_request).await {
        Ok(response) => {
            println!("Retrieved coupon: {}", response.coupon.code);
            println!("  Name: {}", response.coupon.name);
            println!("  Reusable: {}", response.coupon.reusable);
            println!("  Limited Plans: {}", response.coupon.limited_plans);
            println!(
                "  Limited Billable Metrics: {}",
                response.coupon.limited_billable_metrics
            );
        }
        Err(e) => println!("Failed to get coupon: {e}"),
    }

    // Update a coupon
    println!("\n--- Updating coupon ---");
    let update_input = UpdateCouponInput::new()
        .with_name("Welcome 15% Discount".to_string())
        .with_percentage_rate("15".to_string());

    let update_request = UpdateCouponRequest::new("WELCOME10".to_string(), update_input);
    match client.update_coupon(update_request).await {
        Ok(response) => {
            println!("Updated coupon: {}", response.coupon.code);
            println!("  New Name: {}", response.coupon.name);
            println!(
                "  New Percentage Rate: {}%",
                response.coupon.percentage_rate.as_deref().unwrap_or("N/A")
            );
        }
        Err(e) => println!("Failed to update coupon: {e}"),
    }

    // Update coupon with plan limitations
    let limited_update = UpdateCouponInput::new()
        .with_limited_plans(vec!["startup".to_string(), "professional".to_string()]);

    let update_request = UpdateCouponRequest::new("WELCOME10".to_string(), limited_update);
    match client.update_coupon(update_request).await {
        Ok(response) => {
            println!("\nUpdated coupon with plan limitations:");
            println!("  Limited Plans: {}", response.coupon.limited_plans);
            if let Some(codes) = &response.coupon.plan_codes {
                println!("  Plan Codes: {:?}", codes);
            }
        }
        Err(e) => println!("Failed to update coupon: {e}"),
    }

    // Delete a coupon (commented out to preserve test data)
    println!("\n--- Deleting coupon (SUMMER50) ---");
    let delete_request = DeleteCouponRequest::new("SUMMER50".to_string());
    match client.delete_coupon(delete_request).await {
        Ok(response) => {
            println!("Deleted coupon: {}", response.coupon.code);
            println!(
                "  Terminated At: {:?}",
                response.coupon.terminated_at.map(|t| t.to_string())
            );
        }
        Err(e) => println!("Failed to delete coupon: {e}"),
    }

    Ok(())
}
