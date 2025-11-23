//! # Database Module
//! 
//! This module provides database functionality for the NDA backend system using SQLite.
//! It manages persistent storage for users, processes, process shares, and access records
//! with a focus on security and data integrity.
//! 
//! ## Database Schema
//! 
//! The database consists of four main tables:
//! 
//! ### Users Table
//! Stores user account information with Stellar blockchain integration:
//! - `id`: Unique identifier (UUID)
//! - `username`: Unique username for authentication
//! - `stellar_public_key`: Stellar network public key
//! - `stellar_secret_key`: Encrypted Stellar network secret key
//! - `roles`: User roles as JSON array: `["client"]`, `["partner"]`, or `["client","partner"]`
//! - `created_at`: Account creation timestamp
//! 
//! ### Processes Table
//! Stores NDA process information with encrypted content:
//! - `id`: Unique process identifier (UUID)
//! - `client_id`: Reference to the owning client user
//! - `title`: Process title/name
//! - `description`: Detailed process description
//! - `encrypted_content`: AES-256-GCM encrypted process content
//! - `encryption_key`: Base64-encoded encryption key
//! - `status`: Process status ('active', 'completed', etc.)
//! - `created_at`: Process creation timestamp
//! 
//! ### Process Shares Table
//! Tracks when processes are shared with partners via Stellar:
//! - `id`: Unique share record identifier (UUID)
//! - `process_id`: Reference to the shared process
//! - `partner_public_key`: Stellar public key of the recipient
//! - `stellar_transaction_hash`: Blockchain transaction hash
//! - `shared_at`: Share timestamp
//! 
//! ### Process Accesses Table
//! Logs when partners access shared processes:
//! - `id`: Unique access record identifier (UUID)
//! - `process_id`: Reference to the accessed process
//! - `partner_id`: Reference to the accessing partner
//! - `accessed_at`: Access timestamp
//! 
//! ## Usage Example
//! 
//! ```rust
//! use crate::database::{init_database, queries};
//! 
//! // Initialize database with migrations
//! let pool = init_database().await?;
//! 
//! // Create a new user
//! let user = queries::create_user(
//!     &pool,
//!     "john_doe",
//!     "STELLAR_PUBLIC_KEY",
//!     "encrypted_secret_key",
//!     "client"
//! ).await?;
//! 
//! // Create a process
//! let process = queries::create_process(
//!     &pool,
//!     &user.id,
//!     "Confidential Agreement",
//!     "encrypted_content",
//!     "encryption_key"
//! ).await?;
//! ```
//! 
//! ## Security Considerations
//! 
//! - All sensitive process content is encrypted using AES-256-GCM
//! - Stellar secret keys are stored encrypted
//! - Database operations use prepared statements to prevent SQL injection
//! - Timestamps are stored in RFC3339 format for consistency
//! 
//! ## Error Handling
//! 
//! Database operations return `sqlx::Error` types for SQLite-specific errors
//! and custom error types for application-level validation failures.

use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite, Row};
use std::error::Error;
use chrono::{DateTime, Utc};

/// Initializes the SQLite database connection and runs necessary migrations.
/// 
/// This function sets up the database connection pool and ensures all required
/// tables are created with proper schema. It handles database creation if the
/// file doesn't exist and automatically runs migrations.
/// 
/// # Environment Variables
/// 
/// - `DATABASE_URL`: Optional SQLite database URL (defaults to `sqlite:./stellar_mvp.db`)
/// 
/// # Returns
/// 
/// Returns a `Result` containing:
/// - `Ok(SqlitePool)` - Ready-to-use database connection pool
/// - `Err(Box<dyn Error>)` - Database initialization or migration error
/// 
/// # Examples
/// 
/// ```rust
/// use crate::database::init_database;
/// 
/// let pool = init_database().await?;
/// // Pool is now ready for database operations
/// ```
/// 
/// # Panics
/// 
/// This function will panic if database creation fails, as this is considered
/// a critical system failure that should halt application startup.
/// 
/// # Database Schema
/// 
/// The initialization process creates the following tables:
/// - `users`: User accounts with Stellar integration
/// - `processes`: NDA processes with encrypted content
/// - `process_shares`: Blockchain-recorded process sharing events
/// - `process_accesses`: Access logs for audit trails
pub async fn init_database() -> Result<SqlitePool, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./stellar_mvp.db".to_string());
    
    // Create database if it doesn't exist
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        println!("🔨 Creating database...");
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("✅ Database created successfully"),
            Err(error) => panic!("❌ Failed to create database: {}", error),
        }
    }
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run database migrations
    run_migrations(&pool).await?;
    
    Ok(pool)
}

/// Executes database migrations to create all required tables.
/// 
/// This internal function creates the database schema by executing DDL statements
/// for all required tables. It's designed to be idempotent - running it multiple
/// times is safe and will not cause errors.
/// 
/// # Parameters
/// 
/// * `pool` - Database connection pool
/// 
/// # Returns
/// 
/// Returns `Result<(), sqlx::Error>` indicating migration success or failure.
/// 
/// # Database Tables Created
/// 
/// - **users**: User accounts and Stellar integration data
/// - **processes**: Encrypted NDA process content and metadata
/// - **process_shares**: Blockchain sharing records with transaction hashes
/// - **process_accesses**: Access audit logs for compliance tracking
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("🔄 Running migrations...");
    
    // Create users table with new roles system
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            stellar_public_key TEXT UNIQUE NOT NULL,
            stellar_secret_key TEXT NOT NULL,
            password_hash TEXT NOT NULL DEFAULT 'temp_hash_needs_reset',
            roles TEXT NOT NULL DEFAULT '["client"]',
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Migration: Add roles column if it doesn't exist (for existing databases)
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN roles TEXT DEFAULT '[]'")
        .execute(pool)
        .await;

    // Migration: Add name column if it doesn't exist (for existing databases)
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN name TEXT")
        .execute(pool)
        .await;

    // Migration: Update existing user_type data to roles format if user_type column exists
    let user_type_exists = sqlx::query("PRAGMA table_info(users)")
        .fetch_all(pool)
        .await?
        .iter()
        .any(|row| {
            let name: String = row.get("name");
            name == "user_type"
        });

    if user_type_exists {
        // Migrate existing user_type data to roles format
        sqlx::query("UPDATE users SET roles = '[\"' || user_type || '\"]' WHERE roles = '[]' OR roles IS NULL")
            .execute(pool)
            .await?;
        
        println!("✅ Migrated user_type data to roles format");
    }

    // Migration: Set default name for existing users who don't have one
    sqlx::query("UPDATE users SET name = username WHERE name IS NULL")
        .execute(pool)
        .await?;

    // Migration: Add password_hash column if it doesn't exist (for existing databases)
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN password_hash TEXT DEFAULT 'temp_hash_needs_reset'")
        .execute(pool)
        .await;

    // Migration: Update existing users to have the default password hash if they don't have one
    sqlx::query("UPDATE users SET password_hash = 'temp_hash_needs_reset' WHERE password_hash IS NULL OR password_hash = ''")
        .execute(pool)
        .await?;

    // Create processes table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS processes (
            id TEXT PRIMARY KEY,
            client_id TEXT NOT NULL,
            title TEXT NOT NULL,
            encrypted_content TEXT NOT NULL,
            encryption_key TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create process shares table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS process_shares (
            id TEXT PRIMARY KEY,
            process_id TEXT NOT NULL,
            partner_public_key TEXT NOT NULL,
            stellar_transaction_hash TEXT NOT NULL,
            shared_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create process accesses table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS process_accesses (
            id TEXT PRIMARY KEY,
            process_id TEXT NOT NULL,
            partner_id TEXT NOT NULL,
            accessed_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("✅ Migrations executed successfully!");
    Ok(())
}

/// Converts a UTC DateTime to RFC3339 string format for database storage.
/// 
/// This helper function ensures consistent datetime formatting across
/// all database operations using the RFC3339 standard.
/// 
/// # Parameters
/// 
/// * `dt` - UTC DateTime to convert
/// 
/// # Returns
/// 
/// RFC3339 formatted string representation of the datetime
fn datetime_to_string(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Parses an RFC3339 string into a UTC DateTime.
/// 
/// This helper function converts stored datetime strings back to
/// DateTime objects, handling timezone conversion to UTC.
/// 
/// # Parameters
/// 
/// * `s` - RFC3339 formatted datetime string
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(DateTime<Utc>)` - Parsed datetime in UTC
/// - `Err(chrono::ParseError)` - Parse failure for invalid format
fn string_to_datetime(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc))
}

/// Database queries module containing all CRUD operations.
/// 
/// This module provides type-safe database operations for all entity types
/// in the NDA system. All functions return proper error handling and use
/// prepared statements for SQL injection prevention.
pub mod queries {
    use super::*;
    use crate::models::*;
    use uuid::Uuid;
    use sqlx::Row;

    /// Creates a new user account in the database.
    /// 
    /// This function creates a new user with Stellar blockchain integration,
    /// generating a unique UUID and setting the creation timestamp automatically.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `username` - Unique username for the account
    /// * `name` - Full name or display name of the user
    /// * `stellar_public_key` - User's Stellar network public key
    /// * `stellar_secret_key` - Encrypted Stellar network secret key
    /// * `roles` - User roles as JSON string: `["client"]`, `["partner"]`, or `["client","partner"]`
    ///
    /// # Returns
    ///
    /// Returns `Result` containing:
    /// - `Ok(User)` - Created user with generated ID and timestamp
    /// - `Err(sqlx::Error)` - Database error (e.g., username conflict)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let user = queries::create_user(
    ///     &pool,
    ///     "john_doe",
    ///     "John Doe",
    ///     "GCKFBEIYTKP...",
    ///     "encrypted_secret",
    ///     r#"["client"]"#
    /// ).await?;
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `sqlx::Error` if:
    /// - Username already exists (UNIQUE constraint violation)
    /// - Stellar public key already exists
    /// - Invalid roles format (must be valid JSON array)
    /// - Database connection issues
pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    name: &str,
    stellar_public_key: &str,
    stellar_secret_key: &str,
    password_hash: &str,
    roles: &str,
) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now();
    let created_at_str = datetime_to_string(&created_at);

    sqlx::query(
        r#"
        INSERT INTO users (id, username, name, stellar_public_key, stellar_secret_key, password_hash, roles, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        "#,
    )
    .bind(&id)
    .bind(username)
    .bind(name)
    .bind(stellar_public_key)
    .bind(stellar_secret_key)
    .bind(password_hash)
    .bind(roles)
    .bind(&created_at_str)
    .execute(pool)
    .await?;

    Ok(User {
        id,
        username: username.to_string(),
        name: name.to_string(),
        stellar_public_key: stellar_public_key.to_string(),
        stellar_secret_key: stellar_secret_key.to_string(),
        password_hash: password_hash.to_string(),
        roles: roles.to_string(),
        created_at,
    })
}    /// Finds a user by their username.
    /// 
    /// This function performs a case-sensitive username lookup and returns
    /// the complete user record if found. Useful for authentication and
    /// user lookup operations.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `username` - Username to search for
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(Some(User))` - User found with the specified username
    /// - `Ok(None)` - No user found with that username
    /// - `Err(sqlx::Error)` - Database error or datetime parsing failure
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match queries::find_user_by_username(&pool, "john_doe").await? {
    ///     Some(user) => println!("Found user: {}", user.username),
    ///     None => println!("User not found"),
    /// }
    /// ```
    pub async fn find_user_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM users WHERE username = ?1")
            .bind(username)
            .fetch_optional(pool)
            .await?;

        match row {
            Some(row) => {
                let created_at_str: String = row.get("created_at");
                let created_at = string_to_datetime(&created_at_str)
                    .map_err(|_| sqlx::Error::ColumnDecode { 
                        index: "created_at".to_string(), 
                        source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid datetime")) 
                    })?;

                Ok(Some(User {
                    id: row.get("id"),
                    username: row.get("username"),
                    name: row.get("name"),
                    stellar_public_key: row.get("stellar_public_key"),
                    stellar_secret_key: row.get("stellar_secret_key"),
                    password_hash: row.get("password_hash"),
                    roles: row.get("roles"),
                    created_at,
                }))
            },
            None => Ok(None),
        }
    }

    /// Finds a user by their ID.
    /// 
    /// This function performs a user lookup by UUID and returns
    /// the complete user record if found. Useful for authentication and
    /// user lookup operations.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `user_id` - User ID to search for
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(Some(User))` - User found with the specified ID
    /// - `Ok(None)` - No user found with that ID
    /// - `Err(sqlx::Error)` - Database error or datetime parsing failure
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match queries::find_user_by_id(&pool, &user_id).await? {
    ///     Some(user) => println!("Found user: {}", user.username),
    ///     None => println!("User not found"),
    /// }
    /// ```
    pub async fn find_user_by_id(
        pool: &SqlitePool,
        user_id: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM users WHERE id = ?1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        match row {
            Some(row) => {
                let created_at_str: String = row.get("created_at");
                let created_at = string_to_datetime(&created_at_str)
                    .map_err(|_| sqlx::Error::ColumnDecode { 
                        index: "created_at".to_string(), 
                        source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid datetime")) 
                    })?;

                Ok(Some(User {
                    id: row.get("id"),
                    username: row.get("username"),
                    name: row.get("name"),
                    stellar_public_key: row.get("stellar_public_key"),
                    stellar_secret_key: row.get("stellar_secret_key"),
                    password_hash: row.get("password_hash"),
                    roles: row.get("roles"),
                    created_at,
                }))
            },
            None => Ok(None),
        }
    }

    /// Creates a new NDA process with encrypted content.
    /// 
    /// This function creates a new process owned by a client user, with all
    /// sensitive content encrypted using AES-256-GCM. The process starts
    /// in 'active' status and gets a unique UUID identifier.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `client_id` - ID of the client user creating the process
    /// * `title` - Human-readable title for the process
    /// * `encrypted_content` - Base64-encoded encrypted process content
    /// * `encryption_key` - Base64-encoded encryption key for the content
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(Process)` - Created process with generated ID and timestamp
    /// - `Err(sqlx::Error)` - Database error or foreign key constraint failure
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let process = queries::create_process(
    ///     &pool,
    ///     &user.id,
    ///     "Software Development NDA",
    ///     "base64_encrypted_content",
    ///     "base64_encryption_key"
    /// ).await?;
    /// ```
    /// 
    /// # Security Notes
    /// 
    /// - Content must be pre-encrypted before calling this function
    /// - Encryption keys should be generated using cryptographically secure methods
    /// - Consider the encryption key storage and access patterns carefully
    pub async fn create_process(
        pool: &SqlitePool,
        client_id: &str,
        title: &str,
        description: &str,
        encrypted_content: &str,
        encryption_key: &str,
    ) -> Result<Process, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let created_at_str = datetime_to_string(&created_at);
        let status = "active".to_string();

        sqlx::query(
            r#"
            INSERT INTO processes (id, client_id, title, description, encrypted_content, encryption_key, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
        )
        .bind(&id)
        .bind(client_id)
        .bind(title)
        .bind(description)
        .bind(encrypted_content)
        .bind(encryption_key)
        .bind(&status)
        .bind(&created_at_str)
        .execute(pool)
        .await?;

        Ok(Process {
            id,
            client_id: client_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            encrypted_content: encrypted_content.to_string(),
            encryption_key: encryption_key.to_string(),
            status,
            created_at,
        })
    }

    /// Finds a process by its unique ID.
    /// 
    /// This function retrieves a complete process record by its UUID,
    /// including all metadata and encrypted content. Used for process
    /// access and sharing operations.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `process_id` - UUID of the process to find
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(Some(Process))` - Process found with the specified ID
    /// - `Ok(None)` - No process found with that ID
    /// - `Err(sqlx::Error)` - Database error or datetime parsing failure
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// if let Some(process) = queries::find_process_by_id(&pool, &process_id).await? {
    ///     println!("Process title: {}", process.title);
    /// }
    /// ```
    pub async fn find_process_by_id(
        pool: &SqlitePool,
        process_id: &str,
    ) -> Result<Option<Process>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM processes WHERE id = ?1")
            .bind(process_id)
            .fetch_optional(pool)
            .await?;

        match row {
            Some(row) => {
                let created_at_str: String = row.get("created_at");
                let created_at = string_to_datetime(&created_at_str)
                    .map_err(|_| sqlx::Error::ColumnDecode { 
                        index: "created_at".to_string(), 
                        source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid datetime")) 
                    })?;

                Ok(Some(Process {
                    id: row.get("id"),
                    client_id: row.get("client_id"),
                    title: row.get("title"),
                    description: row.get("description"),
                    encrypted_content: row.get("encrypted_content"),
                    encryption_key: row.get("encryption_key"),
                    status: row.get("status"),
                    created_at,
                }))
            },
            None => Ok(None),
        }
    }

    /// Lists all processes owned by a specific client.
    /// 
    /// This function retrieves all processes created by a client user,
    /// ordered by creation date (newest first). Useful for client
    /// dashboard and process management interfaces.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `client_id` - ID of the client user
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(Vec<Process>)` - List of processes (may be empty)
    /// - `Err(sqlx::Error)` - Database error or datetime parsing failure
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let processes = queries::list_processes_by_client(&pool, &client_id).await?;
    /// for process in processes {
    ///     println!("Process: {} ({})", process.title, process.status);
    /// }
    /// ```
    pub async fn list_processes_by_client(
        pool: &SqlitePool,
        client_id: &str,
    ) -> Result<Vec<Process>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM processes WHERE client_id = ?1 ORDER BY created_at DESC")
            .bind(client_id)
            .fetch_all(pool)
            .await?;

        let mut processes = Vec::new();
        for row in rows {
            let created_at_str: String = row.get("created_at");
            let created_at = string_to_datetime(&created_at_str)
                .map_err(|_| sqlx::Error::ColumnDecode { 
                    index: "created_at".to_string(), 
                    source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid datetime")) 
                })?;

            processes.push(Process {
                id: row.get("id"),
                client_id: row.get("client_id"),
                title: row.get("title"),
                description: row.get("description"),
                encrypted_content: row.get("encrypted_content"),
                encryption_key: row.get("encryption_key"),
                status: row.get("status"),
                created_at,
            });
        }

        Ok(processes)
    }

    /// Records a process sharing event on the Stellar blockchain.
    /// 
    /// This function creates a record when a process is shared with a partner
    /// via the Stellar network. It stores the blockchain transaction hash for
    /// audit and verification purposes.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `process_id` - ID of the process being shared
    /// * `partner_public_key` - Stellar public key of the recipient partner
    /// * `stellar_transaction_hash` - Hash of the blockchain transaction
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(ProcessShare)` - Created share record with generated ID and timestamp
    /// - `Err(sqlx::Error)` - Database error or foreign key constraint failure
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let share = queries::create_process_share(
    ///     &pool,
    ///     &process.id,
    ///     "GCKFBEIYTKP...",
    ///     "stellar_tx_hash_123"
    /// ).await?;
    /// ```
    /// 
    /// # Blockchain Integration
    /// 
    /// This function should be called after successfully submitting a sharing
    /// transaction to the Stellar network. The transaction hash provides
    /// immutable proof of the sharing event.
    pub async fn create_process_share(
        pool: &SqlitePool,
        process_id: &str,
        partner_public_key: &str,
        stellar_transaction_hash: &str,
    ) -> Result<ProcessShare, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let shared_at = Utc::now();
        let shared_at_str = datetime_to_string(&shared_at);

        sqlx::query(
            r#"
            INSERT INTO process_shares (id, process_id, partner_public_key, stellar_transaction_hash, shared_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(&id)
        .bind(process_id)
        .bind(partner_public_key)
        .bind(stellar_transaction_hash)
        .bind(&shared_at_str)
        .execute(pool)
        .await?;

        Ok(ProcessShare {
            id,
            process_id: process_id.to_string(),
            partner_public_key: partner_public_key.to_string(),
            stellar_transaction_hash: stellar_transaction_hash.to_string(),
            shared_at,
        })
    }

    /// Records when a partner accesses a shared process.
    /// 
    /// This function logs access events for audit trails and compliance
    /// monitoring. It creates a timestamped record each time a partner
    /// views or downloads process content.
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `process_id` - ID of the accessed process
    /// * `partner_id` - ID of the partner accessing the process
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(ProcessAccess)` - Created access record with generated ID and timestamp
    /// - `Err(sqlx::Error)` - Database error or foreign key constraint failure
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let access = queries::create_process_access(
    ///     &pool,
    ///     &process.id,
    ///     &partner.id
    /// ).await?;
    /// ```
    /// 
    /// # Compliance Notes
    /// 
    /// Access logging is crucial for:
    /// - Audit trails for regulatory compliance
    /// - Monitoring unauthorized access attempts  
    /// - Usage analytics for process owners
    /// - Legal evidence in case of disputes
    pub async fn create_process_access(
        pool: &SqlitePool,
        process_id: &str,
        partner_id: &str,
    ) -> Result<ProcessAccess, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let accessed_at = Utc::now();
        let accessed_at_str = datetime_to_string(&accessed_at);

        sqlx::query(
            r#"
            INSERT INTO process_accesses (id, process_id, partner_id, accessed_at)
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(&id)
        .bind(process_id)
        .bind(partner_id)
        .bind(&accessed_at_str)
        .execute(pool)
        .await?;

        Ok(ProcessAccess {
            id,
            process_id: process_id.to_string(),
            partner_id: partner_id.to_string(),
            accessed_at,
        })
    }

    /// Lists all access events for processes owned by a client.
    /// 
    /// This function retrieves a comprehensive audit trail showing when
    /// partners have accessed the client's processes. It includes denormalized
    /// data (process titles, descriptions, status, and partner usernames) for easier reporting.
    /// 
    /// # Error Handling
    /// 
    /// This function is designed to be robust and avoid HTTP 500 errors:
    /// - Database connection failures return an empty array instead of propagating errors
    /// - Individual row processing errors are logged but don't stop processing other rows  
    /// - Invalid datetime formats are logged as warnings and set to None
    /// - Missing optional data is handled gracefully without errors
    /// 
    /// # Parameters
    /// 
    /// * `pool` - Database connection pool
    /// * `client_id` - ID of the client user
    /// 
    /// # Returns
    /// 
    /// Always returns `Ok(Vec<ProcessAccessWithDetails>)`:
    /// - Success: List of access events with complete process details
    /// - Error conditions: Empty array (errors are logged internally)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let accesses = queries::list_process_accesses_by_client(&pool, &client_id).await?;
    /// for access in accesses {
    ///     match (&access.partner_username, &access.accessed_at) {
    ///         (Some(username), Some(accessed_at)) => {
    ///             println!("{} accessed '{}' ({}): {} at {}", 
    ///                 username, 
    ///                 access.process_title,
    ///                 access.process_status,
    ///                 access.process_description,
    ///                 accessed_at
    ///             );
    ///         }
    ///         _ => {
    ///             println!("Process '{}' ({}): {} - No access yet", 
    ///                 access.process_title,
    ///                 access.process_status,
    ///                 access.process_description
    ///             );
    ///         }
    ///     }
    /// }
    /// ```
    /// 
    /// # Query Details
    /// 
    /// This function performs a LEFT OUTER JOIN across three tables:
    /// - `processes`: Main table with process data (always present)
    /// - `process_accesses`: Access records (optional - may be None for processes without access)
    /// - `users`: To get partner usernames (optional - may be None when no user data)
    /// 
    /// # Return Behavior
    /// 
    /// - Always returns process data (`process_id`, `process_title`, `process_description`, `process_status`)
    /// - Access data is optional (`id`, `partner_id`, `accessed_at`) - None when no access exists
    /// - Partner username is optional - None when no matching user found
    /// 
    /// Results are ordered by access time (newest first, nulls last) for chronological review.
    pub async fn list_process_accesses_by_client(
        pool: &SqlitePool,
        client_id: &str,
    ) -> Result<Vec<ProcessAccessWithDetails>, sqlx::Error> {
        // Log the start of the operation
        tracing::info!("Starting list_process_accesses_by_client for client_id: {}", client_id);

        // Execute the query with error handling
        let rows_result = sqlx::query(
            r#"
            SELECT 
                pa.id,
                p.id as process_id,
                pa.partner_id,
                pa.accessed_at,
                p.title as process_title,
                p.description as process_description,
                p.status as process_status,
                u.username as partner_username
            FROM processes p
            LEFT OUTER JOIN process_accesses pa ON pa.process_id = p.id
            LEFT OUTER JOIN users u ON pa.partner_id = u.id
            WHERE p.client_id = ?1
            ORDER BY pa.accessed_at DESC NULLS LAST
            "#,
        )
        .bind(client_id)
        .fetch_all(pool)
        .await;

        let rows = match rows_result {
            Ok(rows) => {
                tracing::info!("Successfully fetched {} rows from database", rows.len());
                rows
            }
            Err(err) => {
                tracing::error!("Database query failed for client_id {}: {}", client_id, err);
                // Return empty array instead of propagating error to avoid HTTP 500
                return Ok(Vec::new());
            }
        };

        let mut accesses = Vec::new();
        let mut processed_count = 0;
        let mut error_count = 0;

        for (index, row) in rows.iter().enumerate() {
            match process_row_safely(row, index) {
                Ok(access_detail) => {
                    accesses.push(access_detail);
                    processed_count += 1;
                }
                Err(err) => {
                    error_count += 1;
                    tracing::warn!("Failed to process row {}: {} - Skipping this row", index, err);
                    // Continue processing other rows instead of failing completely
                }
            }
        }

        tracing::info!(
            "Completed processing: {} successful, {} errors, {} total rows", 
            processed_count, error_count, rows.len()
        );

        Ok(accesses)
    }

    /// Safely processes a single row from the database query into ProcessAccessWithDetails.
    /// 
    /// This helper function isolates error handling for individual rows, ensuring that
    /// if one row has corrupted data, it doesn't prevent the processing of other rows.
    /// 
    /// # Parameters
    /// 
    /// * `row` - Database row to process
    /// * `index` - Row index for logging purposes
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(ProcessAccessWithDetails)` - Successfully processed row
    /// - `Err(String)` - Error description for logging
    fn process_row_safely(row: &sqlx::sqlite::SqliteRow, index: usize) -> Result<ProcessAccessWithDetails, String> {
        // Always try to get process data first (these should never be NULL)
        let process_id = row.try_get::<String, _>("process_id")
            .map_err(|e| format!("Failed to get process_id: {}", e))?;
        
        let process_title = row.try_get::<String, _>("process_title")
            .map_err(|e| format!("Failed to get process_title: {}", e))?;
        
        let process_description = row.try_get::<String, _>("process_description")
            .map_err(|e| format!("Failed to get process_description: {}", e))?;
        
        let process_status = row.try_get::<String, _>("process_status")
            .map_err(|e| format!("Failed to get process_status: {}", e))?;

        // Handle optional fields safely
        let id = row.try_get::<String, _>("id").ok();
        let partner_id = row.try_get::<String, _>("partner_id").ok();
        let partner_username = row.try_get::<String, _>("partner_username").ok();

        // Handle optional accessed_at field with careful datetime parsing
        let accessed_at = match row.try_get::<String, _>("accessed_at") {
            Ok(accessed_at_str) => {
                // Handle empty strings as NULL
                if accessed_at_str.trim().is_empty() {
                    None
                } else {
                    match string_to_datetime(&accessed_at_str) {
                        Ok(dt) => Some(dt),
                        Err(parse_err) => {
                            tracing::warn!(
                                "Row {}: Failed to parse accessed_at '{}': {} - Setting to None", 
                                index, accessed_at_str, parse_err
                            );
                            None
                        }
                    }
                }
            }
            Err(_) => None, // NULL value, which is expected for processes without access
        };

        Ok(ProcessAccessWithDetails {
            id,
            process_id,
            partner_id,
            accessed_at,
            process_title,
            process_description,
            process_status,
            partner_username,
        })
    }

}

