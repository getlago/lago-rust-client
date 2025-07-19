/// A trait for types that can be converted to query parameters for list filtering operations.
///
/// This trait provides a common interface for filter structs that need to be serialized
/// into HTTP query parameters for API requests.
pub trait ListFilters {
    /// Converts the filter struct into a vector of query parameter key-value pairs.
    ///
    /// # Returns
    /// A vector of tuples, where each tuple contains a parameter name and its string value.
    fn to_query_params(&self) -> Vec<(&str, String)>;

    /// Checks if any filters are currently set.
    ///
    /// # Returns
    /// `true` if there are filters to apply, `false` if no filters are set.
    fn has_filters(&self) -> bool {
        !self.to_query_params().is_empty()
    }
}
