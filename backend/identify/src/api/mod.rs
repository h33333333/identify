use std::sync::Arc;

use identify_macros::gen_model;
use sqlx::SqlitePool;

pub mod services;

/// Alias to simplify signatures.
pub type ApiState = Arc<InnerApiState>;

gen_model! {
    /// API-wide state.
    ///
    /// Each service is expected to extract only the parts it needs.
    pub struct InnerApiState {
         #[get(copy)]
         pool: &'static SqlitePool,
    }

    pub struct NewInnerApiStateAttrs;
}

impl InnerApiState {
    pub fn new(attrs: NewInnerApiStateAttrs) -> Self {
        InnerApiState { pool: attrs.pool }
    }
}
