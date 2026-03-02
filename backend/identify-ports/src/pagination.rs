use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ContinuationToken {
    /// UUID of the last item returned in the previous response.
    pub start_from: Uuid,
}

#[derive(Debug, Clone)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub continuation_token: Option<ContinuationToken>,
}

impl<T> PaginatedResponse<T> {
    pub fn new(
        items: Vec<T>,
        get_continuation_token: impl FnOnce(&[T]) -> Option<ContinuationToken>,
    ) -> Self {
        PaginatedResponse {
            continuation_token: get_continuation_token(&items),
            items,
        }
    }
}
