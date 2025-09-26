// src/handlers.rs
use axum::{
    extract::{State, Json, Query},
    response::Json as ResponseJson,
    http::StatusCode,
};
use std::sync::Arc;
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;  // ← Adicionar esta linha
use sqlx;        // ← Adicionar esta linha

use crate::{
    models::*,
    stellar_real::StellarClient,
    crypto::{generate_key, encrypt_content, decrypt_content},
    database::queries,
};

// Definir AppState aqui mesmo
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

// Query parameters para listar processos
#[derive(Deserialize)]
pub struct ListProcessesQuery {
    pub client_username: Option<String>,
}

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResponseJson<UserResponse>, StatusCode> {
    // Verificar se usuário já existe
    if let Ok(Some(_)) = queries::find_user_by_username(&state.pool, &payload.username).await {
        return Err(StatusCode::CONFLICT);
    }
    
    // Criar conta Stellar real
    let stellar_account = StellarClient::generate_keypair()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Financiar conta na testnet automaticamente
    let stellar_client = StellarClient::new_testnet();
    let _funded = stellar_client
        .fund_testnet_account(&stellar_account.public_key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Criar usuário no banco
    let user = queries::create_user(
        &state.pool,
        &payload.username,
        &stellar_account.public_key,
        &stellar_account.secret_key,
        &payload.user_type,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(user.into()))
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<UserResponse>, StatusCode> {
    let user = queries::find_user_by_username(&state.pool, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(ResponseJson(user.into()))
}

pub async fn create_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProcessRequest>,
) -> Result<ResponseJson<ProcessResponse>, StatusCode> {
    // Buscar cliente pelo username
    let client = queries::find_user_by_username(&state.pool, &payload.client_username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let encryption_key = generate_key();
    let encrypted_content = encrypt_content(&payload.confidential_content, &encryption_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let process = queries::create_process(
        &state.pool,
        &client.id,
        &payload.title,
        &encrypted_content,
        &encryption_key,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(process.into()))
}

pub async fn share_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ShareProcessRequest>,
) -> Result<ResponseJson<ProcessShare>, StatusCode> {
    let stellar_client = StellarClient::new_testnet();
    
    // Buscar processo
    let _process = queries::find_process_by_id(&state.pool, &payload.process_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Buscar cliente pelo username
    let client = queries::find_user_by_username(&state.pool, &payload.client_username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Enviar transação Stellar real
    let tx_result = stellar_client
        .share_process_transaction(
            &client.stellar_secret_key,
            &payload.supplier_public_key,
            &payload.process_id,
            &format!("NDA_SHARE:{}", payload.process_id),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Registrar compartilhamento
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

// src/handlers.rs - Substituir a função access_process

pub async fn access_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AccessProcessRequest>,
) -> Result<ResponseJson<ProcessAccessResponse>, StatusCode> {
    // Buscar processo com campos específicos
    let process = sqlx::query!(
        r#"
        SELECT id, client_id, title, encrypted_content, encryption_key, status, created_at
        FROM processes WHERE id = ?
        "#,
        payload.process_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Buscar fornecedor com campos específicos
    let supplier = sqlx::query!(
        r#"
        SELECT id, username, stellar_public_key, stellar_secret_key, user_type, created_at
        FROM users WHERE username = ?
        "#,
        payload.supplier_username
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Verificar se existe compartilhamento no banco
    let share_exists = sqlx::query!(
        "SELECT id FROM process_shares WHERE process_id = ? AND supplier_public_key = ?",
        payload.process_id,
        supplier.stellar_public_key
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if share_exists.is_none() {
        println!("❌ Acesso negado: Processo não foi compartilhado com este fornecedor");
        return Err(StatusCode::FORBIDDEN);
    }

    println!("✅ Acesso autorizado: Compartilhamento encontrado no banco");

    // Descriptografar conteúdo
    let decrypted_content = decrypt_content(&process.encrypted_content, &process.encryption_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Registrar acesso
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

    println!("📊 Acesso registrado com sucesso");

    let response = ProcessAccessResponse {
        process_id: payload.process_id,
        title: process.title,
        content: decrypted_content,
        accessed_at: now,
    };

    Ok(ResponseJson(response))
}

pub async fn list_processes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListProcessesQuery>,
) -> Result<ResponseJson<Vec<ProcessResponse>>, StatusCode> {
    let client_username = params.client_username.ok_or(StatusCode::BAD_REQUEST)?;
    
    // Buscar cliente pelo username
    let client = queries::find_user_by_username(&state.pool, &client_username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let processes = queries::list_processes_by_client(&state.pool, &client.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<ProcessResponse> = processes.into_iter().map(|p| p.into()).collect();
    Ok(ResponseJson(response))
}

pub async fn get_notifications(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListProcessesQuery>,
) -> Result<ResponseJson<Vec<ProcessAccessWithDetails>>, StatusCode> {
    let client_username = params.client_username.ok_or(StatusCode::BAD_REQUEST)?;
    
    // Buscar cliente pelo username
    let client = queries::find_user_by_username(&state.pool, &client_username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let notifications = queries::list_process_accesses_by_client(&state.pool, &client.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(notifications))
}