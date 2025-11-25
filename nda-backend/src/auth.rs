use bcrypt::{hash, verify, DEFAULT_COST};

/// Authentication utilities for password hashing and verification.
pub struct Auth;

impl Auth {
    /// Hash a password using bcrypt with default cost (12).
    /// 
    /// # Arguments
    /// 
    /// * `password` - Plain text password to hash
    /// 
    /// # Returns
    /// 
    /// Returns `Result<String, bcrypt::BcryptError>` containing the hashed password.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let hashed = Auth::hash_password("my_secure_password").unwrap();
    /// ```
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }
    
    /// Verify a password against its hash.
    /// 
    /// # Arguments
    /// 
    /// * `password` - Plain text password to verify
    /// * `hash` - Previously hashed password to compare against
    /// 
    /// # Returns
    /// 
    /// Returns `Result<bool, bcrypt::BcryptError>` indicating whether the password matches.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let is_valid = Auth::verify_password("my_password", &hashed_password).unwrap();
    /// ```
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }
}