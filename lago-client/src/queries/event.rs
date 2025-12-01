use lago_types::{
    error::Result,
    requests::event::{CreateEventRequest, GetEventRequest},
    responses::event::{CreateEventResponse, GetEventResponse},
};

use crate::client::LagoClient;

impl LagoClient {
    /// Retrieves a specific event by its transaction ID
    ///
    /// # Arguments
    /// * `request` - The request containing the transaction ID to retrieve
    ///
    /// # Returns
    /// A `Result` containing the event data or an error
    ///
    /// # Example
    /// ```no_run
    /// use lago_client::LagoClient;
    /// use lago_types::requests::event::GetEventRequest;
    ///
    /// # async fn example() -> lago_types::error::Result<()> {
    /// let client = LagoClient::from_env()?;
    /// let request = GetEventRequest::new("transaction_123".to_string());
    /// let response = client.get_event(request).await?;
    /// println!("Event code: {}", response.event.code);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_event(&self, request: GetEventRequest) -> Result<GetEventResponse> {
        let region = self.config.region()?;
        // URL encode the transaction_id to handle special characters
        let encoded_transaction_id = urlencoding::encode(&request.transaction_id).into_owned();
        let url = format!("{}/events/{}", region.endpoint(), encoded_transaction_id);

        self.make_request("GET", &url, None::<&()>).await
    }

    /// Creates a new usage event
    ///
    /// This endpoint is used for transmitting usage measurement events to either
    /// a designated customer or a specific subscription.
    ///
    /// # Arguments
    /// * `request` - The request containing the event data to create
    ///
    /// # Returns
    /// A `Result` containing the created event acknowledgment or an error
    ///
    /// # Example
    /// ```no_run
    /// use lago_client::LagoClient;
    /// use lago_types::requests::event::{CreateEventInput, CreateEventRequest};
    /// use serde_json::json;
    ///
    /// # async fn example() -> lago_types::error::Result<()> {
    /// let client = LagoClient::from_env()?;
    /// let event_input = CreateEventInput::for_customer(
    ///     "transaction_456".to_string(),
    ///     "customer_123".to_string(),
    ///     "api_calls".to_string(),
    /// )
    /// .with_properties(json!({"calls": 150}));
    ///
    /// let request = CreateEventRequest::new(event_input);
    /// let response = client.create_event(request).await?;
    /// println!("Event created: {}", response.event.transaction_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_event(&self, request: CreateEventRequest) -> Result<CreateEventResponse> {
        let region = self.config.region()?;
        let url = format!("{}/events", region.endpoint());

        self.make_request("POST", &url, Some(&request)).await
    }
}
