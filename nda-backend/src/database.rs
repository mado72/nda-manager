// src/database.rs
use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite};
use std::error::Error;
use chrono::{DateTime, Utc};

pub async fn init_database() -> Result<SqlitePool, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./stellar_mvp.db".to_string());
    
    // Criar banco se não existir
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        println!("🔨 Criando banco de dados...");
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("✅ Banco criado com sucesso"),
            Err(error) => panic!("❌ Erro ao criar banco: {}", error),
        }
    }
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Executar migrações
    run_migrations(&pool).await?;
    
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("🔄 Executando migrações...");
    
    // Criar tabela de usuários
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            stellar_public_key TEXT UNIQUE NOT NULL,
            stellar_secret_key TEXT NOT NULL,
            user_type TEXT NOT NULL CHECK (user_type IN ('client', 'supplier')),
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Criar tabela de processos
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

    // Criar tabela de compartilhamentos
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS process_shares (
            id TEXT PRIMARY KEY,
            process_id TEXT NOT NULL,
            supplier_public_key TEXT NOT NULL,
            stellar_transaction_hash TEXT NOT NULL,
            shared_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Criar tabela de acessos
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS process_accesses (
            id TEXT PRIMARY KEY,
            process_id TEXT NOT NULL,
            supplier_id TEXT NOT NULL,
            accessed_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("✅ Migrações executadas com sucesso!");
    Ok(())
}

// Função auxiliar para converter DateTime para string
fn datetime_to_string(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

// Função auxiliar para converter string para DateTime
fn string_to_datetime(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc))
}

// Módulo de queries
pub mod queries {
    use super::*;
    use crate::models::*;
    use uuid::Uuid;
    use sqlx::Row;

    pub async fn create_user(
        pool: &SqlitePool,
        username: &str,
        stellar_public_key: &str,
        stellar_secret_key: &str,
        user_type: &str,
    ) -> Result<User, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let created_at_str = datetime_to_string(&created_at);

        sqlx::query(
            r#"
            INSERT INTO users (id, username, stellar_public_key, stellar_secret_key, user_type, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(&id)
        .bind(username)
        .bind(stellar_public_key)
        .bind(stellar_secret_key)
        .bind(user_type)
        .bind(&created_at_str)
        .execute(pool)
        .await?;

        Ok(User {
            id,
            username: username.to_string(),
            stellar_public_key: stellar_public_key.to_string(),
            stellar_secret_key: stellar_secret_key.to_string(),
            user_type: user_type.to_string(),
            created_at,
        })
    }

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
                    stellar_public_key: row.get("stellar_public_key"),
                    stellar_secret_key: row.get("stellar_secret_key"),
                    user_type: row.get("user_type"),
                    created_at,
                }))
            },
            None => Ok(None),
        }
    }

    pub async fn create_process(
        pool: &SqlitePool,
        client_id: &str,
        title: &str,
        encrypted_content: &str,
        encryption_key: &str,
    ) -> Result<Process, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let created_at_str = datetime_to_string(&created_at);
        let status = "active".to_string();

        sqlx::query(
            r#"
            INSERT INTO processes (id, client_id, title, encrypted_content, encryption_key, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(&id)
        .bind(client_id)
        .bind(title)
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
            encrypted_content: encrypted_content.to_string(),
            encryption_key: encryption_key.to_string(),
            status,
            created_at,
        })
    }

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
                    encrypted_content: row.get("encrypted_content"),
                    encryption_key: row.get("encryption_key"),
                    status: row.get("status"),
                    created_at,
                }))
            },
            None => Ok(None),
        }
    }

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
                encrypted_content: row.get("encrypted_content"),
                encryption_key: row.get("encryption_key"),
                status: row.get("status"),
                created_at,
            });
        }

        Ok(processes)
    }

    pub async fn create_process_share(
        pool: &SqlitePool,
        process_id: &str,
        supplier_public_key: &str,
        stellar_transaction_hash: &str,
    ) -> Result<ProcessShare, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let shared_at = Utc::now();
        let shared_at_str = datetime_to_string(&shared_at);

        sqlx::query(
            r#"
            INSERT INTO process_shares (id, process_id, supplier_public_key, stellar_transaction_hash, shared_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .bind(&id)
        .bind(process_id)
        .bind(supplier_public_key)
        .bind(stellar_transaction_hash)
        .bind(&shared_at_str)
        .execute(pool)
        .await?;

        Ok(ProcessShare {
            id,
            process_id: process_id.to_string(),
            supplier_public_key: supplier_public_key.to_string(),
            stellar_transaction_hash: stellar_transaction_hash.to_string(),
            shared_at,
        })
    }

    pub async fn create_process_access(
        pool: &SqlitePool,
        process_id: &str,
        supplier_id: &str,
    ) -> Result<ProcessAccess, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let accessed_at = Utc::now();
        let accessed_at_str = datetime_to_string(&accessed_at);

        sqlx::query(
            r#"
            INSERT INTO process_accesses (id, process_id, supplier_id, accessed_at)
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(&id)
        .bind(process_id)
        .bind(supplier_id)
        .bind(&accessed_at_str)
        .execute(pool)
        .await?;

        Ok(ProcessAccess {
            id,
            process_id: process_id.to_string(),
            supplier_id: supplier_id.to_string(),
            accessed_at,
        })
    }

    pub async fn list_process_accesses_by_client(
        pool: &SqlitePool,
        client_id: &str,
    ) -> Result<Vec<ProcessAccessWithDetails>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                pa.id,
                pa.process_id,
                pa.supplier_id,
                pa.accessed_at,
                p.title as process_title,
                u.username as supplier_username
            FROM process_accesses pa
            JOIN processes p ON pa.process_id = p.id
            JOIN users u ON pa.supplier_id = u.id
            WHERE p.client_id = ?1
            ORDER BY pa.accessed_at DESC
            "#,
        )
        .bind(client_id)
        .fetch_all(pool)
        .await?;

        let mut accesses = Vec::new();
        for row in rows {
            let accessed_at_str: String = row.get("accessed_at");
            let accessed_at = string_to_datetime(&accessed_at_str)
                .map_err(|_| sqlx::Error::ColumnDecode { 
                    index: "accessed_at".to_string(), 
                    source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid datetime")) 
                })?;

            accesses.push(ProcessAccessWithDetails {
                id: row.get("id"),
                process_id: row.get("process_id"),
                supplier_id: row.get("supplier_id"),
                accessed_at,
                process_title: row.get("process_title"),
                supplier_username: row.get("supplier_username"),
            });
        }

        Ok(accesses)
    }

}

