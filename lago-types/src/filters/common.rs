pub trait ListFilters {
    fn to_query_params(&self) -> Vec<(&str, String)>;

    fn has_filters(&self) -> bool {
        !self.to_query_params().is_empty()
    }
}