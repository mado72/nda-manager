use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub stellar_public_key: String,
    pub stellar_secret_key: String, // Em produção, usar KMS
    pub user_type: String, // "client" ou "supplier"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Process {
    pub id: String,
    pub client_id: String,
    pub title: String,
    pub encrypted_content: String,
    pub encryption_key: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProcessShare {
    pub id: String,
    pub process_id: String,
    pub supplier_public_key: String,
    pub stellar_transaction_hash: String,
    pub shared_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProcessAccess {
    pub id: String,
    pub process_id: String,
    pub supplier_id: String,
    pub accessed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProcessAccessWithDetails {
    pub id: String,
    pub process_id: String,
    pub supplier_id: String,
    pub accessed_at: DateTime<Utc>,
    pub process_title: String,
    pub supplier_username: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub user_type: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProcessRequest {
    pub title: String,
    pub confidential_content: String,
    pub client_username: String,  // ← Adicionar este campo
}

#[derive(Debug, Deserialize)]
pub struct ShareProcessRequest {
    pub process_id: String,
    pub supplier_public_key: String,
    pub client_username: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessProcessRequest {
    pub process_id: String,
    pub supplier_public_key: String,
    pub supplier_username: String,
}

// Response models para API
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub stellar_public_key: String,
    pub user_type: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            stellar_public_key: user.stellar_public_key,
            user_type: user.user_type,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProcessResponse {
    pub id: String,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<Process> for ProcessResponse {
    fn from(process: Process) -> Self {
        ProcessResponse {
            id: process.id,
            title: process.title,
            status: process.status,
            created_at: process.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProcessAccessResponse {
    pub process_id: String,
    pub title: String,
    pub content: String,
    pub accessed_at: DateTime<Utc>,
}