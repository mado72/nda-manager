// src/main.rs
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

use handlers::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logging
    tracing_subscriber::fmt::init();

    // Conectar ao banco
    let pool = database::init_database().await?;

    // Estado da aplicação
    let state = Arc::new(AppState { pool });

    // Configurar rotas
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/users/register", post(handlers::register_user))
        .route("/api/users/login", post(handlers::login_user))
        .route("/api/processes", post(handlers::create_process))
        .route("/api/processes", get(handlers::list_processes))
        .route("/api/processes/share", post(handlers::share_process))
        .route("/api/processes/access", post(handlers::access_process))
        .route("/api/notifications", get(handlers::get_notifications))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Servidor rodando em http://localhost:3000");
    
    axum::serve(listener, app).await?;

    Ok(())
}