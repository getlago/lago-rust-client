use lago_client::LagoClient;
use lago_types::{
    models::{BillableMetricAggregationType, BillableMetricFilter, BillableMetricRoundingFunction},
    requests::billable_metric::{CreateBillableMetricInput, CreateBillableMetricRequest, GetBillableMetricRequest, ListBillableMetricsRequest},
    filters::billable_metric::BillableMetricFilter as BillableMetricFilterQuery,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: Create a new billable metric
    let new_metric = CreateBillableMetricInput::new(
        "Storage Usage".to_string(),
        "storage_gb".to_string(),
        BillableMetricAggregationType::SumAgg,
    )
    .with_description("Tracks storage usage in gigabytes".to_string())
    .with_field_name("gb_used".to_string())
    .with_recurring(false)
    .with_rounding_function(BillableMetricRoundingFunction::Round)
    .with_rounding_precision(2)
    .with_filters(vec![
        BillableMetricFilter::new(
            "region".to_string(),
            vec!["us-east-1".to_string(), "eu-west-1".to_string()],
        ),
    ]);

    let create_request = CreateBillableMetricRequest::new(new_metric);
    let created_metric = client.create_billable_metric(create_request).await?;
    println!("Created billable metric: {:?}", created_metric.billable_metric.name);

    // Example 2: List all billable metrics
    let list_request = ListBillableMetricsRequest::new();
    let metrics = client.list_billable_metrics(Some(list_request)).await?;
    println!("Found {} billable metrics", metrics.billable_metrics.len());

    // Example 3: List billable metrics with filters
    let filtered_request = ListBillableMetricsRequest::new()
        .with_filters(
            BillableMetricFilterQuery::new()
                .with_aggregation_type("sum_agg".to_string())
                .with_recurring(false)
        );
    let filtered_metrics = client.list_billable_metrics(Some(filtered_request)).await?;
    println!("Found {} filtered billable metrics", filtered_metrics.billable_metrics.len());

    // Example 4: Get a specific billable metric by code
    let get_request = GetBillableMetricRequest::new("storage_gb".to_string());
    let metric = client.get_billable_metric(get_request).await?;
    println!("Retrieved metric: {}", metric.billable_metric.name);

    Ok(())
}
