use lago_client::LagoClient;
use lago_types::{
    filters::api_log::ApiLogFilters,
    models::{HttpMethod, HttpStatus, PaginationParams, StatusOutcome},
    requests::api_log::{GetApiLogRequest, ListApiLogsRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: List all API logs with default pagination
    let list_request = ListApiLogsRequest::new();
    let api_logs = client.list_api_logs(Some(list_request)).await?;
    println!("Found {} API logs", api_logs.api_logs.len());

    // Example 2: List API logs with pagination
    let paginated_request = ListApiLogsRequest::new()
        .with_pagination(PaginationParams::default().with_page(1).with_per_page(20));
    let paginated_logs = client.list_api_logs(Some(paginated_request)).await?;
    println!(
        "Page {}/{}: {} API logs",
        paginated_logs.meta.current_page,
        paginated_logs.meta.total_pages,
        paginated_logs.api_logs.len()
    );

    // Example 3: Filter API logs by HTTP methods
    let method_filtered_request = ListApiLogsRequest::new().with_filters(
        ApiLogFilters::new().with_http_methods(vec![HttpMethod::Post, HttpMethod::Put]),
    );
    let method_filtered_logs = client.list_api_logs(Some(method_filtered_request)).await?;
    println!(
        "Found {} API logs for POST/PUT methods",
        method_filtered_logs.api_logs.len()
    );

    // Example 4: Filter API logs by status outcomes
    let status_filtered_request = ListApiLogsRequest::new().with_filters(
        ApiLogFilters::new().with_http_statuses(vec![HttpStatus::Outcome(StatusOutcome::Failed)]),
    );
    let status_filtered_logs = client.list_api_logs(Some(status_filtered_request)).await?;
    println!(
        "Found {} failed API logs",
        status_filtered_logs.api_logs.len()
    );

    // Example 5: Filter API logs by specific status codes
    let code_filtered_request = ListApiLogsRequest::new().with_filters(
        ApiLogFilters::new().with_http_statuses(vec![HttpStatus::Code(200), HttpStatus::Code(201)]),
    );
    let code_filtered_logs = client.list_api_logs(Some(code_filtered_request)).await?;
    println!(
        "Found {} API logs with 200/201 status codes",
        code_filtered_logs.api_logs.len()
    );

    // Example 6: Filter API logs by date range
    let date_filtered_request = ListApiLogsRequest::new().with_filters(
        ApiLogFilters::new().with_date_range("2025-01-01".to_string(), "2025-01-31".to_string()),
    );
    let date_filtered_logs = client.list_api_logs(Some(date_filtered_request)).await?;
    println!(
        "Found {} API logs in January 2025",
        date_filtered_logs.api_logs.len()
    );

    // Example 7: Filter API logs by API version
    let version_filtered_request = ListApiLogsRequest::new()
        .with_filters(ApiLogFilters::new().with_api_version("v1".to_string()));
    let version_filtered_logs = client.list_api_logs(Some(version_filtered_request)).await?;
    println!(
        "Found {} API logs for version v1",
        version_filtered_logs.api_logs.len()
    );

    // Example 8: Filter API logs by request path
    let path_filtered_request = ListApiLogsRequest::new()
        .with_filters(ApiLogFilters::new().with_request_paths("/billable_metrics/".to_string()));
    let path_filtered_logs = client.list_api_logs(Some(path_filtered_request)).await?;
    println!(
        "Found {} API logs for /billable_metrics/ path",
        path_filtered_logs.api_logs.len()
    );

    // Example 9: Combine multiple filters
    let combined_request = ListApiLogsRequest::new()
        .with_pagination(PaginationParams::default().with_per_page(10))
        .with_filters(
            ApiLogFilters::new()
                .with_http_methods(vec![HttpMethod::Post])
                .with_http_statuses(vec![HttpStatus::Outcome(StatusOutcome::Succeeded)])
                .with_api_version("v1".to_string())
                .with_from_date("2025-01-01".to_string()),
        );
    let combined_logs = client.list_api_logs(Some(combined_request)).await?;
    println!(
        "Found {} API logs matching combined filters",
        combined_logs.api_logs.len()
    );

    // Example 10: Get a specific API log by request ID
    // Note: Replace with an actual request_id from your list results
    if let Some(first_log) = api_logs.api_logs.first() {
        let request_id = first_log.request_id.to_string();
        let get_request = GetApiLogRequest::new(request_id.clone());
        let api_log = client.get_api_log(get_request).await?;
        println!(
            "Retrieved API log: {:?} {} - Status {}",
            api_log.api_log.http_method, api_log.api_log.request_path, api_log.api_log.http_status
        );
    }

    Ok(())
}
