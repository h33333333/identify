mod claims;

use crate::{InfrastructureError, Result};
use std::{sync::OnceLock, time::Duration};

use chrono::Utc;
use identify_domain::User;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use uuid::Uuid;

static JWT_KEYS: OnceLock<(EncodingKey, DecodingKey)> = OnceLock::new();

const ALG: Algorithm = Algorithm::RS512;

#[derive(Debug)]
pub struct JwtClient {
    encoding_key: &'static EncodingKey,
    decoding_key: &'static DecodingKey,
    token_ttl: Duration,
    validation: Validation,
}

impl JwtClient {
    /// Creates a new client using the already existing keys (or initializes them first using the provided
    /// raw keys).
    ///
    /// # Safety
    ///
    /// It's assumed that the keys won't change after the first call to [JwtClient::new].
    ///
    /// Providing different keys after the first call **WILL NOT** update the loaded keys.
    pub fn new(
        rsa_private_pem_key: &[u8],
        rsa_public_pem_key: &[u8],
        token_ttl: Duration,
    ) -> Result<Self> {
        let (encoding_key, decoding_key) =
            Self::get_or_init_keys(rsa_private_pem_key, rsa_public_pem_key)?;

        Ok(JwtClient {
            encoding_key,
            decoding_key,
            token_ttl,
            validation: Self::get_validation(ALG),
        })
    }

    /// Issues a new JWT token for the provided user.
    pub fn issue_for_user(&self, user: &User) -> Result<String> {
        let header = jsonwebtoken::Header::new(ALG);

        let now = Utc::now();

        let expires_at = now + self.token_ttl;

        let claims = claims::Claims::new(claims::NewClaimsAttrs {
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            sub: user.id().to_uuid().to_string().into(),
            meta: claims::ClaimsMetadata::new(claims::NewClaimsMetadataAttrs {
                first_name: user.first_name().into(),
                last_name: user.last_name().as_ref().map(Into::into),
            }),
        });

        let token = jsonwebtoken::encode(&header, &claims, self.encoding_key)
            .map_err(|e| {
            InfrastructureError::client_with_message(
                e,
                "Failed to sign a JWT token",
            )
        })?;

        Ok(token)
    }

    /// Decodes and validates the provided token.
    ///
    /// Returns the UUID of the user the token was issued for.
    pub fn decode(&self, token: &str) -> Result<uuid::Uuid> {
        let token = jsonwebtoken::decode::<claims::Claims>(
            token,
            self.decoding_key,
            &self.validation,
        )?;

        Uuid::parse_str(token.claims.sub().as_ref()).map_err(|e| {
            InfrastructureError::client_with_message(
                e,
                "Sub in a receive JWT token is not a valid UUID",
            )
        })
    }

    /// This helper is needed because [OnceLock::get_or_try_init] is still unstable.
    fn get_or_init_keys(
        rsa_private_pem_key: &[u8],
        rsa_public_pem_key: &[u8],
    ) -> Result<(&'static EncodingKey, &'static DecodingKey)> {
        let (encoding_key, decoding_key) = if let Some(keys) = JWT_KEYS.get() {
            keys
        } else {
            let encoding_key = EncodingKey::from_rsa_pem(rsa_private_pem_key)
                .map_err(|e| {
                InfrastructureError::internal_with_message(
                    e,
                    "Failed to parse RSA private key",
                )
            })?;

            let decoding_key = DecodingKey::from_rsa_pem(rsa_public_pem_key)
                .map_err(|e| {
                    InfrastructureError::internal_with_message(
                        e,
                        "Failed to parse RSA public key",
                    )
                })?;

            JWT_KEYS.get_or_init(|| (encoding_key, decoding_key))
        };

        Ok((encoding_key, decoding_key))
    }

    /// Returns a new validation config.
    fn get_validation(algorithm: Algorithm) -> Validation {
        let mut config = Validation::new(algorithm);
        config.set_audience(&[claims::AUDIENCE]);
        config.set_required_spec_claims(&["exp", "aud", "sub"]);
        config
    }
}
