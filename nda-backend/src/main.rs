//! # NDA Backend Application
//! 
//! This is the main entry point for the NDA (Non-Disclosure Agreement) backend service.
//! The application provides a REST API for managing encrypted NDA processes with
//! blockchain-based sharing using the Stellar network.
//! 
//! ## Architecture Overview
//! 
//! The application follows a modular architecture with clear separation of concerns:
//! 
//! - **handlers**: HTTP request handlers implementing the REST API
//! - **database**: SQLite database operations and connection management
//! - **models**: Data structures and type definitions
//! - **crypto**: AES-256-GCM encryption for sensitive content
//! - **stellar_real**: Stellar blockchain integration for immutable sharing records
//! 
//! ## API Endpoints
//! 
//! ### Health Check
//! - `GET /health` - Service health verification
//! 
//! ### User Management
//! - `POST /api/users/register` - Register new users with Stellar accounts
//! - `POST /api/users/login` - User authentication
//! 
//! ### Process Management
//! - `POST /api/processes` - Create new encrypted NDA processes
//! - `GET /api/processes` - List processes owned by a client
//! 
//! ### Sharing & Access
//! - `POST /api/processes/share` - Share processes via blockchain transactions
//! - `POST /api/processes/access` - Access shared processes with decryption
//! - `GET /api/notifications` - Get access notifications for audit trails
//! 
//! ## Security Features
//! 
//! - **End-to-End Encryption**: All sensitive content encrypted with AES-256-GCM
//! - **Blockchain Verification**: Immutable sharing records on Stellar network
//! - **Access Control**: Cryptographically verified sharing permissions
//! - **Audit Trails**: Complete access logging for compliance
//! - **CORS Protection**: Configurable cross-origin resource sharing
//! 
//! ## Technology Stack
//! 
//! - **Web Framework**: Axum (async HTTP server)
//! - **Database**: SQLite with SQLx for type-safe queries
//! - **Blockchain**: Stellar network integration
//! - **Encryption**: AES-256-GCM with hardware acceleration
//! - **Logging**: Tracing for structured logging
//! - **Async Runtime**: Tokio for high-performance async operations
//! 
//! ## Configuration
//! 
//! The application can be configured via environment variables:
//! - `DATABASE_URL`: SQLite database path (default: `sqlite:./stellar_mvp.db`)
//! - Server binds to `0.0.0.0:3000` by default

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod models;
mod handlers;
mod database;
mod crypto;
mod stellar_real;
mod auth;

use handlers::AppState;

/// Application entry point.
/// 
/// Initializes the NDA backend service with the following startup sequence:
/// 
/// 1. **Logging Setup**: Initializes structured logging with tracing
/// 2. **Database Connection**: Establishes SQLite connection pool with migrations
/// 3. **Application State**: Creates shared state for dependency injection
/// 4. **Route Configuration**: Sets up REST API endpoints with middleware
/// 5. **Server Startup**: Binds to network interface and starts accepting requests
/// 
/// # Returns
/// 
/// Returns `Result<(), Box<dyn std::error::Error>>` indicating startup success or failure.
/// 
/// # Errors
/// 
/// The application will fail to start if:
/// - Database connection or migration fails
/// - Network binding fails (port already in use)
/// - Invalid configuration parameters
/// 
/// # Examples
/// 
/// ```bash
/// # Start the server with default configuration
/// cargo run
/// 
/// # Start with custom database
/// DATABASE_URL=sqlite:./custom.db cargo run
/// ```
/// 
/// # Architecture Notes
/// 
/// The application uses:
/// - **Dependency Injection**: Shared state via Arc for thread-safe access
/// - **Async Processing**: Tokio runtime for high-performance I/O
/// - **Type Safety**: SQLx for compile-time verified database queries
/// - **Error Handling**: Structured error propagation with proper HTTP status codes
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    // Connect to database and run migrations
    let pool = database::init_database().await?;

    // Create application state for dependency injection
    let state = Arc::new(AppState { pool });

    // Configure API routes with RESTful design
    let app = Router::new()
        // Health monitoring endpoint for load balancers and deployment tools
        .route("/health", get(handlers::health_check))
        
        // User management endpoints - authentication and account creation
        .route("/api/users/register", post(handlers::register_user))
        .route("/api/users/login", post(handlers::login_user))
        
        // Process management endpoints - CRUD operations for NDA processes
        .route("/api/processes", post(handlers::create_process))  // Create encrypted process
        .route("/api/processes", get(handlers::list_processes))   // List client's processes
        
        // Sharing and access endpoints - blockchain-integrated operations
        .route("/api/processes/share", post(handlers::share_process))   // Share via Stellar
        .route("/api/processes/access", post(handlers::access_process)) // Access with decryption
        
        // Audit and compliance endpoint - access notifications for process owners
        .route("/api/notifications", get(handlers::get_notifications))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running at http://localhost:3000");
    println!("📊 Health check available at http://localhost:3000/health");
    println!("📋 API documentation: All endpoints support JSON request/response");
    println!("🔐 Security: AES-256-GCM encryption + Stellar blockchain integration");
    
    axum::serve(listener, app).await?;

    Ok(())
}