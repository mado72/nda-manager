//! # JWT Authentication Module
//! 
//! This module provides JSON Web Token (JWT) authentication for the NDA backend.
//! It implements industry best practices for secure token generation, validation,
//! and revocation.
//! 
//! ## Features
//! 
//! - **Access Tokens**: Short-lived tokens (15 minutes) for API authentication
//! - **Refresh Tokens**: Long-lived tokens (7 days) for token renewal
//! - **Token Revocation**: Blacklist system for immediate token invalidation
//! - **Claims Validation**: Automatic expiration and signature verification
//! - **Secure Storage**: Cryptographically signed tokens with secret key
//! 
//! ## Token Structure
//! 
//! JWT tokens contain the following claims:
//! - `sub` (Subject): User ID
//! - `email`: User email address
//! - `roles`: Array of user roles (client, partner)
//! - `iat` (Issued At): Token creation timestamp
//! - `exp` (Expiration): Token expiration timestamp
//! - `jti` (JWT ID): Unique token identifier for revocation
//! 
//! ## Security Best Practices
//! 
//! - Tokens are signed with HS256 algorithm
//! - Secret key must be at least 32 characters
//! - Access tokens expire after 15 minutes
//! - Refresh tokens expire after 7 days
//! - Revoked tokens are stored in memory blacklist
//! - All tokens are validated on each request

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use utoipa::ToSchema;

/// JWT Claims structure containing user information and token metadata.
/// 
/// This structure is embedded in all JWT tokens and validated on each request.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Claims {
    /// Subject (User ID)
    pub sub: String,
    
    /// User email address
    pub email: String,
    
    /// User roles (e.g., ["client"], ["partner"], or ["client", "partner"])
    pub roles: Vec<String>,
    
    /// Issued At timestamp (Unix timestamp)
    pub iat: i64,
    
    /// Expiration timestamp (Unix timestamp)
    pub exp: i64,
    
    /// JWT ID - unique identifier for token revocation
    pub jti: String,
}

impl Claims {
    /// Create new JWT claims with specified expiration time.
    /// 
    /// # Arguments
    /// 
    /// * `user_id` - Unique user identifier
    /// * `email` - User email address
    /// * `roles` - User roles for authorization
    /// * `expires_in_minutes` - Token lifetime in minutes
    /// 
    /// # Returns
    /// 
    /// Returns a new `Claims` instance with generated timestamps and unique JWT ID.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use nda_backend::jwt::Claims;
    /// 
    /// let claims = Claims::new(
    ///     "user-123".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec!["client".to_string()],
    ///     15, // 15 minutes
    /// );
    /// ```
    pub fn new(user_id: String, email: String, roles: Vec<String>, expires_in_minutes: i64) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::minutes(expires_in_minutes);
        
        Self {
            sub: user_id,
            email,
            roles,
            iat: now.timestamp(),
            exp: expiration.timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
        }
    }
}

/// Token blacklist for managing revoked tokens.
/// 
/// This structure maintains an in-memory set of revoked JWT IDs.
/// Tokens are checked against this blacklist during validation.
/// 
/// ## Thread Safety
/// 
/// Uses `Arc<RwLock<HashSet>>` for thread-safe concurrent access.
#[derive(Clone)]
pub struct TokenBlacklist {
    revoked: Arc<RwLock<HashSet<String>>>,
}

impl TokenBlacklist {
    /// Create a new empty token blacklist.
    pub fn new() -> Self {
        Self {
            revoked: Arc::new(RwLock::new(HashSet::new())),
        }
    }
    
    /// Add a token to the blacklist.
    /// 
    /// # Arguments
    /// 
    /// * `jti` - JWT ID to revoke
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use nda_backend::jwt::TokenBlacklist;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let blacklist = TokenBlacklist::new();
    ///     blacklist.revoke("token-id-123").await;
    /// }
    /// ```
    pub async fn revoke(&self, jti: &str) {
        self.revoked.write().await.insert(jti.to_string());
    }
    
    /// Check if a token is revoked.
    /// 
    /// # Arguments
    /// 
    /// * `jti` - JWT ID to check
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the token is revoked, `false` otherwise.
    pub async fn is_revoked(&self, jti: &str) -> bool {
        self.revoked.read().await.contains(jti)
    }
    
    /// Get the total number of revoked tokens.
    /// 
    /// Useful for monitoring and debugging.
    pub async fn count(&self) -> usize {
        self.revoked.read().await.len()
    }
    
    /// Clear all revoked tokens.
    /// 
    /// Should be used carefully, typically only for maintenance or testing.
    pub async fn clear(&self) {
        self.revoked.write().await.clear();
    }
}

impl Default for TokenBlacklist {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate an access token (short-lived, 15 minutes).
/// 
/// Access tokens are used for authenticating API requests.
/// They should be stored in memory (not localStorage) on the client.
/// 
/// # Arguments
/// 
/// * `user_id` - Unique user identifier
/// * `email` - User email address
/// * `roles` - User roles for authorization
/// * `secret` - JWT secret key (min 32 characters recommended)
/// 
/// # Returns
/// 
/// Returns `Result<String, jsonwebtoken::errors::Error>` containing the JWT token.
/// 
/// # Examples
/// 
/// ```rust
/// use nda_backend::jwt::generate_access_token;
/// 
/// let token = generate_access_token(
///     "user-123",
///     "user@example.com",
///     vec!["client".to_string()],
///     "your-super-secret-key-min-32-chars",
/// ).unwrap();
/// ```
pub fn generate_access_token(
    user_id: &str,
    email: &str,
    roles: Vec<String>,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(
        user_id.to_string(),
        email.to_string(),
        roles,
        15, // 15 minutes
    );
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Generate a refresh token (long-lived, 7 days).
/// 
/// Refresh tokens are used to obtain new access tokens without re-authentication.
/// They should be stored securely (HttpOnly cookies or secure storage).
/// 
/// # Arguments
/// 
/// * `user_id` - Unique user identifier
/// * `email` - User email address
/// * `roles` - User roles for authorization
/// * `secret` - JWT secret key (min 32 characters recommended)
/// 
/// # Returns
/// 
/// Returns `Result<String, jsonwebtoken::errors::Error>` containing the JWT token.
/// 
/// # Examples
/// 
/// ```rust
/// use nda_backend::jwt::generate_refresh_token;
/// 
/// let token = generate_refresh_token(
///     "user-123",
///     "user@example.com",
///     vec!["client".to_string()],
///     "your-super-secret-key-min-32-chars",
/// ).unwrap();
/// ```
pub fn generate_refresh_token(
    user_id: &str,
    email: &str,
    roles: Vec<String>,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(
        user_id.to_string(),
        email.to_string(),
        roles,
        10080, // 7 days (7 * 24 * 60)
    );
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Validate and decode a JWT token.
/// 
/// This function verifies the token signature and checks expiration.
/// 
/// # Arguments
/// 
/// * `token` - JWT token string
/// * `secret` - JWT secret key used for signing
/// 
/// # Returns
/// 
/// Returns `Result<Claims, jsonwebtoken::errors::Error>` containing the decoded claims.
/// 
/// # Errors
/// 
/// Returns error if:
/// - Token signature is invalid
/// - Token is expired
/// - Token format is invalid
/// 
/// # Examples
/// 
/// ```rust
/// use nda_backend::jwt::validate_token;
/// 
/// match validate_token(&token, "secret-key") {
///     Ok(claims) => println!("User ID: {}", claims.sub),
///     Err(e) => eprintln!("Invalid token: {}", e),
/// }
/// ```
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}

/// Extract token from Authorization header.
/// 
/// Expects format: "Bearer <token>"
/// 
/// # Arguments
/// 
/// * `auth_header` - Authorization header value
/// 
/// # Returns
/// 
/// Returns `Option<&str>` containing the token, or `None` if format is invalid.
/// 
/// # Examples
/// 
/// ```rust
/// use nda_backend::jwt::extract_token_from_header;
/// 
/// let header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
/// if let Some(token) = extract_token_from_header(header) {
///     println!("Token: {}", token);
/// }
/// ```
pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") && auth_header.len() > 7 {
        Some(&auth_header[7..])
    } else {
        None
    }
}

/// Validate JWT token from Authorization header and check blacklist.
/// 
/// This function extracts and validates a JWT token from the Authorization header,
/// checking both the token signature/expiration and whether it has been revoked.
/// 
/// # Arguments
/// 
/// * `auth_header` - Optional Authorization header value
/// * `jwt_secret` - JWT secret key for validation
/// * `blacklist` - Token blacklist to check for revoked tokens
/// 
/// # Returns
/// 
/// Returns `Result<Claims, &'static str>` containing:
/// - `Ok(Claims)` - Valid, non-revoked token claims
/// - `Err(&str)` - Error message describing validation failure
/// 
/// # Errors
/// 
/// Returns error if:
/// - Authorization header is missing
/// - Token format is invalid (not "Bearer <token>")
/// - Token signature is invalid
/// - Token is expired
/// - Token has been revoked (in blacklist)
/// 
/// # Examples
/// 
/// ```rust
/// use nda_backend::jwt::{validate_auth_header, TokenBlacklist};
/// 
/// #[tokio::main]
/// async fn main() {
///     let blacklist = TokenBlacklist::new();
///     let header = Some("Bearer eyJhbGc...".to_string());
///     
///     match validate_auth_header(header.as_deref(), "secret", &blacklist).await {
///         Ok(claims) => println!("User: {}", claims.sub),
///         Err(e) => eprintln!("Auth error: {}", e),
///     }
/// }
/// ```
pub async fn validate_auth_header(
    auth_header: Option<&str>,
    jwt_secret: &str,
    blacklist: &TokenBlacklist,
) -> Result<Claims, &'static str> {
    // Check if Authorization header exists
    let auth_header = auth_header.ok_or("Missing Authorization header")?;
    
    // Extract token from "Bearer <token>" format
    let token = extract_token_from_header(auth_header)
        .ok_or("Invalid Authorization header format")?;
    
    // Validate token signature and expiration
    let claims = validate_token(token, jwt_secret)
        .map_err(|_| "Invalid or expired token")?;
    
    // Check if token has been revoked
    if blacklist.is_revoked(&claims.jti).await {
        return Err("Token has been revoked");
    }
    
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "test-secret-key-at-least-32-characters-long";

    #[test]
    fn test_generate_and_validate_access_token() {
        let token = generate_access_token(
            "user-123",
            "test@example.com",
            vec!["client".to_string()],
            TEST_SECRET,
        )
        .unwrap();

        let claims = validate_token(&token, TEST_SECRET).unwrap();
        
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.roles, vec!["client".to_string()]);
    }

    #[test]
    fn test_generate_and_validate_refresh_token() {
        let token = generate_refresh_token(
            "user-456",
            "refresh@example.com",
            vec!["partner".to_string()],
            TEST_SECRET,
        )
        .unwrap();

        let claims = validate_token(&token, TEST_SECRET).unwrap();
        
        assert_eq!(claims.sub, "user-456");
        assert_eq!(claims.email, "refresh@example.com");
        assert_eq!(claims.roles, vec!["partner".to_string()]);
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid.token.here", TEST_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_secret() {
        let token = generate_access_token(
            "user-789",
            "wrong@example.com",
            vec!["client".to_string()],
            TEST_SECRET,
        )
        .unwrap();

        let result = validate_token(&token, "wrong-secret");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_token_from_header() {
        let header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test";
        let token = extract_token_from_header(header);
        assert_eq!(token, Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test"));

        let invalid_header = "InvalidFormat";
        assert_eq!(extract_token_from_header(invalid_header), None);
    }

    #[tokio::test]
    async fn test_token_blacklist() {
        let blacklist = TokenBlacklist::new();
        let jti = "test-token-id";

        assert!(!blacklist.is_revoked(jti).await);
        
        blacklist.revoke(jti).await;
        assert!(blacklist.is_revoked(jti).await);
        
        assert_eq!(blacklist.count().await, 1);
        
        blacklist.clear().await;
        assert_eq!(blacklist.count().await, 0);
    }
}
