mod use_cases;

use identify_domain::DomainError;
pub use use_cases::*;

pub type Result<T> = std::result::Result<T, DomainError>;
