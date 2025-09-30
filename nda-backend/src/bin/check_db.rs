use nda_backend::database::init_database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = init_database().await?;
    
    println!("🔍 Verificando dados após migração...");
    
    // Buscar todos os usuários para verificar a migração
    let users = sqlx::query!(
        "SELECT id, username, roles FROM users"
    )
    .fetch_all(&pool)
    .await?;
    
    println!("\n👥 Usuários no banco de dados:");
    println!("=====================================");
    
    for user in users {
        println!("ID: {}", user.id.as_deref().unwrap_or("N/A"));
        println!("Username: {}", user.username);
        println!("Roles: {}", if user.roles.is_empty() { "N/A" } else { &user.roles });
        println!("-------------------------------------");
    }
    
    Ok(())
}