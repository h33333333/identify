use uuid::Uuid;

pub mod group;
pub mod system_settings;
pub mod user;

pub const UUID_NAMESPACE: Uuid = Uuid::from_bytes(*b"identify-backend");
