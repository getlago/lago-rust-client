use lago_types::{
    error::{LagoError, Result},
    requests::event::{CreateEventRequest, GetEventRequest, ListEventsRequest},
    responses::event::{CreateEventResponse, GetEventResponse, ListEventsResponse},
};
use url::Url;

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

    /// Retrieves a list of events with optional filtering parameters
    ///
    /// This endpoint is used for retrieving all events, with support for filtering
    /// by subscription, billable metric code, and timestamp range.
    ///
    /// # Arguments
    /// * `request` - Optional request containing filter and pagination parameters.
    ///   If `None`, returns all events with default pagination.
    ///
    /// # Returns
    /// A `Result` containing the list of events with pagination metadata or an error
    ///
    /// # Example
    /// ```no_run
    /// use lago_client::LagoClient;
    /// use lago_types::requests::event::ListEventsRequest;
    /// use lago_types::models::PaginationParams;
    ///
    /// # async fn example() -> lago_types::error::Result<()> {
    /// let client = LagoClient::from_env()?;
    ///
    /// // List all events
    /// let events = client.list_events(None).await?;
    /// println!("Found {} events", events.events.len());
    ///
    /// // List events with filters
    /// let request = ListEventsRequest::new()
    ///     .with_pagination(PaginationParams::new().with_per_page(50))
    ///     .with_external_subscription_id("sub_123".to_string())
    ///     .with_code("api_calls".to_string())
    ///     .with_timestamp_range(
    ///         "2024-01-01T00:00:00Z".to_string(),
    ///         "2024-01-31T23:59:59Z".to_string(),
    ///     );
    /// let filtered_events = client.list_events(Some(request)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_events(
        &self,
        request: Option<ListEventsRequest>,
    ) -> Result<ListEventsResponse> {
        let request = request.unwrap_or_default();
        let region = self.config.region()?;
        let mut url = Url::parse(&format!("{}/events", region.endpoint()))
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
}
