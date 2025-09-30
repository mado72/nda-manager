use sqlx::sqlite::SqlitePool;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:./stellar_mvp.db";
    let pool = SqlitePool::connect(&database_url).await?;
    
    println!("🔍 Verificando estrutura da tabela users...");
    
    // Verificar se a coluna roles existe
    let schema = sqlx::query_scalar::<_, String>(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name='users'"
    )
    .fetch_one(&pool)
    .await?;
    
    println!("📋 Schema da tabela users:");
    println!("{}", schema);
    
    // Verificar dados dos usuários
    println!("\n👥 Dados dos usuários:");
    let users = sqlx::query!("SELECT id, username, roles FROM users")
        .fetch_all(&pool)
        .await?;
    
    for user in users {
        println!("ID: {}, Username: {}, Roles: {}", 
            user.id.as_deref().unwrap_or("N/A"), 
            user.username, 
            user.roles.as_deref().unwrap_or("N/A")
        );
    }
    
    pool.close().await;
    Ok(())
}