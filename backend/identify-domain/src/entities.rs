use uuid::Uuid;

pub mod user;

pub const UUID_NAMESPACE: Uuid = Uuid::from_bytes(*b"identify-backend");
