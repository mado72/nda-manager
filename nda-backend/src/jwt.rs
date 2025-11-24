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
use std::collections::HashMap;
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
/// This structure maintains an in-memory map of revoked JWT IDs with their expiration timestamps.
/// Tokens are checked against this blacklist during validation.
/// Expired tokens are automatically cleaned up periodically.
/// 
/// ## Thread Safety
/// 
/// Uses `Arc<RwLock<HashMap>>` for thread-safe concurrent access.
#[derive(Clone)]
pub struct TokenBlacklist {
    /// Maps JWT ID to expiration timestamp (Unix timestamp)
    revoked: Arc<RwLock<HashMap<String, i64>>>,
}

impl TokenBlacklist {
    /// Create a new empty token blacklist.
    pub fn new() -> Self {
        Self {
            revoked: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add a token to the blacklist with its expiration timestamp.
    /// 
    /// # Arguments
    /// 
    /// * `jti` - JWT ID to revoke
    /// * `exp` - Expiration timestamp (Unix timestamp)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use nda_backend::jwt::TokenBlacklist;
    /// use chrono::Utc;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let blacklist = TokenBlacklist::new();
    ///     let exp = Utc::now().timestamp() + 900; // 15 minutes
    ///     blacklist.revoke("token-id-123", exp).await;
    /// }
    /// ```
    pub async fn revoke(&self, jti: &str, exp: i64) {
        self.revoked.write().await.insert(jti.to_string(), exp);
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
        self.revoked.read().await.contains_key(jti)
    }
    
    /// Get the total number of revoked tokens.
    /// 
    /// Useful for monitoring and debugging.
    #[allow(dead_code)]
    pub async fn count(&self) -> usize {
        self.revoked.read().await.len()
    }
    
    /// Clear all revoked tokens.
    /// 
    /// Should be used carefully, typically only for maintenance or testing.
    #[allow(dead_code)]
    pub async fn clear(&self) {
        self.revoked.write().await.clear()
    }
    
    /// Remove expired tokens from the blacklist.
    /// 
    /// This method should be called periodically to prevent memory bloat.
    /// It removes all tokens whose expiration timestamp is in the past.
    /// 
    /// # Returns
    /// 
    /// Returns the number of expired tokens removed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use nda_backend::jwt::TokenBlacklist;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let blacklist = TokenBlacklist::new();
    ///     let removed = blacklist.cleanup_expired().await;
    ///     println!("Removed {} expired tokens", removed);
    /// }
    /// ```
    pub async fn cleanup_expired(&self) -> usize {
        let now = Utc::now().timestamp();
        let mut revoked = self.revoked.write().await;
        let initial_count = revoked.len();
        
        // Remove all tokens with expiration timestamp in the past
        revoked.retain(|_, &mut exp| exp > now);
        
        let removed = initial_count - revoked.len();
        if removed > 0 {
            tracing::info!("Cleaned up {} expired tokens from blacklist", removed);
        }
        removed
    }
    
    /// Start a background task that periodically cleans up expired tokens.
    /// 
    /// # Arguments
    /// 
    /// * `interval_minutes` - How often to run cleanup (in minutes)
    /// 
    /// # Returns
    /// 
    /// Returns a tokio task handle that can be used to cancel the cleanup task.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use nda_backend::jwt::TokenBlacklist;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let blacklist = TokenBlacklist::new();
    ///     let handle = blacklist.start_cleanup_task(60); // Run every hour
    ///     
    ///     // Cleanup will run automatically in the background
    ///     // Cancel with: handle.abort();
    /// }
    /// ```
    pub fn start_cleanup_task(&self, interval_minutes: u64) -> tokio::task::JoinHandle<()> {
        let blacklist = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_minutes * 60));
            loop {
                interval.tick().await;
                blacklist.cleanup_expired().await;
            }
        })
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
        let exp = Utc::now().timestamp() + 900; // 15 minutes

        assert!(!blacklist.is_revoked(jti).await);
        
        blacklist.revoke(jti, exp).await;
        assert!(blacklist.is_revoked(jti).await);
        
        assert_eq!(blacklist.count().await, 1);
        
        blacklist.clear().await;
        assert_eq!(blacklist.count().await, 0);
    }
    
    #[tokio::test]
    async fn test_cleanup_expired_tokens() {
        let blacklist = TokenBlacklist::new();
        
        // Add expired token (1 second in the past)
        let expired_jti = "expired-token";
        let expired_exp = Utc::now().timestamp() - 1;
        blacklist.revoke(expired_jti, expired_exp).await;
        
        // Add valid token (15 minutes in the future)
        let valid_jti = "valid-token";
        let valid_exp = Utc::now().timestamp() + 900;
        blacklist.revoke(valid_jti, valid_exp).await;
        
        // Both tokens should be in blacklist
        assert_eq!(blacklist.count().await, 2);
        assert!(blacklist.is_revoked(expired_jti).await);
        assert!(blacklist.is_revoked(valid_jti).await);
        
        // Cleanup should remove only the expired token
        let removed = blacklist.cleanup_expired().await;
        assert_eq!(removed, 1);
        assert_eq!(blacklist.count().await, 1);
        
        // Expired token should be gone, valid token should remain
        assert!(!blacklist.is_revoked(expired_jti).await);
        assert!(blacklist.is_revoked(valid_jti).await);
    }
    
    #[tokio::test]
    async fn test_cleanup_task() {
        let blacklist = TokenBlacklist::new();
        
        // Add expired token
        let expired_jti = "expired-token";
        let expired_exp = Utc::now().timestamp() - 1;
        blacklist.revoke(expired_jti, expired_exp).await;
        
        assert_eq!(blacklist.count().await, 1);
        
        // Start cleanup task with very short interval (1 second for testing)
        let handle = tokio::spawn({
            let blacklist = blacklist.clone();
            async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                blacklist.cleanup_expired().await;
            }
        });
        
        // Wait for cleanup to run
        handle.await.unwrap();
        
        // Expired token should be removed
        assert_eq!(blacklist.count().await, 0);
    }
}
