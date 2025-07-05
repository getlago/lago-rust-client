use lago_client::LagoClient;
use lago_types::requests::invoice::ListInvoicesRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = LagoClient::from_env()?;

  let request = ListInvoicesRequest::default();
  let invoices = client.list_invoices(Some(request)).await?;

  println!("Found {} invoices", invoices.invoices.len());

  Ok(())
}