use lago_client::LagoClient;
use lago_types::requests::invoice::{
    DownloadInvoiceRequest, GetInvoiceRequest, ListCustomerInvoicesRequest, ListInvoicesRequest,
    UpdateInvoiceInput, UpdateInvoiceMetadataInput, UpdateInvoiceRequest, VoidInvoiceRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Example 1: List all invoices
    let list_request = ListInvoicesRequest::new();
    let invoices = client.list_invoices(Some(list_request)).await?;
    println!("Found {} invoices", invoices.invoices.len());

    for invoice in &invoices.invoices {
        println!(
            "  Invoice {}: {} {} - {:?}",
            invoice.number, invoice.total_amount_cents, invoice.currency, invoice.status
        );
    }

    // Example 2: Get a specific invoice by ID (using first invoice from list)
    if let Some(first_invoice) = invoices.invoices.first() {
        let invoice_id = first_invoice.lago_id.as_ref().unwrap();
        let get_request = GetInvoiceRequest::new(invoice_id.to_string());
        let invoice = client.get_invoice(get_request).await?;
        println!("\nRetrieved invoice: {}", invoice.invoice.number);
        println!("  Status: {:?}", invoice.invoice.status);
        println!(
            "  Total: {} {}",
            invoice.invoice.total_amount_cents, invoice.invoice.currency
        );

        // Example 3: List invoices for the same customer
        if let Some(customer) = &first_invoice.customer {
            if let Some(customer_id) = &customer.external_id {
                let list_customer_request = ListCustomerInvoicesRequest::new(customer_id.clone());
                let customer_invoices =
                    client.list_customer_invoices(list_customer_request).await?;
                println!(
                    "\nCustomer {} has {} invoices",
                    customer_id,
                    customer_invoices.invoices.len()
                );
            }
        }

        // Example 4: Update an invoice's metadata
        let metadata =
            UpdateInvoiceMetadataInput::new("example_key".to_string(), "example_value".to_string());
        let update_input = UpdateInvoiceInput::new().with_metadata(vec![metadata]);
        let update_request = UpdateInvoiceRequest::new(invoice_id.to_string(), update_input);
        let updated = client.update_invoice(update_request).await?;
        println!("\nUpdated invoice {} with metadata", updated.invoice.number);

        // Example 5: Download an invoice PDF
        let download_request = DownloadInvoiceRequest::new(invoice_id.to_string());
        let downloaded = client.download_invoice(download_request).await?;
        if let Some(url) = downloaded.invoice.file_url {
            println!("\nInvoice PDF URL: {}", url);
        } else {
            println!("\nInvoice PDF not yet available");
        }

        // Example 6: Void a finalized invoice (commented out to avoid modifying data)
        // Only finalized invoices can be voided - this changes the status to "voided"
        // let void_request = VoidInvoiceRequest::new(invoice_id.to_string());
        // let voided = client.void_invoice(void_request).await?;
        // println!("\nVoided invoice: {} - Status: {:?}", voided.invoice.number, voided.invoice.status);
        println!("\nExample 6: Void invoice (skipped - uncomment to test)");
        println!("  Usage: VoidInvoiceRequest::new(invoice_id)");
    } else {
        println!("No invoices found, skipping detailed examples");
    }

    Ok(())
}
