// src/bin/test_stellar.rs
use nda_backend::stellar_real::StellarClient;  // ← Mudança aqui

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Testando Integração Stellar Testnet");
    println!("=====================================");
    
    // Criar cliente Stellar
    let stellar = StellarClient::new_testnet();
    
    // Testar conexão
    stellar.test_connection().await?;
    
    // Criar conta de teste
    println!("\n🧪 Criando conta de teste...");
    let account = stellar.create_test_account().await?;
    
    println!("\n📋 Conta Criada:");
    println!("   Public Key: {}", account.public_key);
    println!("   Secret Key: {}...", &account.secret_key[0..10]);
    
    // Verificar saldo
    let balance = stellar.get_xlm_balance(&account.public_key).await?;
    println!("   Saldo XLM: {}", balance);
    
    println!("\n✅ Teste concluído com sucesso!");
    
    Ok(())
}