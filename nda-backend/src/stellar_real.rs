// src/stellar_real.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use stellar_strkey::ed25519;
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct StellarClient {
    horizon_url: String,
    client: Client,
    network_passphrase: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StellarAccount {
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
    pub successful: bool,
    pub ledger: Option<u64>,
    pub result_xdr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub account_id: String,
    pub sequence: String,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: String,
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub id: String,
    pub hash: String,
    pub ledger: u64,
    pub created_at: String,
    pub source_account: String,
    pub memo: Option<String>,
    pub memo_type: Option<String>,
    pub successful: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsResponse {
    #[serde(rename = "_embedded")]
    pub embedded: EmbeddedTransactions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedTransactions {
    pub records: Vec<TransactionRecord>,
}

impl StellarClient {
    pub fn new_testnet() -> Self {
        Self {
            horizon_url: "https://horizon-testnet.stellar.org".to_string(),
            client: Client::new(),
            network_passphrase: "Test SDF Network ; September 2015".to_string(),
        }
    }

    pub fn new_mainnet() -> Self {
        Self {
            horizon_url: "https://horizon.stellar.org".to_string(),
            client: Client::new(),
            network_passphrase: "Public Global Stellar Network ; September 2015".to_string(),
        }
    }

    /// Gera uma nova carteira Stellar usando stellar-strkey
    pub fn generate_keypair() -> Result<StellarAccount, Box<dyn Error>> {
        // Usar OsRng diretamente (compatível com rand 0.7)
        let keypair = Keypair::generate(&mut OsRng);
        
        // Usar stellar-strkey para encoding correto
        let public_key = ed25519::PublicKey(keypair.public.to_bytes()).to_string();
        let secret_key = ed25519::PrivateKey(keypair.secret.to_bytes()).to_string();

        Ok(StellarAccount {
            public_key,
            secret_key,
        })
    }

    /// Extrai chave pública da chave secreta
    pub fn get_public_from_secret(secret_key: &str) -> Result<String, Box<dyn Error>> {
        // Parse da secret key usando stellar-strkey
        let private_key = ed25519::PrivateKey::from_string(secret_key)?;
        
        // Criar keypair do ed25519-dalek
        let secret = SecretKey::from_bytes(&private_key.0)?;
        let public: PublicKey = (&secret).into();
        
        // Converter para formato Stellar
        let stellar_public = ed25519::PublicKey(public.to_bytes());
        
        Ok(stellar_public.to_string())
    }

    /// Financia conta na testnet usando Friendbot
    pub async fn fund_testnet_account(&self, public_key: &str) -> Result<bool, Box<dyn Error>> {
        let url = format!("https://friendbot.stellar.org?addr={}", public_key);
        
        println!("🤖 Financiando conta testnet: {}", public_key);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let success = response.status().is_success();
        
        if success {
            println!("✅ Conta financiada com sucesso!");
            
            // Aguardar um pouco para a transação ser processada
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        } else {
            let error_text = response.text().await.unwrap_or_default();
            println!("❌ Erro ao financiar conta: {}", error_text);
        }

        Ok(success)
    }

    /// Obtém informações da conta
    pub async fn get_account(&self, account_id: &str) -> Result<AccountResponse, Box<dyn Error>> {
        let url = format!("{}/accounts/{}", self.horizon_url, account_id);
        
        println!("🔍 Buscando informações da conta: {}", account_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Conta não encontrada: {} - {}", account_id, error_text).into());
        }

        let account = response.json::<AccountResponse>().await?;
        
        println!("✅ Conta encontrada - Sequence: {}", account.sequence);
        
        Ok(account)
    }

    /// Cria transação de compartilhamento de processo (versão simplificada para MVP)
    pub async fn share_process_transaction(
        &self,
        source_secret: &str,
        destination_public: &str,
        process_id: &str,
        memo: &str,
    ) -> Result<TransactionResponse, Box<dyn Error>> {
        println!("📤 Criando transação de compartilhamento...");
        println!("   Processo: {}", process_id);
        println!("   Destino: {}", destination_public);
        
        // Para MVP, vamos simular uma transação válida
        // Em produção, isso construiria e submeteria uma transação real
        
        let source_public = Self::get_public_from_secret(source_secret)?;
        
        // Verificar se as contas existem
        let _source_account = self.get_account(&source_public).await?;
        let _dest_account = self.get_account(destination_public).await?;
        
        // Simular hash de transação realista
        let transaction_data = format!("{}:{}:{}:{}", source_public, destination_public, process_id, memo);
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        println!("✅ Transação simulada criada: {}", &hash[0..16]);
        
        Ok(TransactionResponse {
            hash: hash[0..64].to_string(),
            successful: true,
            ledger: Some(chrono::Utc::now().timestamp() as u64),
            result_xdr: None,
        })
    }

    /// Verifica se um usuário tem acesso a um processo via blockchain
    pub async fn verify_process_access(
        &self,
        process_id: &str,
        user_public_key: &str,
    ) -> Result<bool, Box<dyn Error>> {
        println!("🔍 Verificando acesso ao processo: {}", process_id);
        println!("   Usuário: {}", user_public_key);
        
        // Buscar transações da conta do usuário
        let transactions = self.get_account_transactions(user_public_key).await?;
        
        // Verificar se existe transação relacionada ao processo
        for tx in transactions {
            if self.transaction_contains_process(&tx, process_id)? {
                println!("✅ Acesso verificado via blockchain!");
                return Ok(true);
            }
        }
        
        println!("❌ Acesso não encontrado na blockchain");
        Ok(false)
    }

    /// Busca transações de uma conta
    async fn get_account_transactions(&self, account_id: &str) -> Result<Vec<TransactionRecord>, Box<dyn Error>> {
        let url = format!("{}/accounts/{}/transactions?limit=200&order=desc", self.horizon_url, account_id);
        
        println!("🔍 Buscando transações da conta...");
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            println!("❌ Erro ao buscar transações: {}", response.status());
            return Ok(vec![]);
        }

        let data: TransactionsResponse = response.json().await?;
        
        println!("✅ Encontradas {} transações", data.embedded.records.len());
        
        Ok(data.embedded.records)
    }

    /// Verifica se transação contém referência ao processo
    fn transaction_contains_process(&self, transaction: &TransactionRecord, process_id: &str) -> Result<bool, Box<dyn Error>> {
        // Verificar memo da transação
        if let Some(memo) = &transaction.memo {
            if memo.contains(process_id) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Testa conectividade com a rede Stellar
    pub async fn test_connection(&self) -> Result<bool, Box<dyn Error>> {
        let url = format!("{}/", self.horizon_url);
        
        println!("🌐 Testando conexão com Stellar Testnet...");
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let success = response.status().is_success();
        
        if success {
            println!("✅ Conexão com Stellar Testnet OK!");
            
            // Mostrar informações da rede
            if let Ok(info) = response.json::<serde_json::Value>().await {
                if let Some(network) = info.get("network_passphrase") {
                    println!("   Network: {}", network);
                }
                if let Some(version) = info.get("horizon_version") {
                    println!("   Horizon Version: {}", version);
                }
            }
        } else {
            println!("❌ Erro na conexão: {}", response.status());
        }

        Ok(success)
    }

    /// Cria conta de teste e financia automaticamente
    pub async fn create_test_account(&self) -> Result<StellarAccount, Box<dyn Error>> {
        println!("🧪 Criando conta de teste...");
        
        // Gerar keypair
        let account = Self::generate_keypair()?;
        
        println!("   Public Key: {}", account.public_key);
        
        // Financiar na testnet
        self.fund_testnet_account(&account.public_key).await?;
        
        // Verificar se a conta foi criada
        match self.get_account(&account.public_key).await {
            Ok(_) => {
                println!("✅ Conta de teste criada e financiada!");
                Ok(account)
            }
            Err(e) => {
                println!("❌ Erro ao verificar conta: {}", e);
                Err(e)
            }
        }
    }

    /// Obtém saldo XLM de uma conta
    pub async fn get_xlm_balance(&self, account_id: &str) -> Result<String, Box<dyn Error>> {
        let account = self.get_account(account_id).await?;
        
        for balance in account.balances {
            if balance.asset_type == "native" {
                return Ok(balance.balance);
            }
        }
        
        Ok("0".to_string())
    }
}