use lago_client::{
  LagoClient,
  Config,
  Region,
  Credentials,
  RetryConfig,
  RetryMode,
};

use lago_types::requests::invoice::ListInvoicesRequest;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::builder()
    .credentials(Credentials::new("YOUR_API_KEY".to_string()))
    .region(Region::Us)
    .timeout(Duration::from_secs(30))
    .retry_config(
      RetryConfig::builder()
        .mode(RetryMode::Standard)
        .max_attempts(3)
        .initial_delay(Duration::from_millis(100))
        .max_delay(Duration::from_secs(10))
        .backoff_multiplier(2.0)
        .build()
    )
    .build();

  let client = LagoClient::new(config);

  println!("Client created with custom configuration");

  let request = ListInvoicesRequest::default();
  let invoices = client.list_invoices(Some(request)).await?;

  println!("Found {} invoices", invoices.invoices.len());

  Ok(())
}