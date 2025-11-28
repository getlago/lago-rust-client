use lago_client::LagoClient;
use lago_types::{
    filters::applied_coupon::AppliedCouponFilter,
    models::{AppliedCouponStatus, CouponExpiration, CouponFrequency, PaginationParams},
    requests::applied_coupon::{ApplyCouponInput, ApplyCouponRequest, ListAppliedCouponsRequest},
    requests::coupon::{CreateCouponInput, CreateCouponRequest, GetCouponRequest},
    requests::customer::{CreateCustomerInput, CreateCustomerRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Create or update a customer to apply coupons to
    let new_customer = CreateCustomerInput::new("customer_123".to_string())
        .with_name("Example Customer".to_string())
        .with_email("customer@example.com".to_string());
    let create_customer_request = CreateCustomerRequest::new(new_customer);
    let created_customer = client.create_customer(create_customer_request).await?;
    println!(
        "Customer ready: {}",
        created_customer
            .customer
            .external_id
            .as_deref()
            .unwrap_or("customer_123")
    );

    // Fetch or create the coupon
    let coupon_code = "WELCOME10";
    let coupon = match client
        .get_coupon(GetCouponRequest::new(coupon_code.to_string()))
        .await
    {
        Ok(response) => {
            println!("Found existing coupon: {}", response.coupon.code);
            response.coupon
        }
        Err(_) => {
            // Coupon doesn't exist, create it
            let new_coupon = CreateCouponInput::percentage(
                "Welcome 10% Discount".to_string(),
                coupon_code.to_string(),
                "10".to_string(),
                CouponFrequency::Once,
                CouponExpiration::NoExpiration,
            )
            .with_reusable(true);
            let create_coupon_request = CreateCouponRequest::new(new_coupon);
            let created_coupon = client.create_coupon(create_coupon_request).await?;
            println!("Created coupon: {}", created_coupon.coupon.code);
            created_coupon.coupon
        }
    };
    println!("Coupon ready: {}", coupon.code);

    // Apply the coupon to the customer
    let apply_input = ApplyCouponInput::new("customer_123".to_string(), "WELCOME10".to_string());
    let apply_request = ApplyCouponRequest::new(apply_input);
    let applied = client.apply_coupon(apply_request).await?;
    println!(
        "Applied coupon '{}' to customer '{}' with status {:?}",
        applied.applied_coupon.coupon_code,
        applied.applied_coupon.external_customer_id,
        applied.applied_coupon.status
    );

    // List all applied coupons with default pagination
    let list_request = ListAppliedCouponsRequest::new();
    let applied_coupons = client.list_applied_coupons(Some(list_request)).await?;
    println!(
        "Found {} applied coupons",
        applied_coupons.applied_coupons.len()
    );

    // List applied coupons with pagination
    let paginated_request = ListAppliedCouponsRequest::new()
        .with_pagination(PaginationParams::default().with_page(1).with_per_page(20));
    let paginated_coupons = client.list_applied_coupons(Some(paginated_request)).await?;
    println!(
        "Page {}/{}: {} applied coupons",
        paginated_coupons.meta.current_page,
        paginated_coupons.meta.total_pages,
        paginated_coupons.applied_coupons.len()
    );

    // Filter applied coupons by status (active only)
    let active_request = ListAppliedCouponsRequest::new()
        .with_filters(AppliedCouponFilter::new().with_status(AppliedCouponStatus::Active));
    let active_coupons = client.list_applied_coupons(Some(active_request)).await?;
    println!(
        "Found {} active applied coupons",
        active_coupons.applied_coupons.len()
    );

    // Filter applied coupons by external customer ID
    let customer_request = ListAppliedCouponsRequest::new().with_filters(
        AppliedCouponFilter::new().with_external_customer_id("customer_123".to_string()),
    );
    let customer_coupons = client.list_applied_coupons(Some(customer_request)).await?;
    println!(
        "Found {} applied coupons for customer_123",
        customer_coupons.applied_coupons.len()
    );

    // Filter applied coupons by coupon code
    let codes_request = ListAppliedCouponsRequest::new()
        .with_filters(AppliedCouponFilter::new().with_coupon_codes(vec!["WELCOME10".to_string()]));
    let codes_coupons = client.list_applied_coupons(Some(codes_request)).await?;
    println!(
        "Found {} applied coupons matching WELCOME10",
        codes_coupons.applied_coupons.len()
    );

    // Combine multiple filters
    let combined_request = ListAppliedCouponsRequest::new()
        .with_pagination(PaginationParams::default().with_per_page(10))
        .with_filters(
            AppliedCouponFilter::new()
                .with_status(AppliedCouponStatus::Active)
                .with_external_customer_id("customer_123".to_string()),
        );
    let combined_coupons = client.list_applied_coupons(Some(combined_request)).await?;
    println!(
        "Found {} active applied coupons for customer_123",
        combined_coupons.applied_coupons.len()
    );

    // Display applied coupon details
    if let Some(coupon) = applied_coupons.applied_coupons.first() {
        println!("\nApplied Coupon Details:");
        println!("  Lago ID: {}", coupon.lago_id);
        println!("  Coupon Code: {}", coupon.coupon_code);
        println!("  Coupon Name: {}", coupon.coupon_name);
        println!("  Customer: {}", coupon.external_customer_id);
        println!("  Status: {:?}", coupon.status);
        println!("  Frequency: {:?}", coupon.frequency);
        if let Some(amount) = coupon.amount_cents {
            println!(
                "  Amount: {} {}",
                amount,
                coupon.amount_currency.as_deref().unwrap_or("")
            );
        }
        if let Some(rate) = &coupon.percentage_rate {
            println!("  Percentage Rate: {}%", rate);
        }
        if let Some(remaining) = coupon.amount_cents_remaining {
            println!("  Amount Remaining: {}", remaining);
        }
        if let Some(duration_remaining) = coupon.frequency_duration_remaining {
            println!("  Periods Remaining: {}", duration_remaining);
        }
        if let Some(expiration) = coupon.expiration_at {
            println!("  Expires At: {}", expiration);
        }
        println!("  Created At: {}", coupon.created_at);
    }

    Ok(())
}
