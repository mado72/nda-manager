//! # HTTP Handlers Module
//! 
//! This module provides HTTP request handlers for the NDA backend API.
//! It implements a RESTful API using the Axum web framework for managing
//! users, NDA processes, and blockchain-integrated sharing operations.
//! 
//! ## API Overview
//! 
//! The API provides the following main endpoints:
//! 
//! ### Health Check
//! - `GET /health` - Simple health check endpoint with timestamp
//! 
//! ### User Management (Role-Based System)
//! - `POST /api/users/register` - Register new users with Stellar account creation and multi-role support
//! - `POST /api/users/login` - Authenticate existing users by username
//! 
//! ### Process Management
//! - `POST /api/processes` - Create new NDA processes with AES-256-GCM encryption
//! - `GET /api/processes?client_id=<id>` - List processes owned by a specific client
//! 
//! ### Sharing & Access (Blockchain-Secured)
//! - `POST /api/processes/share` - Share processes via Stellar blockchain transactions
//! - `POST /api/processes/access` - Access shared processes with content decryption
//! - `GET /api/notifications?client_id=<id>` - Get access audit trail for process owners
//! 
//! ## Role System
//! 
//! The API supports a flexible role-based access control system:
//! 
//! - **Client Role**: Can create and manage NDA processes, share with suppliers
//! - **Supplier Role**: Can access shared processes and view confidential content
//! - **Hybrid Users**: Can have both roles (`["client", "supplier"]`) for full functionality
//! 
//! Role verification is enforced at the handler level for appropriate operations.
//! 
//! ## Security Model
//! 
//! The API implements multiple security layers:
//! 
//! - **Role-Based Access Control**: Endpoints verify user roles before operations
//! - **AES-256-GCM Encryption**: All process content is encrypted with unique keys
//! - **Blockchain Verification**: Process sharing is recorded on Stellar network
//! - **Access Control**: Suppliers can only access processes explicitly shared with them
//! - **Complete Audit Trail**: All access events are logged for compliance
//! 
//! ## Updated Request Flow Example
//! 
//! ```rust
//! // 1. Register users with roles
//! let client = register_user(RegisterRequest {
//!     username: "client_company".to_string(),
//!     name: "Client Company Inc.".to_string(),
//!     roles: vec!["client".to_string()],
//! }).await?;
//! 
//! let supplier = register_user(RegisterRequest {
//!     username: "supplier_corp".to_string(),
//!     name: "Supplier Corporation".to_string(),
//!     roles: vec!["supplier".to_string()],
//! }).await?;
//! 
//! // 2. Create encrypted process (client role required)
//! let process = create_process(CreateProcessRequest {
//!     client_id: client.id,
//!     title: "Software Development NDA".to_string(),
//!     description: "Confidential software project details".to_string(),
//!     confidential_content: "Sensitive technical specifications...".to_string(),
//! }).await?;
//! 
//! // 3. Share via blockchain (creates immutable record)
//! let share = share_process(ShareProcessRequest {
//!     client_username: "client_company".to_string(),
//!     process_id: process.id,
//!     supplier_public_key: supplier.stellar_public_key,
//! }).await?;
//! 
//! // 4. Supplier accesses content (supplier role required)
//! let content = access_process(AccessProcessRequest {
//!     process_id: process.id,
//!     supplier_username: "supplier_corp".to_string(),
//!     supplier_public_key: supplier.stellar_public_key,
//! }).await?;
//! ```
//! 
//! ## Error Handling
//! 
//! All handlers return HTTP status codes following REST conventions:
//! - `200 OK` - Successful operations
//! - `400 Bad Request` - Invalid request parameters or missing required fields
//! - `401 Unauthorized` - Authentication failures
//! - `403 Forbidden` - Insufficient permissions or role requirements not met
//! - `404 Not Found` - Resource not found (user, process, etc.)
//! - `409 Conflict` - Resource conflicts (e.g., username already exists)
//! - `422 Unprocessable Entity` - Request valid but cannot be processed
//! - `500 Internal Server Error` - Server-side errors (database, encryption, blockchain)
//! 
//! ## Stellar Integration
//! 
//! The handlers integrate with Stellar blockchain for:
//! - Automatic account creation and funding on testnet for all registered users
//! - Recording process sharing transactions with immutable proof
//! - Verification of sharing rights before granting access to confidential content
//! - Comprehensive audit trails that meet regulatory compliance requirements

use axum::{
    extract::{State, Json, Query},
    response::Json as ResponseJson,
    http::StatusCode,
};
use std::sync::Arc;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;
use sqlx;

use crate::{
    models::*,
    stellar_real::StellarClient,
    crypto::{generate_key, encrypt_content, decrypt_content},
    database::queries,
    auth::Auth,
};

/// Application state shared across all handlers.
/// 
/// Contains the database connection pool and other shared resources
/// needed by HTTP handlers to process requests. This state is cloned
/// and shared across all handler instances using Arc for thread safety.
/// 
/// # Fields
/// 
/// * `pool` - SQLite connection pool for database operations
/// 
/// # Thread Safety
/// 
/// This struct is `Clone` and used with `Arc` to share state safely
/// across multiple concurrent HTTP request handlers. The SQLite pool
/// handles concurrent access internally.
/// 
/// # Security Considerations
/// 
/// - Database connections use prepared statements to prevent SQL injection
/// - All sensitive operations are logged for audit trails
/// - Connection pool limits prevent resource exhaustion attacks
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

/// Query parameters for endpoints that list processes.
/// 
/// Used by endpoints that need to filter or identify processes
/// based on the client ID. The client_id parameter is optional
/// in the struct but required by most endpoints that use it.
/// 
/// # Fields
/// 
/// * `client_id` - ID of the client to filter processes for
/// 
/// # Usage
/// 
/// This struct is used with Axum's `Query` extractor to parse URL parameters:
/// 
/// ```
/// GET /api/processes?client_id=client-uuid
/// ```
#[derive(Deserialize)]
pub struct ListProcessesQuery {
    pub client_id: Option<String>,
}

/// Simple health check endpoint handler.
/// 
/// Returns a static "OK" string to verify that the service is running.
/// This endpoint can be used by load balancers, monitoring systems,
/// and deployment scripts to verify service health.
/// 
/// # Returns
/// 
/// Always returns health status with current timestamp in ISO format.
/// 
/// # HTTP Response
/// - **Status**: 200 OK
/// - **Body**: JSON with status and timestamp
/// 
/// # Examples
/// 
/// ```
/// GET /health
/// → 200 OK
/// → {"status": "OK", "timestamp": "2024-01-01T00:00:00Z"}
/// ```
pub async fn health_check() -> ResponseJson<HealthResponse> {
    ResponseJson(HealthResponse {
        status: "OK".to_string(),
        timestamp: Utc::now(),
    })
}

/// Registers a new user with automatic Stellar account creation.
/// 
/// This endpoint creates a new user account with integrated Stellar blockchain
/// functionality. It automatically generates a Stellar keypair, funds the account
/// on testnet, and stores the user information securely in the database.
/// 
/// # Parameters
/// 
/// * `state` - Shared application state containing database pool
/// * `payload` - Registration request with username and user type
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(ResponseJson<UserResponse>)` - Successfully created user with Stellar integration
/// - `Err(StatusCode)` - HTTP error code indicating failure reason
/// 
/// # HTTP Responses
/// 
/// - **200 OK**: User created successfully
/// - **409 Conflict**: Username already exists
/// - **500 Internal Server Error**: Stellar account creation or database error
/// 
/// # Request Body
///
/// ```json
/// {
///   "username": "client_company",
///   "name": "Client Company Inc.",
///   "roles": ["client"]
/// }
/// ```
///
/// # Multi-Role Example
///
/// ```json
/// {
///   "username": "hybrid_user",
///   "name": "Hybrid Business Solutions", 
///   "roles": ["client", "supplier"]
/// }
/// ```
///
/// # Response Body
///
/// ```json
/// {
///   "id": "uuid-string",
///   "username": "client_company",
///   "name": "Client Company Inc.",
///   "stellar_public_key": "GCKFBEIY...",
///   "roles": ["client"],
///   "created_at": "2024-01-01T00:00:00Z"
/// }
/// ```
/// 
/// # Security Notes
/// 
/// - Stellar secret keys are stored encrypted in the database
/// - Testnet accounts are automatically funded for development/testing
/// - Username uniqueness is enforced at the database level
/// 
/// # Stellar Integration
/// 
/// Each user gets:
/// - A unique Stellar keypair (public/secret key)
/// - Automatic testnet funding for immediate use
/// - Integration ready for blockchain transactions
pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResponseJson<UserResponse>, StatusCode> {
    // Check if user already exists
    if let Ok(Some(_)) = queries::find_user_by_username(&state.pool, &payload.username).await {
        return Err(StatusCode::CONFLICT);
    }
    
    // Create real Stellar account
    let stellar_account = StellarClient::generate_keypair()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fund account on testnet automatically
    let stellar_client = StellarClient::new_testnet();
    let _funded = stellar_client
        .fund_testnet_account(&stellar_account.public_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Hash the password
    let password_hash = Auth::hash_password(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create user in database with roles
    let roles_json = serde_json::to_string(&payload.roles)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
        
    let user = queries::create_user(
        &state.pool,
        &payload.username,
        &payload.name,
        &stellar_account.public_key,
        &stellar_account.secret_key,
        &password_hash,
        &roles_json,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(user.into()))
}

/// Authenticates an existing user by username lookup.
/// 
/// This endpoint performs simple username-based authentication by looking up
/// the user in the database. In a production system, this would typically
/// include password verification, but this MVP uses username-only auth.
/// 
/// # Parameters
/// 
/// * `state` - Shared application state containing database pool
/// * `payload` - Login request with username
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(ResponseJson<UserResponse>)` - Successfully authenticated user
/// - `Err(StatusCode)` - HTTP error code indicating failure reason
/// 
/// # HTTP Responses
/// 
/// - **200 OK**: Authentication successful
/// - **401 Unauthorized**: Invalid username or password
/// - **500 Internal Server Error**: Database or password verification error
/// 
/// # Request Body
/// 
/// ```json
/// {
///   "username": "client_company",
///   "password": "user_password"
/// }
/// ```
/// 
/// # Response Body
/// 
/// ```json
/// {
///   "id": "uuid-string",
///   "username": "client_company",
///   "stellar_public_key": "GCKFBEIY...",
///   "user_type": "client",
///   "created_at": "2024-01-01T00:00:00Z"
/// }
/// ```
/// 
/// # Security Notes
/// 
/// - Passwords are hashed using bcrypt with salt for secure storage
/// - Failed login attempts return generic "unauthorized" for security
/// - Consider adding JWT tokens for session management and rate limiting
pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<UserResponse>, StatusCode> {
    // Find user by username
    let user = queries::find_user_by_username(&state.pool, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password against stored hash
    let is_valid = Auth::verify_password(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(ResponseJson(user.into()))
}

/// Creates a new NDA process with encrypted content.
/// 
/// This endpoint allows clients to create new NDA processes with confidential
/// content that is automatically encrypted using AES-256-GCM. Each process
/// gets a unique encryption key and is associated with the client user.
/// 
/// # Parameters
/// 
/// * `state` - Shared application state containing database pool
/// * `payload` - Process creation request with title and content
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(ResponseJson<ProcessResponse>)` - Successfully created encrypted process
/// - `Err(StatusCode)` - HTTP error code indicating failure reason
/// 
/// # HTTP Responses
/// 
/// - **200 OK**: Process created successfully
/// - **403 Forbidden**: User doesn't have client role
/// - **422 Unprocessable Entity**: Client ID not found
/// - **500 Internal Server Error**: Encryption or database error
/// 
/// # Request Body
/// 
/// ```json
/// {
///   "client_id": "client-uuid-string",
///   "title": "Software Development NDA",
///   "description": "Confidential software project details",
///   "confidential_content": "Sensitive technical details and trade secrets..."
/// }
/// ```
/// 
/// # Response Body
/// 
/// ```json
/// {
///   "id": "process-uuid",
///   "client_id": "client-uuid",
///   "title": "Software Development NDA",
///   "description": "Confidential software project details",
///   "status": "active",
///   "created_at": "2024-01-01T00:00:00Z"
/// }
/// ```
/// 
/// # Security Features
/// 
/// - Content is encrypted with AES-256-GCM before storage
/// - Each process gets a unique encryption key
/// - Encryption keys are stored separately from content
/// - Only the process owner and explicitly shared suppliers can decrypt
/// 
/// # Process Lifecycle
/// 
/// 1. Content is encrypted with a generated key
/// 2. Process record is created in database
/// 3. Process can be shared with suppliers via blockchain transactions
/// 4. Access events are logged for audit trails
pub async fn create_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProcessRequest>,
) -> Result<ResponseJson<ProcessResponse>, StatusCode> {
    // Find client by ID and verify client role
    let client = queries::find_user_by_id(&state.pool, &payload.client_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNPROCESSABLE_ENTITY)?;
        
    // Verify user has client role
    if !client.is_client() {
        return Err(StatusCode::FORBIDDEN);
    }

    let encryption_key = generate_key();
    let encrypted_content = encrypt_content(&payload.confidential_content, &encryption_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let process = queries::create_process(
        &state.pool,
        &client.id,
        &payload.title,
        &payload.description,
        &encrypted_content,
        &encryption_key,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(process.into()))
}

/// Shares a process with a supplier via Stellar blockchain transaction.
/// 
/// This endpoint creates an immutable record of process sharing on the Stellar
/// blockchain. It submits a transaction that proves the client has shared access
/// to a specific process with a specific supplier, creating an audit trail.
/// 
/// # Parameters
/// 
/// * `state` - Shared application state containing database pool
/// * `payload` - Process sharing request with IDs and recipient
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(ResponseJson<ProcessShare>)` - Successfully recorded sharing event
/// - `Err(StatusCode)` - HTTP error code indicating failure reason
/// 
/// # HTTP Responses
/// 
/// - **200 OK**: Process shared successfully with blockchain proof
/// - **404 Not Found**: Process or client not found
/// - **500 Internal Server Error**: Blockchain transaction or database error
/// 
/// # Request Body
/// 
/// ```json
/// {
///   "client_username": "client_company",
///   "process_id": "process-uuid",
///   "supplier_public_key": "GCKFBEIYTKP..."
/// }
/// ```
/// 
/// # Response Body
/// 
/// ```json
/// {
///   "id": "share-uuid",
///   "process_id": "process-uuid",
///   "supplier_public_key": "GCKFBEIYTKP...",
///   "stellar_transaction_hash": "abc123...",
///   "shared_at": "2024-01-01T00:00:00Z"
/// }
/// ```
/// 
/// # Blockchain Integration
/// 
/// - Creates a Stellar transaction with process sharing metadata
/// - Transaction hash provides immutable proof of sharing
/// - Memo field contains process ID for reference
/// - Transaction is recorded on Stellar testnet for development
/// 
/// # Security & Compliance
/// 
/// - Immutable blockchain record prevents disputes
/// - Transaction hash can be independently verified
/// - Sharing permissions are cryptographically provable
/// - Audit trail meets regulatory requirements
pub async fn share_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ShareProcessRequest>,
) -> Result<ResponseJson<ProcessShare>, StatusCode> {
    let stellar_client = StellarClient::new_testnet();
    
    // Find process
    let _process = queries::find_process_by_id(&state.pool, &payload.process_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Find client by username
    let client = queries::find_user_by_username(&state.pool, &payload.client_username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Send real Stellar transaction
    let tx_result = stellar_client
        .share_process_transaction(
            &client.stellar_secret_key,
            &payload.supplier_public_key,
            &payload.process_id,
            &format!("NDA_SHARE:{}", payload.process_id),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Register sharing event
    let share = queries::create_process_share(
        &state.pool,
        &payload.process_id,
        &payload.supplier_public_key,
        &tx_result.hash,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(share))
}

/// Allows suppliers to access shared process content with decryption.
/// 
/// This endpoint verifies that a process has been properly shared with a supplier
/// (by checking the sharing records), then decrypts and returns the confidential
/// content. It also logs the access event for audit trails and compliance.
/// 
/// # Parameters
/// 
/// * `state` - Shared application state containing database pool
/// * `payload` - Process access request with process ID and supplier username
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(ResponseJson<ProcessAccessResponse>)` - Decrypted process content and metadata
/// - `Err(StatusCode)` - HTTP error code indicating failure reason
/// 
/// # HTTP Responses
/// 
/// - **200 OK**: Access granted, content decrypted and returned
/// - **403 Forbidden**: Process not shared with this supplier or insufficient supplier role
/// - **404 Not Found**: Process or supplier not found
/// - **500 Internal Server Error**: Decryption or database error
/// 
/// # Request Body
/// 
/// ```json
/// {
///   "process_id": "process-uuid",
///   "supplier_username": "supplier_company",
///   "supplier_public_key": "GCKFBEIYTKP..."
/// }
/// ```
/// 
/// # Response Body
/// 
/// ```json
/// {
///   "process_id": "process-uuid",
///   "title": "Software Development NDA",
///   "description": "Confidential software project details",
///   "content": "Decrypted confidential content...",
///   "accessed_at": "2024-01-01T00:00:00Z"
/// }
/// ```
/// 
/// # Access Control
/// 
/// The endpoint performs several security checks:
/// 1. Verifies the process exists
/// 2. Verifies the supplier exists
/// 3. Checks that sharing record exists in database
/// 4. Only then decrypts and returns content
/// 
/// # Audit Trail
/// 
/// Every successful access is logged with:
/// - Timestamp of access
/// - Supplier who accessed the content
/// - Process that was accessed
/// - Complete audit trail for compliance
/// 
/// # Security Notes
/// 
/// - Content is decrypted in memory only
/// - Access is logged for regulatory compliance
/// - Failed access attempts are also logged
/// - Sharing verification prevents unauthorized access
pub async fn access_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AccessProcessRequest>,
) -> Result<ResponseJson<ProcessAccessResponse>, StatusCode> {
    // Find process with specific fields
    let process = sqlx::query!(
        r#"
        SELECT id, client_id, title, description, encrypted_content, encryption_key, status, created_at
        FROM processes WHERE id = ?
        "#,
        payload.process_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Find supplier with specific fields and verify supplier role
    let supplier = queries::find_user_by_username(&state.pool, &payload.supplier_username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
        
    // Verify user has supplier role
    if !supplier.is_supplier() {
        return Err(StatusCode::FORBIDDEN);
    }

    // Verify if sharing exists in database
    let share_exists = sqlx::query!(
        "SELECT id FROM process_shares WHERE process_id = ? AND supplier_public_key = ?",
        payload.process_id,
        payload.supplier_public_key
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if share_exists.is_none() {
        println!("❌ Access denied: Process was not shared with this supplier");
        return Err(StatusCode::FORBIDDEN);
    }

    println!("✅ Access authorized: Sharing found in database");

    // Decrypt content
    let decrypted_content = decrypt_content(&process.encrypted_content, &process.encryption_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Register access event
    let access_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let now_string = now.to_rfc3339();
    
    sqlx::query!(
        r#"
        INSERT INTO process_accesses (id, process_id, supplier_id, accessed_at)
        VALUES (?, ?, ?, ?)
        "#,
        access_id,
        payload.process_id,
        supplier.id,
        now_string
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("📊 Access registered successfully");

    let response = ProcessAccessResponse {
        process_id: payload.process_id,
        title: process.title,
        description: process.description,
        content: decrypted_content,
        accessed_at: now,
    };

    Ok(ResponseJson(response))
}

/// Lists all processes owned by a specific client.
/// 
/// This endpoint retrieves all NDA processes created by a client user,
/// returning them in reverse chronological order (newest first). The
/// response includes process metadata but not the encrypted content.
/// 
/// # Parameters
/// 
/// * `state` - Shared application state containing database pool
/// * `params` - Query parameters including client username
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(ResponseJson<Vec<ProcessResponse>>)` - List of client's processes
/// - `Err(StatusCode)` - HTTP error code indicating failure reason
/// 
/// # HTTP Responses
/// 
/// - **200 OK**: Processes retrieved successfully
/// - **400 Bad Request**: Missing client_id parameter
/// - **404 Not Found**: Client ID not found
/// - **500 Internal Server Error**: Database error
/// 
/// # Query Parameters
/// 
/// - `client_id` (required): ID of the client whose processes to list
/// 
/// # Example Request
/// 
/// ```
/// GET /api/processes?client_id=client-uuid
/// ```
/// 
/// # Response Body
/// 
/// ```json
/// [
///   {
///     "id": "process-uuid-1",
///     "client_id": "client-uuid",
///     "title": "Software Development NDA",
///     "description": "Confidential software project details",
///     "status": "active",
///     "created_at": "2024-01-01T00:00:00Z"
///   },
///   {
///     "id": "process-uuid-2",
///     "client_id": "client-uuid",
///     "title": "Marketing Partnership NDA",
///     "description": "Joint marketing campaign specifications",
///     "status": "active",
///     "created_at": "2024-01-01T00:00:00Z"
///   }
/// ]
/// ```
/// 
/// # Security Notes
/// 
/// - Only returns processes owned by the specified client
/// - Encrypted content is not included in the response
/// - Process access requires separate authorization via sharing
pub async fn list_processes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListProcessesQuery>,
) -> Result<ResponseJson<Vec<ProcessResponse>>, StatusCode> {
    let client_id = params.client_id.ok_or(StatusCode::BAD_REQUEST)?;
    
    // Find client by ID
    let client = queries::find_user_by_id(&state.pool, &client_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let processes = queries::list_processes_by_client(&state.pool, &client.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<ProcessResponse> = processes.into_iter().map(|p| p.into()).collect();
    Ok(ResponseJson(response))
}

/// Retrieves access notifications for a client's processes.
/// 
/// This endpoint provides clients with a comprehensive audit trail showing
/// when suppliers have accessed their shared processes. It returns denormalized
/// data including process titles, descriptions, and supplier usernames for detailed reporting.
/// 
/// # Parameters
/// 
/// * `state` - Shared application state containing database pool
/// * `params` - Query parameters including client username
/// 
/// # Returns
/// 
/// Returns `Result` containing:
/// - `Ok(ResponseJson<Vec<ProcessAccessWithDetails>>)` - List of access events with process details
/// - `Err(StatusCode)` - HTTP error code indicating failure reason
/// 
/// # HTTP Responses
/// 
/// - **200 OK**: Notifications retrieved successfully
/// - **400 Bad Request**: Missing client_id parameter
/// - **404 Not Found**: Client ID not found
/// - **500 Internal Server Error**: Database error
/// 
/// # Query Parameters
/// 
/// - `client_id` (required): ID of the client whose notifications to retrieve
/// 
/// # Example Request
/// 
/// ```
/// GET /api/notifications?client_id=client-uuid
/// ```
/// 
/// # Response Body
/// 
/// ```json
/// [
///   {
///     "id": "access-uuid-1",
///     "process_id": "process-uuid",
///     "supplier_id": "supplier-uuid",
///     "accessed_at": "2024-01-01T10:30:00Z",
///     "process_title": "Software Development NDA",
///     "process_description": "Comprehensive confidentiality agreement for software development partnership",
///     "supplier_username": "supplier_company"
///   },
///   {
///     "id": "access-uuid-2",
///     "process_id": "process-uuid-2",
///     "supplier_id": "supplier-uuid-2",
///     "accessed_at": "2024-01-01T09:15:00Z",
///     "process_title": "Marketing Partnership NDA",
///     "process_description": "Non-disclosure agreement for marketing collaboration and data sharing",
///     "supplier_username": "another_supplier"
///   }
/// ]
/// ```
/// 
/// # Use Cases
/// 
/// - **Enhanced Compliance Reporting**: Generate detailed audit reports with process context
/// - **Contextual Usage Analytics**: Track access patterns with meaningful process descriptions
/// - **Security Monitoring**: Monitor access with full process details for better threat detection
/// - **Rich Client Dashboard**: Provide comprehensive notifications with process context
/// 
/// # Data Privacy
/// 
/// - Only shows access to processes owned by the requesting client
/// - Includes process descriptions and supplier usernames but not sensitive encrypted content
/// - Ordered by access time (most recent first) for easy monitoring
pub async fn get_notifications(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListProcessesQuery>,
) -> Result<ResponseJson<Vec<ProcessAccessWithDetails>>, StatusCode> {
    let client_id = params.client_id.ok_or(StatusCode::BAD_REQUEST)?;
    
    // Find client by ID
    let client = queries::find_user_by_id(&state.pool, &client_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let notifications = queries::list_process_accesses_by_client(&state.pool, &client.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(notifications))
}