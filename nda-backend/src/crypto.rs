//! # Crypto Module
//! 
//! This module provides cryptographic functionality for the NDA backend system.
//! It implements AES-256-GCM encryption for secure content protection with the following features:
//! 
//! - **Encryption Algorithm**: AES-256-GCM (Galois/Counter Mode)
//! - **Key Size**: 256-bit keys generated using cryptographically secure random number generation
//! - **Authentication**: Built-in authentication and integrity verification via GCM mode
//! - **Nonce**: 96-bit (12 bytes) random nonce generated for each encryption operation
//! 
//! ## Security Model
//! 
//! - Keys are generated using OS-provided cryptographically secure random number generator
//! - Each encryption operation uses a unique, randomly generated nonce
//! - The nonce is prepended to the ciphertext for storage/transmission
//! - Base64 encoding is used for safe text representation of binary data
//! 
//! ## Usage Example
//! 
//! ```rust
//! use crate::crypto::{generate_key, encrypt_content, decrypt_content};
//! 
//! // Generate a new encryption key
//! let key = generate_key();
//! 
//! // Encrypt sensitive content
//! let encrypted = encrypt_content("Confidential NDA content", &key)?;
//! 
//! // Decrypt when needed
//! let decrypted = decrypt_content(&encrypted, &key)?;
//! assert_eq!(decrypted, "Confidential NDA content");
//! ```
//! 
//! ## Error Handling
//! 
//! All cryptographic operations return `Result` types with descriptive error messages.
//! Common error scenarios include invalid keys, corrupted data, and encoding issues.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

/// Represents cryptographic operation errors.
/// 
/// This error type encapsulates all possible failures that can occur during
/// cryptographic operations such as key generation, encryption, and decryption.
/// 
/// # Common Error Scenarios
/// 
/// - **Key Decode Errors**: Invalid Base64 format in encryption keys
/// - **Encryption Failures**: Algorithm-level encryption errors (rare)
/// - **Decryption Failures**: Wrong key, corrupted data, or tampering detected
/// - **Data Format Errors**: Invalid encrypted data format or insufficient length
/// - **Encoding Errors**: UTF-8 conversion failures in decrypted content
/// 
/// # Error Handling Best Practices
/// 
/// ```rust
/// match encrypt_content("data", &key) {
///     Ok(encrypted) => {
///         // Handle successful encryption
///         println!("Encryption successful");
///     }
///     Err(e) => {
///         // Log error details for debugging (avoid logging sensitive data)
///         eprintln!("Encryption failed: {}", e);
///         // Return appropriate error response to user
///     }
/// }
/// ```
#[derive(Debug)]
pub struct CryptoError(String);

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cryptographic operation failed: {}", self.0)
    }
}

impl std::error::Error for CryptoError {}

/// Generates a new cryptographically secure 256-bit AES key.
/// 
/// This function creates a new encryption key using the OS-provided cryptographically 
/// secure random number generator. The key is suitable for use with AES-256-GCM encryption.
/// 
/// # Returns
/// 
/// Returns a Base64-encoded string representation of the 256-bit key. This string can be
/// safely stored and transmitted as it contains only printable ASCII characters.
/// 
/// # Examples
/// 
/// ```rust
/// let encryption_key = generate_key();
/// // The key is now ready for use with encrypt_content() and decrypt_content()
/// ```
/// 
/// # Security Notes
/// 
/// - The generated key has 256 bits of entropy
/// - Uses the operating system's cryptographically secure random number generator
/// - Each call generates a unique key
/// - Keys should be stored securely and not logged or transmitted in plain text
pub fn generate_key() -> String {
    let key = Aes256Gcm::generate_key(OsRng);
    general_purpose::STANDARD.encode(key)
}

/// Encrypts text content using AES-256-GCM encryption.
/// 
/// This function takes plain text content and encrypts it using the provided key with
/// AES-256-GCM algorithm. Each encryption operation uses a unique random nonce for security.
/// 
/// # Parameters
/// 
/// * `content` - The plain text content to encrypt. Must be valid UTF-8.
/// * `key` - Base64-encoded encryption key (must be generated using `generate_key()` or equivalent)
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(String)` - Base64-encoded encrypted data (nonce + ciphertext + authentication tag)
/// - `Err(Box<dyn std::error::Error>)` - Encryption error (invalid key, encryption failure, etc.)
/// 
/// # Examples
/// 
/// ```rust
/// let key = generate_key();
/// let encrypted = encrypt_content("Sensitive information", &key)?;
/// // The encrypted string can now be safely stored or transmitted
/// ```
/// 
/// # Security Notes
/// 
/// - Uses AES-256-GCM which provides both confidentiality and authenticity
/// - A random 96-bit nonce is generated for each encryption operation
/// - The nonce is prepended to the ciphertext for storage
/// - The same plaintext will produce different ciphertext on each encryption
/// 
/// # Errors
/// 
/// This function may return errors for:
/// - Invalid or corrupted Base64 key format
/// - Encryption algorithm failures
/// - Memory allocation issues
pub fn encrypt_content(content: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Decode the Base64-encoded key into raw bytes
    let key_bytes = general_purpose::STANDARD.decode(key)
        .map_err(|e| CryptoError(format!("Failed to decode key: {}", e)))?;
    
    // Create AES-256-GCM cipher with the decoded key
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate a random 96-bit (12 bytes) nonce for this encryption operation
    let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the content using AES-256-GCM
    let ciphertext = cipher.encrypt(nonce, content.as_bytes())
        .map_err(|e| CryptoError(format!("Encryption failed: {:?}", e)))?;
    
    // Combine nonce + ciphertext for storage (nonce is needed for decryption)
    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);
    
    // Encode the combined data as Base64 for safe text representation
    Ok(general_purpose::STANDARD.encode(encrypted_data))
}

/// Decrypts content that was encrypted using `encrypt_content()`.
/// 
/// This function takes Base64-encoded encrypted content and decrypts it using the provided key.
/// It verifies the authenticity and integrity of the data using AES-256-GCM's built-in authentication.
/// 
/// # Parameters
/// 
/// * `encrypted_content` - Base64-encoded encrypted data (produced by `encrypt_content()`)
/// * `key` - Base64-encoded decryption key (same key used for encryption)
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(String)` - The original plain text content
/// - `Err(Box<dyn std::error::Error>)` - Decryption error (invalid key, corrupted data, etc.)
/// 
/// # Examples
/// 
/// ```rust
/// let key = generate_key();
/// let encrypted = encrypt_content("Secret message", &key)?;
/// let decrypted = decrypt_content(&encrypted, &key)?;
/// assert_eq!(decrypted, "Secret message");
/// ```
/// 
/// # Security Notes
/// 
/// - Automatically verifies data authenticity and integrity
/// - Will fail if the data has been tampered with or corrupted
/// - Will fail if an incorrect key is used
/// - The nonce is automatically extracted from the encrypted data
/// 
/// # Errors
/// 
/// This function may return errors for:
/// - Invalid or corrupted Base64 encoded data
/// - Wrong decryption key
/// - Corrupted or tampered encrypted data
/// - Data too short to contain valid nonce
/// - UTF-8 conversion errors in the decrypted content
pub fn decrypt_content(encrypted_content: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Decode the Base64-encoded key into raw bytes
    let key_bytes = general_purpose::STANDARD.decode(key)
        .map_err(|e| CryptoError(format!("Failed to decode key: {}", e)))?;
    
    // Create AES-256-GCM cipher with the decoded key
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Decode the Base64-encoded encrypted data
    let encrypted_data = general_purpose::STANDARD.decode(encrypted_content)
        .map_err(|e| CryptoError(format!("Failed to decode encrypted data: {}", e)))?;
    
    // Ensure the encrypted data is long enough to contain a nonce (12 bytes minimum)
    if encrypted_data.len() < 12 {
        return Err(Box::new(CryptoError("Invalid encrypted data: insufficient length".to_string())));
    }

    // Split the encrypted data into nonce (first 12 bytes) and ciphertext (remaining bytes)
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt the ciphertext using the nonce and verify authenticity
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError(format!("Decryption failed: {:?}", e)))?;
    
    // Convert the decrypted bytes back to a UTF-8 string
    String::from_utf8(plaintext)
        .map_err(|e| Box::new(CryptoError(format!("UTF-8 conversion error: {}", e))) as Box<dyn std::error::Error>)
}