use argon2::{
    Argon2, PasswordHasher as _,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{InfrastructureError, Result};

/// Hashes the provided password using [argon2] and returns a ready-to-be-stored hash.
///
/// Generates a random password salt internally.
pub fn hash_password(input: impl AsRef<[u8]>) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let hasher = Argon2::default();

    let hash = hasher
        .hash_password(input.as_ref(), &salt)
        .map_err(|err| {
            InfrastructureError::client_with_message(
                eyre::eyre!(err),
                "Failed to hash a password using argon2",
            )
        })?
        .to_string();

    Ok(hash)
}
