use thiserror::Error;

pub mod api;
pub mod storage;

pub type Result<T> = std::result::Result<T, InfrastructureError>;

#[derive(Debug, Error)]
pub enum InfrastructureError {}
