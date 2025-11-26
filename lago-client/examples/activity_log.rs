use lago_client::LagoClient;
use lago_types::{
    filters::activity_log::ActivityLogFilters,
    models::{ActivitySource, PaginationParams},
    requests::activity_log::{GetActivityLogRequest, ListActivityLogsRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: List all activity logs with default pagination
    let list_request = ListActivityLogsRequest::new();
    let activity_logs = client.list_activity_logs(Some(list_request)).await?;
    println!("Found {} activity logs", activity_logs.activity_logs.len());

    // Example 2: List activity logs with pagination
    let paginated_request = ListActivityLogsRequest::new().with_pagination(
        PaginationParams::default()
            .with_page(1)
            .with_per_page(20),
    );
    let paginated_logs = client.list_activity_logs(Some(paginated_request)).await?;
    println!(
        "Page {}/{}: {} activity logs",
        paginated_logs.meta.current_page,
        paginated_logs.meta.total_pages,
        paginated_logs.activity_logs.len()
    );

    // Example 3: Filter activity logs by activity types
    let type_filtered_request = ListActivityLogsRequest::new().with_filters(
        ActivityLogFilters::new().with_activity_types(vec![
            "billable_metric.created".to_string(),
            "invoice.created".to_string(),
        ]),
    );
    let type_filtered_logs = client.list_activity_logs(Some(type_filtered_request)).await?;
    println!(
        "Found {} activity logs for specified activity types",
        type_filtered_logs.activity_logs.len()
    );

    // Example 4: Filter activity logs by activity sources
    let source_filtered_request = ListActivityLogsRequest::new().with_filters(
        ActivityLogFilters::new().with_activity_sources(vec![ActivitySource::Api, ActivitySource::Front]),
    );
    let source_filtered_logs = client.list_activity_logs(Some(source_filtered_request)).await?;
    println!(
        "Found {} activity logs from API and Front sources",
        source_filtered_logs.activity_logs.len()
    );

    // Example 5: Filter activity logs by user emails
    let email_filtered_request = ListActivityLogsRequest::new().with_filters(
        ActivityLogFilters::new().with_user_emails(vec!["admin@example.com".to_string()]),
    );
    let email_filtered_logs = client.list_activity_logs(Some(email_filtered_request)).await?;
    println!(
        "Found {} activity logs for specified user",
        email_filtered_logs.activity_logs.len()
    );

    // Example 6: Filter activity logs by date range
    let date_filtered_request = ListActivityLogsRequest::new().with_filters(
        ActivityLogFilters::new().with_date_range("2025-01-01".to_string(), "2025-01-31".to_string()),
    );
    let date_filtered_logs = client.list_activity_logs(Some(date_filtered_request)).await?;
    println!(
        "Found {} activity logs in January 2025",
        date_filtered_logs.activity_logs.len()
    );

    // Example 7: Filter activity logs by external customer ID
    let customer_filtered_request = ListActivityLogsRequest::new()
        .with_filters(ActivityLogFilters::new().with_external_customer_id("customer_123".to_string()));
    let customer_filtered_logs = client.list_activity_logs(Some(customer_filtered_request)).await?;
    println!(
        "Found {} activity logs for customer_123",
        customer_filtered_logs.activity_logs.len()
    );

    // Example 8: Filter activity logs by resource types
    let resource_filtered_request = ListActivityLogsRequest::new().with_filters(
        ActivityLogFilters::new().with_resource_types(vec![
            "BillableMetric".to_string(),
            "Invoice".to_string(),
        ]),
    );
    let resource_filtered_logs = client.list_activity_logs(Some(resource_filtered_request)).await?;
    println!(
        "Found {} activity logs for BillableMetric and Invoice resources",
        resource_filtered_logs.activity_logs.len()
    );

    // Example 9: Combine multiple filters
    let combined_request = ListActivityLogsRequest::new()
        .with_pagination(PaginationParams::default().with_per_page(10))
        .with_filters(
            ActivityLogFilters::new()
                .with_activity_sources(vec![ActivitySource::Api])
                .with_resource_types(vec!["Invoice".to_string()])
                .with_from_date("2025-01-01".to_string()),
        );
    let combined_logs = client.list_activity_logs(Some(combined_request)).await?;
    println!(
        "Found {} activity logs matching combined filters",
        combined_logs.activity_logs.len()
    );

    // Example 10: Get a specific activity log by activity ID
    if let Some(first_log) = activity_logs.activity_logs.first() {
        let activity_id = first_log.activity_id.to_string();
        let get_request = GetActivityLogRequest::new(activity_id.clone());
        let activity_log = client.get_activity_log(get_request).await?;
        println!(
            "Retrieved activity log: {} - {} from {:?}",
            activity_log.activity_log.activity_type,
            activity_log.activity_log.resource_type.as_deref().unwrap_or("N/A"),
            activity_log.activity_log.activity_source
        );
        if let Some(activity_object) = &activity_log.activity_log.activity_object {
            println!("Activity object: {:?}", activity_object);
        }
    }

    Ok(())
}
