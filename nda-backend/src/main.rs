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
//! - `POST /api/users/auto-login` - Automatic login using localStorage data
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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod handlers;
mod database;
mod crypto;
mod stellar_real;
mod auth;
mod jwt;

use handlers::{AppState, ListProcessesQuery};
use models::*;

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health_check,
        handlers::register_user,
        handlers::login_user,
        handlers::auto_login_user,
        handlers::refresh_token,
        handlers::logout_user,
        handlers::create_process,
        handlers::share_process,
        handlers::access_process,
        handlers::list_processes,
        handlers::get_notifications,
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            AutoLoginRequest,
            RefreshTokenRequest,
            LogoutRequest,
            CreateProcessRequest,
            ShareProcessRequest,
            AccessProcessRequest,
            UserResponse,
            LoginResponse,
            ProcessResponse,
            ProcessShare,
            ProcessAccessResponse,
            ProcessAccessWithDetails,
            HealthResponse,
            ListProcessesQuery,
            jwt::Claims,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "User Management", description = "User registration and authentication"),
        (name = "Process Management", description = "NDA process creation and listing"),
        (name = "Sharing & Access", description = "Blockchain-secured sharing and content access"),
        (name = "Audit & Compliance", description = "Access notifications and audit trails")
    ),
    info(
        title = "NDA Backend API",
        version = "1.0.0",
        description = "Blockchain-secured Non-Disclosure Agreement (NDA) contract management system with AES-256-GCM encryption and Stellar network integration.",
        contact(
            name = "API Support",
            email = "support@nda-backend.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
struct ApiDoc;

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

    // Get JWT secret from environment or use default (WARNING: use strong secret in production)
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| {
            tracing::warn!("JWT_SECRET not set, using default (NOT SECURE FOR PRODUCTION)");
            "default-jwt-secret-change-this-in-production-min-32-chars".to_string()
        });

    // Create token blacklist for logout/revocation
    let token_blacklist = jwt::TokenBlacklist::new();

    // Create application state for dependency injection
    let state = Arc::new(AppState { 
        pool,
        jwt_secret,
        token_blacklist,
    });

    // Configure API routes with RESTful design
    let app = Router::new()
        // Health monitoring endpoint for load balancers and deployment tools
        .route("/health", get(handlers::health_check))
        
        // User management endpoints - authentication and account creation
        .route("/api/users/register", post(handlers::register_user))
        .route("/api/users/login", post(handlers::login_user))
        .route("/api/users/auto-login", post(handlers::auto_login_user))
        .route("/api/users/refresh", post(handlers::refresh_token))
        .route("/api/users/logout", post(handlers::logout_user))
        
        // Process management endpoints - CRUD operations for NDA processes
        .route("/api/processes", post(handlers::create_process))  // Create encrypted process
        .route("/api/processes", get(handlers::list_processes))   // List client's processes
        
        // Sharing and access endpoints - blockchain-integrated operations
        .route("/api/processes/share", post(handlers::share_process))   // Share via Stellar
        .route("/api/processes/access", post(handlers::access_process)) // Access with decryption
        
        // Audit and compliance endpoint - access notifications for process owners
        .route("/api/notifications", get(handlers::get_notifications))
        
        // Swagger UI for API documentation
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running at http://localhost:3000");
    println!("📊 Health check available at http://localhost:3000/health");
    println!("📖 Swagger UI available at http://localhost:3000/swagger-ui");
    println!("📄 OpenAPI spec at http://localhost:3000/api-docs/openapi.json");
    println!("🔐 Security: JWT authentication + AES-256-GCM encryption + Stellar blockchain");
    
    axum::serve(listener, app).await?;

    Ok(())
}