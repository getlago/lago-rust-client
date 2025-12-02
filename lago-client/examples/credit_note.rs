use lago_client::LagoClient;
use lago_types::filters::credit_note::CreditNoteFilter;
use lago_types::filters::invoice::InvoiceFilters;
use lago_types::models::{CreditNoteReason, InvoiceStatus};
use lago_types::requests::credit_note::{
    CreateCreditNoteInput, CreateCreditNoteItemInput, CreateCreditNoteRequest,
    GetCreditNoteRequest, ListCreditNotesRequest,
};
use lago_types::requests::invoice::{GetInvoiceRequest, ListInvoicesRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LagoClient::from_env()?;

    // Step 1: Find a finalized invoice that we can create a credit note from
    println!("=== Finding a finalized invoice ===");
    let invoice_request = ListInvoicesRequest::new()
        .with_filters(InvoiceFilters::new().with_status(InvoiceStatus::Finalized));
    let invoices_response = client.list_invoices(Some(invoice_request)).await?;

    if invoices_response.invoices.is_empty() {
        println!("No finalized invoices found. Please create an invoice first.");
        return Ok(());
    }

    // Find an invoice with fees and no coupons applied
    let invoice_summary = invoices_response
        .invoices
        .iter()
        .find(|inv| inv.fees_amount_cents > 0 && inv.coupons_amount_cents == 0)
        .or_else(|| invoices_response.invoices.first());

    let invoice_summary = match invoice_summary {
        Some(inv) => inv,
        None => {
            println!("No suitable invoice found.");
            return Ok(());
        }
    };

    let invoice_lago_id = invoice_summary
        .lago_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    println!(
        "Found invoice: {} (ID: {})",
        invoice_summary.number, invoice_lago_id
    );
    println!(
        "  Total: {} cents, Fees: {} cents",
        invoice_summary.total_amount_cents, invoice_summary.fees_amount_cents
    );

    // Step 2: Fetch the full invoice details to get fee IDs
    println!("\n=== Fetching invoice details to get fees ===");
    let get_invoice_request = GetInvoiceRequest::new(invoice_lago_id.clone());
    let invoice_response = client.get_invoice(get_invoice_request).await?;
    let invoice = &invoice_response.invoice;

    // Check if we have fees
    let fees = match &invoice.fees {
        Some(fees) if !fees.is_empty() => fees,
        _ => {
            println!("Invoice has no fees. Cannot create a credit note.");
            return Ok(());
        }
    };

    println!("Found {} fee(s) on invoice:", fees.len());
    for fee in fees {
        let item_name = fee
            .item
            .as_ref()
            .map(|i| i.name.as_str())
            .unwrap_or("Unknown");
        println!(
            "  - {} (ID: {}): {} cents",
            item_name, fee.lago_id, fee.total_amount_cents
        );
    }

    // Step 3: Create a credit note using the first fee with a positive amount
    println!("\n=== Creating a credit note ===");
    let first_fee = match fees.iter().find(|f| f.total_amount_cents > 0) {
        Some(fee) => fee,
        None => {
            println!("No fee with positive amount found. Cannot create a credit note.");
            return Ok(());
        }
    };

    // Credit a portion of the fee (or the full amount if small)
    let credit_amount = std::cmp::min(first_fee.total_amount_cents, 1000);

    let items = vec![CreateCreditNoteItemInput::new(
        first_fee.lago_id.to_string(),
        credit_amount,
    )];

    let input = CreateCreditNoteInput::new(
        invoice_lago_id.clone(),
        CreditNoteReason::Other,
        credit_amount, // credit_amount_cents
        0,             // refund_amount_cents
        items,
    )
    .with_description("Credit note created from example".to_string());

    let create_request = CreateCreditNoteRequest::new(input);
    let created = client.create_credit_note(create_request).await?;
    println!("Created credit note: {}", created.credit_note.number);
    println!("  Lago ID: {}", created.credit_note.lago_id);
    println!("  Reason: {:?}", created.credit_note.reason);
    println!(
        "  Total amount: {} cents",
        created.credit_note.total_amount_cents
    );

    let created_credit_note_id = created.credit_note.lago_id.to_string();

    // Step 4: Fetch the created credit note
    println!("\n=== Fetching the created credit note ===");
    let get_request = GetCreditNoteRequest::new(created_credit_note_id.clone());
    let credit_note_response = client.get_credit_note(get_request).await?;
    let credit_note = &credit_note_response.credit_note;

    println!("Credit note: {}", credit_note.number);
    println!("  Invoice: {}", credit_note.invoice_number);
    println!("  Reason: {:?}", credit_note.reason);
    println!("  Total amount: {} cents", credit_note.total_amount_cents);
    println!("  Credit amount: {} cents", credit_note.credit_amount_cents);
    println!("  Refund amount: {} cents", credit_note.refund_amount_cents);
    println!("  Credit status: {:?}", credit_note.credit_status);
    println!("  Refund status: {:?}", credit_note.refund_status);
    if let Some(ref items) = credit_note.items {
        println!("  Items: {}", items.len());
        for item in items {
            println!(
                "    - Amount: {} cents, Fee: {:?}",
                item.amount_cents, item.fee
            );
        }
    }

    // Step 5: List all credit notes
    println!("\n=== Listing all credit notes ===");
    let response = client.list_credit_notes(None).await?;
    println!("Found {} credit notes", response.credit_notes.len());
    for cn in &response.credit_notes {
        println!(
            "  - {} ({:?}): {} cents",
            cn.number, cn.reason, cn.total_amount_cents
        );
    }

    // Step 6: Filter credit notes by customer
    if let Some(customer) = &invoice.customer {
        if let Some(ref external_id) = customer.external_id {
            println!("\n=== Filtering credit notes by customer ===");
            let filter = CreditNoteFilter::new().with_external_customer_id(external_id.clone());
            let request = ListCreditNotesRequest::new().with_filters(filter);
            let filtered = client.list_credit_notes(Some(request)).await?;
            println!(
                "Found {} credit notes for customer '{}'",
                filtered.credit_notes.len(),
                external_id
            );
        }
    }

    // Step 7: Filter by reason
    println!("\n=== Filtering credit notes by reason ===");
    let filter = CreditNoteFilter::new().with_reason(CreditNoteReason::Other);
    let request = ListCreditNotesRequest::new().with_filters(filter);
    let filtered = client.list_credit_notes(Some(request)).await?;
    println!(
        "Found {} credit notes with reason 'Other'",
        filtered.credit_notes.len()
    );

    // Step 8: Filter by amount range
    println!("\n=== Filtering by amount range ===");
    let filter = CreditNoteFilter::new().with_amount_range(1, 100000);
    let request = ListCreditNotesRequest::new().with_filters(filter);
    let filtered = client.list_credit_notes(Some(request)).await?;
    println!(
        "Found {} credit notes between 1 and 100000 cents",
        filtered.credit_notes.len()
    );

    Ok(())
}
