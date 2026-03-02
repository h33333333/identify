use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SetupRequest {
    /// First name of the user.
    pub first_name: String,
    /// Last name of the user.
    pub last_name: Option<String>,
    /// Email of the user.
    pub email: String,
    /// Plaintext password.
    pub password: String,
    /// Name of the admin group this used will be assigned to.
    pub admin_group_name: String,
}
