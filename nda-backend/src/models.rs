//! # Data Models Module
//! 
//! This module defines all data structures used throughout the NDA backend application.
//! It includes database entities, API request/response models, and data transformation utilities.
//! 
//! ## Model Categories
//! 
//! ### Database Entities
//! Core business entities that are persisted in the SQLite database:
//! - [`User`] - User accounts with Stellar blockchain integration
//! - [`Process`] - Encrypted NDA processes with metadata
//! - [`ProcessShare`] - Blockchain-recorded sharing events
//! - [`ProcessAccess`] - Access audit logs for compliance
//! - [`ProcessAccessWithDetails`] - Enriched access records with denormalized data
//! 
//! ### API Request Models
//! Structures for deserializing incoming HTTP requests:
//! - [`RegisterRequest`] - User registration payload
//! - [`LoginRequest`] - User authentication payload
//! - [`CreateProcessRequest`] - Process creation payload
//! - [`ShareProcessRequest`] - Process sharing payload
//! - [`AccessProcessRequest`] - Process access payload
//! 
//! ### API Response Models
//! Structures for serializing outgoing HTTP responses:
//! - [`UserResponse`] - User data without sensitive fields
//! - [`ProcessResponse`] - Process metadata without encrypted content
//! - [`ProcessAccessResponse`] - Decrypted process content for authorized access
//! 
//! ## Security Considerations
//! 
//! - **Sensitive Data**: Database models contain encrypted fields and secret keys
//! - **Response Filtering**: Response models exclude sensitive data from API responses
//! - **Type Safety**: All models use strong typing to prevent data corruption
//! - **Serialization**: Proper serde attributes ensure safe JSON handling
//! 
//! ## Database Integration
//! 
//! All database entities derive `FromRow` for seamless SQLx integration,
//! enabling type-safe database queries with compile-time verification.
//! 
//! ## Usage Example
//! 
//! ```rust,no_run
//! use nda_backend::models::*;
//! 
//! fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let json_data = r#"{"title": "Sample Process", "content": "Sample content"}"#;
//!     
//!     // Create a process request from JSON
//!     let request: CreateProcessRequest = serde_json::from_str(&json_data)?;
//!     
//!     // Create a sample process (in real code, this would come from database)
//!     let process = Process {
//!         id: "123".to_string(),
//!         client_id: "456".to_string(),
//!         title: "Sample".to_string(),
//!         encrypted_content: "encrypted".to_string(),
//!         encryption_key: "key".to_string(),
//!         status: "active".to_string(),
//!         created_at: chrono::Utc::now(),
//!     };
//!     
//!     // Convert database entity to API response
//!     let response: ProcessResponse = process.into();
//!     
//!     // Serialize response to JSON
//!     let json = serde_json::to_string(&response)?;
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// User account with Stellar blockchain integration.
/// 
/// Represents a user in the NDA system with associated Stellar network credentials.
/// Users can have multiple roles: client (who create and own processes), partner
/// (who receive shared access to processes), or both simultaneously.
/// 
/// # Fields
/// 
/// * `id` - Unique identifier (UUID)
/// * `username` - Unique username for authentication
/// * `name` - Full name or display name of the user
/// * `stellar_public_key` - Stellar network public key for blockchain operations
/// * `stellar_secret_key` - Stellar network secret key (encrypted in production)
/// * `roles` - JSON string containing user roles: `["client"]`, `["partner"]`, or `["client","partner"]`
/// * `created_at` - Account creation timestamp
/// 
/// # Role System
/// 
/// The `roles` field contains a JSON array of role strings:
/// - `"client"` - Can create and manage NDA processes
/// - `"partner"` - Can access shared processes
/// - Users can have both roles simultaneously for maximum flexibility
/// 
/// # Security Notes
/// 
/// - The `stellar_secret_key` should be encrypted using a Key Management Service (KMS) in production
/// - This model contains sensitive data and should not be directly exposed via APIs
/// - Use [`UserResponse`] for API responses to exclude sensitive fields
/// 
/// # Database Integration
/// 
/// This struct derives `FromRow` for direct SQLx query result mapping.
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub stellar_public_key: String,
    pub stellar_secret_key: String, // In production, use KMS (Key Management Service)
    pub password_hash: String, // Bcrypt hashed password
    pub roles: String, // JSON string: ["client"] | ["partner"] | ["client","partner"]
    pub created_at: DateTime<Utc>,
}

/// Custom Debug implementation that hides sensitive fields.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("name", &self.name)
            .field("stellar_public_key", &self.stellar_public_key)
            .field("stellar_secret_key", &"[REDACTED]")
            .field("password_hash", &"[REDACTED]")
            .field("roles", &self.roles)
            .field("created_at", &self.created_at)
            .finish()
    }
}

/// NDA process with encrypted confidential content.
/// 
/// Represents a Non-Disclosure Agreement process containing encrypted sensitive
/// information that can be selectively shared with partners via blockchain transactions.
/// 
/// # Fields
/// 
/// * `id` - Unique process identifier (UUID)
/// * `client_id` - Reference to the owning client user
/// * `title` - Human-readable process title/description
/// * `description` - Detailed description of the process (required)
/// * `encrypted_content` - AES-256-GCM encrypted confidential content
/// * `encryption_key` - Base64-encoded encryption key for the content
/// * `status` - Process lifecycle status ("active", "completed", etc.)
/// * `created_at` - Process creation timestamp
/// 
/// # Security Model
/// 
/// - Content is encrypted using AES-256-GCM before database storage
/// - Each process has a unique encryption key generated during creation
/// - Only authorized users can decrypt content after blockchain-verified sharing
/// - Access attempts are logged for audit trails and compliance
/// 
/// # Lifecycle
/// 
/// 1. **Creation**: Client creates process with encrypted content
/// 2. **Sharing**: Client shares with partners via Stellar blockchain transaction
/// 3. **Access**: Partners decrypt and access content with proper authorization
/// 4. **Audit**: All access events are logged for compliance reporting
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Process {
    pub id: String,
    pub client_id: String,
    pub title: String,
    pub description: String,
    pub encrypted_content: String,
    pub encryption_key: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Blockchain-recorded process sharing event.
/// 
/// Records when a process has been shared with a partner via a Stellar blockchain
/// transaction, providing immutable proof of sharing for audit and verification.
/// 
/// # Fields
/// 
/// * `id` - Unique sharing record identifier (UUID)
/// * `process_id` - Reference to the shared process
/// * `partner_public_key` - Stellar public key of the recipient partner
/// * `stellar_transaction_hash` - Immutable blockchain transaction hash
/// * `shared_at` - Timestamp when sharing occurred
/// 
/// # Blockchain Integration
/// 
/// - Each sharing event creates a Stellar network transaction
/// - Transaction hash provides cryptographic proof of sharing
/// - Memo field in transaction contains process metadata
/// - Sharing rights can be independently verified on the blockchain
/// 
/// # Compliance Benefits
/// 
/// - Immutable audit trail for regulatory compliance
/// - Cryptographically verifiable sharing permissions
/// - Dispute resolution through blockchain evidence
/// - Time-stamped sharing events for legal requirements
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProcessShare {
    pub id: String,
    pub process_id: String,
    pub partner_public_key: String,
    pub stellar_transaction_hash: String,
    pub shared_at: DateTime<Utc>,
}

/// Process access audit record.
/// 
/// Logs when a partner accesses a shared process, creating a complete
/// audit trail for compliance and monitoring purposes.
/// 
/// # Fields
/// 
/// * `id` - Unique access record identifier (UUID)
/// * `process_id` - Reference to the accessed process
/// * `partner_id` - Reference to the accessing partner user
/// * `accessed_at` - Timestamp when access occurred
/// 
/// # Compliance Features
/// 
/// - Every process access is logged for regulatory requirements
/// - Timestamps provide chronological access history
/// - Failed access attempts are also recorded
/// - Access patterns can be analyzed for security monitoring
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProcessAccess {
    pub id: String,
    pub process_id: String,
    pub partner_id: String,
    pub accessed_at: DateTime<Utc>,
}

/// Enriched process access record with denormalized data.
/// 
/// Extends [`ProcessAccess`] with additional fields for easier reporting
/// and dashboard display without requiring complex joins.
/// 
/// # Fields
/// 
/// * `id` - Unique access record identifier (UUID)
/// * `process_id` - Reference to the accessed process
/// * `partner_id` - Reference to the accessing partner user
/// * `accessed_at` - Timestamp when access occurred
/// * `process_title` - Denormalized process title for display
/// * `process_description` - Denormalized process description for context
/// * `process_status` - Current process status ('active', 'completed', etc.)
/// * `partner_username` - Denormalized partner username for display
/// 
/// # Usage
/// 
/// This model is typically used for:
/// - Client dashboard notifications with detailed process information and status
/// - Compliance reporting interfaces with full context and current status
/// - Access analytics and monitoring with process descriptions and status tracking
/// - Real-time access alerts with meaningful process details and status updates
/// 
/// # Optional Fields
/// 
/// Some fields may be `None` when using LEFT OUTER JOIN queries:
/// - `id`, `partner_id`, `accessed_at`: Optional when no access record exists
/// - `partner_username`: Optional when partner user data is not available
/// 
/// The `process_*` fields are always present as they come from the main processes table.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessAccessWithDetails {
    pub id: Option<String>,
    pub process_id: String,
    pub partner_id: Option<String>,
    pub accessed_at: Option<DateTime<Utc>>,
    pub process_title: String,
    pub process_description: String,
    pub process_status: String,
    pub partner_username: Option<String>,
}

/// User registration request payload.
/// 
/// Contains the necessary information to create a new user account
/// with automatic Stellar blockchain integration.
/// 
/// # Fields
/// 
/// * `username` - Desired unique username
/// * `password` - User password (currently unused in MVP)
/// * `roles` - Array of user roles: `["client"]`, `["partner"]`, or `["client","partner"]`
/// 
/// # Role Validation
/// 
/// - Username must be unique across all users
/// - Roles must contain valid values: "client" and/or "partner"
/// - At least one role must be specified
/// - Password field exists for future authentication enhancement
/// 
/// # Stellar Integration
/// 
/// Upon successful registration:
/// - A Stellar keypair is automatically generated
/// - The account is funded on testnet for immediate use
/// - Stellar credentials are securely stored in the database
/// 
/// # Examples
/// 
/// ```json
/// {
///   "username": "john_doe",
///   "password": "secure_password",
///   "roles": ["client", "partner"]
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub name: String,
    pub password: String,
    pub roles: Vec<String>,
}

/// User authentication request payload.
/// 
/// Simple username-based authentication for the MVP.
/// In production, this would include proper password verification.
/// 
/// # Fields
/// 
/// * `username` - Username for authentication
/// * `password` - Password (currently unused in MVP)
/// 
/// # Security Notes
/// 
/// - Current implementation uses username-only authentication
/// - Password field exists for future security enhancement
/// - Production systems should implement proper password hashing
/// - Consider adding JWT tokens for session management
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Auto login request payload using localStorage information.
/// 
/// Contains the information stored in localStorage for automatic login.
/// This endpoint authenticates users without requiring password re-entry,
/// using the user_name and user_id that are persisted in the frontend.
/// 
/// # Fields
/// 
/// * `user_name` - Username stored in localStorage
/// * `user_id` - User ID stored in localStorage  
/// 
/// # Security Notes
/// 
/// - This endpoint relies on client-side data persistence
/// - Should be used only for convenience, not as primary authentication
/// - Consider implementing session tokens for enhanced security
#[derive(Debug, Deserialize)]
pub struct AutoLoginRequest {
    pub user_name: String,
    pub user_id: String,
}

/// Process creation request payload.
/// 
/// Contains the information needed to create a new encrypted NDA process.
/// The confidential content will be automatically encrypted before storage.
/// 
/// # Fields
/// 
/// * `title` - Human-readable process title/name
/// * `description` - Detailed description of the process (required)
/// * `confidential_content` - Sensitive content to be encrypted
/// * `client_id` - ID of the client creating the process
/// 
/// # Security Processing
/// 
/// After receiving this request:
/// 1. Content is encrypted using AES-256-GCM
/// 2. A unique encryption key is generated
/// 3. Process is associated with the client user
/// 4. Encrypted data is stored in the database
#[derive(Debug, Deserialize)]
pub struct CreateProcessRequest {
    pub title: String,
    pub description: String,
    pub confidential_content: String,
    pub client_id: String,
}

/// Process sharing request payload.
/// 
/// Contains the information needed to share a process with a partner
/// via a Stellar blockchain transaction.
/// 
/// # Fields
/// 
/// * `process_id` - ID of the process to share
/// * `partner_public_key` - Stellar public key of the recipient
/// * `client_username` - Username of the client sharing the process
/// 
/// # Blockchain Integration
/// 
/// This request triggers:
/// 1. Verification of process ownership by client
/// 2. Creation of a Stellar blockchain transaction
/// 3. Recording of the transaction hash for audit
/// 4. Granting of access permissions to the partner
#[derive(Debug, Deserialize)]
pub struct ShareProcessRequest {
    pub process_id: String,
    pub partner_public_key: String,
    pub client_username: String,
}

/// Process access request payload.
/// 
/// Contains the information needed for a partner to access
/// a shared process and decrypt its contents.
/// 
/// # Fields
/// 
/// * `process_id` - ID of the process to access
/// * `partner_public_key` - Stellar public key for verification
/// * `partner_username` - Username of the requesting partner
/// 
/// # Access Control
/// 
/// Before granting access, the system:
/// 1. Verifies the process exists
/// 2. Checks that sharing record exists in database
/// 3. Validates partner credentials
/// 4. Logs the access event for audit
/// 5. Decrypts and returns the content
#[derive(Debug, Deserialize)]
pub struct AccessProcessRequest {
    pub process_id: String,
    pub partner_public_key: String,
    pub partner_username: String,
}

/// User data for API responses (excludes sensitive fields).
/// 
/// Safe representation of user data that excludes sensitive information
/// like the Stellar secret key. Used for all public API responses.
/// 
/// # Fields
/// 
/// * `id` - User identifier
/// * `username` - Public username
/// * `stellar_public_key` - Stellar public key (safe to expose)
/// * `roles` - Array of user roles: `["client"]`, `["partner"]`, or `["client","partner"]`
/// * `created_at` - Account creation timestamp
/// 
/// # Role System
/// 
/// The `roles` field contains an array of role strings:
/// - `"client"` - Can create and manage NDA processes
/// - `"partner"` - Can access shared processes
/// - Users can have both roles simultaneously
/// 
/// # Security Features
/// 
/// - Excludes `stellar_secret_key` for security
/// - Safe for JSON serialization in API responses
/// - Automatically converted from [`User`] via `From` trait
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub name: String,
    pub stellar_public_key: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Converts a [`User`] database entity to a safe API response.
/// 
/// This implementation automatically excludes sensitive fields like
/// the Stellar secret key when converting to API response format.
/// The roles field is parsed from JSON string to Vec<String> for API consumption.
/// 
/// # Security
/// 
/// The conversion deliberately omits:
/// - `stellar_secret_key` - Sensitive cryptographic material
/// - `password_hash` - Hashed password for security
/// 
/// # Role Conversion
/// 
/// The internal JSON string roles field is converted to a Vec<String>
/// for easier consumption by API clients.
/// 
/// # Usage
/// 
/// ```rust,no_run
/// use nda_backend::models::*;
/// 
/// let user = User {
///     id: "123".to_string(),
///     username: "john_doe".to_string(),
///     stellar_public_key: "GABC...".to_string(),
///     stellar_secret_key: "SABC...".to_string(),
///     roles: r#"["client","partner"]"#.to_string(),
///     created_at: chrono::Utc::now(),
/// };
/// let response: UserResponse = user.into();
/// assert_eq!(response.roles, vec!["client", "partner"]);
/// ```
impl User {
    /// Checks if the user has a specific role.
    /// 
    /// # Parameters
    /// 
    /// * `role` - The role to check for ("client" or "partner")
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the user has the specified role, `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let user = User {
    ///     roles: r#"["client","partner"]"#.to_string(),
    ///     // ... other fields
    /// };
    /// assert!(user.has_role("client"));
    /// assert!(user.has_role("partner"));
    /// ```
    pub fn has_role(&self, role: &str) -> bool {
        if let Ok(roles) = serde_json::from_str::<Vec<String>>(&self.roles) {
            roles.contains(&role.to_string())
        } else {
            // Fallback para compatibilidade com formato antigo
            self.roles == role
        }
    }
    
    /// Checks if the user has the "client" role.
    /// 
    /// Clients can create and manage NDA processes.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the user can act as a client.
    pub fn is_client(&self) -> bool {
        self.has_role("client")
    }
    
    /// Checks if the user has the "partner" role.
    /// 
    /// Partners can access processes that have been shared with them.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the user can act as a partner.
    pub fn is_partner(&self) -> bool {
        self.has_role("partner")
    }

    /// Gets all roles assigned to the user.
    /// 
    /// # Returns
    /// 
    /// Returns a `Vec<String>` containing all roles assigned to the user.
    /// Returns an empty vector if roles cannot be parsed.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let user = User {
    ///     roles: r#"["client","partner"]"#.to_string(),
    ///     // ... other fields
    /// };
    /// let roles = user.get_roles();
    /// assert_eq!(roles, vec!["client", "partner"]);
    /// ```
    pub fn get_roles(&self) -> Vec<String> {
        serde_json::from_str::<Vec<String>>(&self.roles)
            .unwrap_or_else(|_| {
                // Fallback para compatibilidade com formato antigo
                if !self.roles.is_empty() {
                    vec![self.roles.clone()]
                } else {
                    vec![]
                }
            })
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        let roles = user.get_roles();
        UserResponse {
            id: user.id,
            username: user.username,
            name: user.name,
            stellar_public_key: user.stellar_public_key,
            roles,
            created_at: user.created_at,
        }
    }
}

/// Process metadata for API responses (excludes encrypted content).
/// 
/// Safe representation of process data that excludes sensitive information
/// like encrypted content and encryption keys. Used for process listings
/// and metadata operations.
/// 
/// # Fields
/// 
/// * `id` - Process identifier
/// * `title` - Process title/description
/// * `status` - Current process status
/// * `created_at` - Process creation timestamp
/// 
/// # Security Features
/// 
/// - Excludes `encrypted_content` and `encryption_key` for security
/// - Safe for public API responses and process listings
/// - Automatically converted from [`Process`] via `From` trait
#[derive(Debug, Serialize)]
pub struct ProcessResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// Converts a [`Process`] database entity to a safe API response.
/// 
/// This implementation automatically excludes sensitive fields like
/// encrypted content and encryption keys when converting to API response format.
/// 
/// # Security
/// 
/// The conversion deliberately omits:
/// - `encrypted_content` - Sensitive encrypted data
/// - `encryption_key` - Cryptographic key material
/// - `client_id` - Internal database reference
/// 
/// # Usage
/// 
/// ```rust,no_run
/// use nda_backend::models::*;
/// 
/// let process = Process {
///     id: "123".to_string(),
///     client_id: "456".to_string(),
///     title: "Sample Process".to_string(),
///     encrypted_content: "encrypted_data".to_string(),
///     encryption_key: "encryption_key".to_string(),
///     status: "active".to_string(),
///     created_at: chrono::Utc::now(),
/// };
/// let response: ProcessResponse = process.into();
/// ```
impl From<Process> for ProcessResponse {
    fn from(process: Process) -> Self {
        ProcessResponse {
            id: process.id,
            title: process.title,
            description: process.description,
            status: process.status,
            created_at: process.created_at,
        }
    }
}

/// Decrypted process content for authorized access responses.
/// 
/// Contains the decrypted confidential content that is returned when
/// a partner successfully accesses a shared process. This response
/// is only generated after proper authorization verification.
/// 
/// # Fields
/// 
/// * `process_id` - ID of the accessed process
/// * `title` - Process title for reference
/// * `content` - Decrypted confidential content
/// * `accessed_at` - Timestamp when access occurred
/// 
/// # Security Notes
/// 
/// - Content is decrypted only in memory during request processing
/// - Access is logged for audit trails and compliance
/// - Only returned after blockchain-verified sharing authorization
/// - Each access event is recorded in the database
/// 
/// # Usage Context
/// 
/// This model is used exclusively for successful process access operations
/// where the partner has proven authorization to view the content.
#[derive(Debug, Serialize)]
pub struct ProcessAccessResponse {
    pub process_id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub accessed_at: DateTime<Utc>,
}

/// Health check response with server status and timestamp.
/// 
/// Returns basic service health information including the current
/// timestamp in ISO 8601 format for monitoring and diagnostic purposes.
/// 
/// # Fields
/// 
/// * `status` - Service status indicator ("OK" when healthy)
/// * `timestamp` - Current server time in ISO 8601 format
/// 
/// # Usage
/// 
/// This model is used by the `/health` endpoint to provide structured
/// health information that can be consumed by monitoring systems.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
}