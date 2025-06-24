use crate::config::AppConfig;
use crate::utils::error::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};
use rand::rngs::OsRng;
// For secure salt generation
use serde::{Deserialize, Serialize};

/// JWT Claims structure used for encoding and decoding tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - user ID
    pub sub: i32,
    /// Expiration timestamp (seconds since epoch)
    pub exp: usize,
}

/// Hash a plaintext password using Argon2.
///
/// # Arguments
/// * `password` - The user's plaintext password
///
/// # Returns
/// * `Ok(String)` with the password hash
/// * `Err(AppError::InternalServerError(String))` on failure
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))
        .map(|hash| hash.to_string())
}

/// Verify a plaintext password against a hashed password.
///
/// # Arguments
/// * `hash` - The stored password hash
/// * `password` - The plaintext password to verify
///
/// # Returns
/// * `Ok(true)` if passwords match
/// * `Ok(false)` if they don't
/// * `Err(AppError::InternalServerError)` on failure
pub fn verify_password(hash: &str, password: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::InternalServerError("Invalid password hash".to_string()))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Create a signed JWT for a user.
///
/// # Arguments
/// * `user_id` - The ID of the authenticated user
/// * `config` - Application configuration with JWT secret
///
/// # Returns
/// * `Ok(String)` - JWT token
/// * `Err(AppError::InternalServerError)` on failure
pub fn create_jwt(user_id: i32, config: &AppConfig) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(config.jwt_expiration))
        .ok_or_else(|| AppError::InternalServerError("Failed to calculate expiration".to_string()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };
    let header = Header::default();

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError("Failed to create JWT".to_string()))
}

/// Decode and validate a JWT token.
///
/// # Arguments
/// * `token` - The JWT string
/// * `config` - App configuration with JWT secret
///
/// # Returns
/// * `Ok(Claims)` - If token is valid
/// * `Err(AppError::Unauthorized)` - If token is invalid or expired
pub fn decode_jwt(token: &str, config: &AppConfig) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))
}
