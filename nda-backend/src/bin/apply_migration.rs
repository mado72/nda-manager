use sqlx::sqlite::SqlitePool;
use tokio;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:./stellar_mvp.db";
    let pool = SqlitePool::connect(&database_url).await?;
    
    println!("🔄 Aplicando migração para adicionar coluna description...");
    
    // Ler o arquivo de migração
    let migration_content = fs::read_to_string("migrations/20250929000001_add_process_description.sql")
        .expect("Erro ao ler arquivo de migração");
    
    // Dividir por declarações separadas
    let statements = migration_content.split(';');
    
    for statement in statements {
        let statement = statement.trim();
        if !statement.is_empty() {
            println!("Executando: {}", statement);
            sqlx::query(statement)
                .execute(&pool)
                .await
                .map_err(|e| {
                    println!("Erro ao executar statement: {}", e);
                    e
                })?;
        }
    }
    
    println!("✅ Migração aplicada com sucesso!");
    
    // Verificar se a coluna foi adicionada
    println!("\n🔍 Verificando estrutura da tabela processes...");
    let schema = sqlx::query_scalar::<_, String>(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name='processes'"
    )
    .fetch_one(&pool)
    .await?;
    
    println!("📋 Schema da tabela processes:");
    println!("{}", schema);
    
    Ok(())
}