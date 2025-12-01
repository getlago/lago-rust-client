use lago_types::{
    error::{LagoError, Result},
    requests::credit_note::{
        CreateCreditNoteRequest, GetCreditNoteRequest, ListCreditNotesRequest,
        UpdateCreditNoteRequest,
    },
    responses::credit_note::{
        CreateCreditNoteResponse, GetCreditNoteResponse, ListCreditNotesResponse,
        UpdateCreditNoteResponse,
    },
};
use url::Url;

use crate::client::LagoClient;

/// Credit note-related operations for the Lago client
impl LagoClient {
    /// Retrieves a list of credit notes with optional filtering parameters
    ///
    /// # Arguments
    /// * `request` - Optional filtering parameters for the credit note list
    ///
    /// # Returns
    /// A `Result` containing the list of credit notes or an error
    ///
    /// # Example
    /// ```no_run
    /// use lago_client::LagoClient;
    /// use lago_types::requests::credit_note::ListCreditNotesRequest;
    /// use lago_types::filters::credit_note::CreditNoteFilter;
    ///
    /// # async fn example() -> lago_types::error::Result<()> {
    /// let client = LagoClient::from_env()?;
    /// let request = ListCreditNotesRequest::new()
    ///     .with_filters(CreditNoteFilter::new().with_external_customer_id("customer_123".to_string()));
    /// let response = client.list_credit_notes(Some(request)).await?;
    /// println!("Found {} credit notes", response.credit_notes.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_credit_notes(
        &self,
        request: Option<ListCreditNotesRequest>,
    ) -> Result<ListCreditNotesResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/credit_notes", region.endpoint()))
            .map_err(|e| LagoError::Configuration(format!("Invalid URL: {e}")))?;

        let query_params = request.to_query_params();

        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url.set_query(Some(&query_string));
        }

        self.make_request("GET", url.as_str(), None::<&()>).await
    }

    /// Retrieves a specific credit note by its Lago ID
    ///
    /// # Arguments
    /// * `request` - The request containing the Lago ID to retrieve
    ///
    /// # Returns
    /// A `Result` containing the credit note data or an error
    ///
    /// # Example
    /// ```no_run
    /// use lago_client::LagoClient;
    /// use lago_types::requests::credit_note::GetCreditNoteRequest;
    ///
    /// # async fn example() -> lago_types::error::Result<()> {
    /// let client = LagoClient::from_env()?;
    /// let request = GetCreditNoteRequest::new("1a901a90-1a90-1a90-1a90-1a901a901a90".to_string());
    /// let response = client.get_credit_note(request).await?;
    /// println!("Credit note number: {}", response.credit_note.number);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_credit_note(
        &self,
        request: GetCreditNoteRequest,
    ) -> Result<GetCreditNoteResponse> {
        let region = self.config.region()?;
        let url = format!("{}/credit_notes/{}", region.endpoint(), request.lago_id);
        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a new credit note
    ///
    /// Credit notes are issued to refund or credit customers for invoices,
    /// either partially or in full.
    ///
    /// # Arguments
    /// * `request` - The request containing the credit note data to create
    ///
    /// # Returns
    /// A `Result` containing the created credit note or an error
    ///
    /// # Example
    /// ```no_run
    /// use lago_client::LagoClient;
    /// use lago_types::requests::credit_note::{
    ///     CreateCreditNoteInput, CreateCreditNoteItemInput, CreateCreditNoteRequest,
    /// };
    /// use lago_types::models::CreditNoteReason;
    ///
    /// # async fn example() -> lago_types::error::Result<()> {
    /// let client = LagoClient::from_env()?;
    /// let items = vec![
    ///     CreateCreditNoteItemInput::new("fee_lago_id".to_string(), 1000),
    /// ];
    /// let input = CreateCreditNoteInput::new(
    ///     "invoice_lago_id".to_string(),
    ///     CreditNoteReason::Other,
    ///     1000,
    ///     0,
    ///     items,
    /// )
    /// .with_description("Credit for billing error".to_string());
    ///
    /// let request = CreateCreditNoteRequest::new(input);
    /// let response = client.create_credit_note(request).await?;
    /// println!("Created credit note: {}", response.credit_note.number);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_credit_note(
        &self,
        request: CreateCreditNoteRequest,
    ) -> Result<CreateCreditNoteResponse> {
        let region = self.config.region()?;
        let url = format!("{}/credit_notes", region.endpoint());
        self.make_request("POST", &url, Some(&request)).await
    }

    /// Updates an existing credit note
    ///
    /// Currently, only the refund_status can be updated on a credit note.
    ///
    /// # Arguments
    /// * `request` - The request containing the Lago ID and update data
    ///
    /// # Returns
    /// A `Result` containing the updated credit note or an error
    ///
    /// # Example
    /// ```no_run
    /// use lago_client::LagoClient;
    /// use lago_types::requests::credit_note::{UpdateCreditNoteInput, UpdateCreditNoteRequest};
    /// use lago_types::models::CreditNoteRefundStatus;
    ///
    /// # async fn example() -> lago_types::error::Result<()> {
    /// let client = LagoClient::from_env()?;
    /// let input = UpdateCreditNoteInput::new()
    ///     .with_refund_status(CreditNoteRefundStatus::Succeeded);
    ///
    /// let request = UpdateCreditNoteRequest::new(
    ///     "1a901a90-1a90-1a90-1a90-1a901a901a90".to_string(),
    ///     input,
    /// );
    /// let response = client.update_credit_note(request).await?;
    /// println!("Updated credit note: {}", response.credit_note.number);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_credit_note(
        &self,
        request: UpdateCreditNoteRequest,
    ) -> Result<UpdateCreditNoteResponse> {
        let region = self.config.region()?;
        let url = format!("{}/credit_notes/{}", region.endpoint(), request.lago_id);
        self.make_request("PUT", &url, Some(&request)).await
    }
}
