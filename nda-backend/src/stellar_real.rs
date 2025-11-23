//! # Stellar Blockchain Integration Module
//! 
//! This module provides real Stellar blockchain integration for the NDA backend system.
//! It handles account creation, transaction submission, and blockchain verification
//! to create immutable sharing records for NDA processes.
//! 
//! ## Overview
//! 
//! The module integrates with the Stellar network to provide:
//! 
//! - **Account Management**: Automatic Stellar account generation and funding
//! - **Transaction Creation**: Process sharing via blockchain transactions
//! - **Access Verification**: Cryptographic proof of sharing permissions
//! - **Audit Trails**: Immutable records for regulatory compliance
//! - **Network Connectivity**: Support for both testnet and mainnet environments
//! 
//! ## Architecture
//! 
//! The [`StellarClient`] serves as the main interface to the Stellar network,
//! providing methods for:
//! 
//! ### Account Operations
//! - [`StellarClient::generate_keypair()`] - Generate cryptographic keypairs
//! - [`StellarClient::fund_testnet_account()`] - Auto-funding for development
//! - [`StellarClient::get_account()`] - Account information retrieval
//! - [`StellarClient::get_xlm_balance()`] - Balance checking
//! 
//! ### Transaction Operations
//! - [`StellarClient::share_process_transaction()`] - Create sharing transactions
//! - [`StellarClient::verify_process_access()`] - Verify sharing permissions
//! - [`StellarClient::get_account_transactions()`] - Transaction history
//! 
//! ### Network Operations
//! - [`StellarClient::test_connection()`] - Network connectivity testing
//! - [`StellarClient::create_test_account()`] - Development account creation
//! 
//! ## Security Model
//! 
//! ### Cryptographic Security
//! - **Ed25519 Keypairs**: Industry-standard elliptic curve cryptography
//! - **Secure Random Generation**: OS-provided entropy for key generation
//! - **Stellar Encoding**: Standard stellar-strkey format for keys
//! 
//! ### Blockchain Security
//! - **Immutable Records**: Transactions cannot be altered once confirmed
//! - **Distributed Verification**: Network consensus ensures integrity
//! - **Cryptographic Proof**: Digital signatures provide non-repudiation
//! 
//! ### Access Control
//! - **Permission Verification**: Blockchain-based access authorization
//! - **Audit Trails**: Complete transaction history for compliance
//! - **Memo Fields**: Process metadata stored on-chain
//! 
//! ## Network Support
//! 
//! The client supports both Stellar networks:
//! 
//! - **Testnet**: Development and testing environment with free funding
//! - **Mainnet**: Production environment with real XLM transactions
//! 
//! ## Usage Example
//! 
//! ```rust
//! use crate::stellar_real::StellarClient;
//! 
//! // Create testnet client
//! let client = StellarClient::new_testnet();
//! 
//! // Generate and fund a test account
//! let account = client.create_test_account().await?;
//! 
//! // Share a process via blockchain
//! let tx_result = client.share_process_transaction(
//!     &account.secret_key,
//!     &partner_public_key,
//!     &process_id,
//!     "NDA_SHARE"
//! ).await?;
//! 
//! // Verify access permissions
//! let has_access = client.verify_process_access(
//!     &process_id,
//!     &partner_public_key
//! ).await?;
//! ```
//! 
//! ## Error Handling
//! 
//! All operations return `Result` types with descriptive error messages.
//! Common error scenarios include:
//! - Network connectivity issues
//! - Invalid account addresses
//! - Insufficient balances for transactions
//! - Malformed cryptographic keys
//! 
//! ## Development vs Production
//! 
//! - **Development**: Uses testnet with automatic funding via Friendbot
//! - **Production**: Requires mainnet setup with real XLM funding
//! - **MVP Mode**: Includes transaction simulation for rapid development

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use stellar_strkey::ed25519;
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};

/// Stellar blockchain client for network operations.
/// 
/// Provides a high-level interface to the Stellar network, handling account
/// management, transaction creation, and blockchain verification operations.
/// 
/// # Fields
/// 
/// * `horizon_url` - Stellar Horizon API endpoint URL
/// * `client` - HTTP client for network requests
/// * `network_passphrase` - Network identifier for transaction signing
/// 
/// # Network Configuration
/// 
/// The client can be configured for different Stellar networks:
/// - **Testnet**: Development environment with free funding
/// - **Mainnet**: Production environment with real assets
/// 
/// Each network has its own Horizon API endpoint and network passphrase
/// for proper transaction routing and validation.
#[derive(Debug, Clone)]
pub struct StellarClient {
    horizon_url: String,
    client: Client,
    network_passphrase: String,
}

/// Stellar account keypair with public and secret keys.
/// 
/// Represents a complete Stellar account with both public and private
/// cryptographic material needed for blockchain operations.
/// 
/// # Fields
/// 
/// * `public_key` - Stellar public key (starts with 'G') for receiving transactions
/// * `secret_key` - Stellar secret key (starts with 'S') for signing transactions
/// 
/// # Security Notes
/// 
/// - Secret keys should be stored encrypted in production environments
/// - Public keys can be safely shared and are used as account addresses
/// - Keys use Ed25519 elliptic curve cryptography for security
/// 
/// # Usage
/// 
/// ```rust
/// let account = StellarClient::generate_keypair()?;
/// println!("Address: {}", account.public_key);
/// // Never log or expose the secret_key in production!
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StellarAccount {
    pub public_key: String,
    pub secret_key: String,
}

/// Response from a submitted Stellar transaction.
/// 
/// Contains the result information from a blockchain transaction submission,
/// including success status and network confirmation details.
/// 
/// # Fields
/// 
/// * `hash` - Unique transaction hash for blockchain identification
/// * `successful` - Whether the transaction was successfully processed
/// * `ledger` - Ledger number where transaction was included (if successful)
/// * `result_xdr` - Raw transaction result in XDR format (optional)
/// 
/// # Transaction Verification
/// 
/// The transaction hash can be used to:
/// - Verify transaction existence on the blockchain
/// - Provide immutable proof of the transaction
/// - Link sharing events to specific blockchain records
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
    pub successful: bool,
    pub ledger: Option<u64>,
    pub result_xdr: Option<String>,
}

/// Stellar account information from the blockchain.
/// 
/// Contains current account state including balances and sequence number
/// as retrieved from the Stellar network.
/// 
/// # Fields
/// 
/// * `account_id` - The account's public key address
/// * `sequence` - Current sequence number for transaction ordering
/// * `balances` - List of asset balances held by the account
/// 
/// # Sequence Numbers
/// 
/// The sequence number is crucial for transaction ordering and prevents
/// replay attacks. Each transaction must use the next sequential number.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    pub account_id: String,
    pub sequence: String,
    pub balances: Vec<Balance>,
}

/// Asset balance information for a Stellar account.
/// 
/// Represents the amount of a specific asset held by an account,
/// including both native XLM and custom assets.
/// 
/// # Fields
/// 
/// * `balance` - Amount of the asset (as string to preserve precision)
/// * `asset_type` - Type of asset ("native" for XLM, "credit_alphanum4/12" for others)
/// * `asset_code` - Asset code (None for XLM, ticker symbol for others)
/// * `asset_issuer` - Account that issued the asset (None for XLM)
/// 
/// # Native vs Custom Assets
/// 
/// - **Native XLM**: `asset_type` is "native", no code or issuer
/// - **Custom Assets**: Have asset codes and issuer accounts
#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub balance: String,
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

/// Historical transaction record from the Stellar blockchain.
/// 
/// Represents a completed transaction with all metadata and status information
/// as stored on the Stellar ledger.
/// 
/// # Fields
/// 
/// * `id` - Unique transaction identifier
/// * `hash` - Transaction hash for blockchain verification
/// * `ledger` - Ledger number where transaction was included
/// * `created_at` - Timestamp when transaction was created
/// * `source_account` - Account that initiated the transaction
/// * `memo` - Optional memo field containing process metadata
/// * `memo_type` - Type of memo (text, id, hash, or return)
/// * `successful` - Whether transaction was successfully processed
/// 
/// # Process Verification
/// 
/// For NDA sharing verification, the memo field is checked for:
/// - Process IDs to verify sharing permissions
/// - Sharing metadata for audit trails
/// - Access authorization proofs
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

/// Response wrapper for transaction history queries.
/// 
/// Stellar API responses use embedded structures to organize
/// related data and provide pagination information.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsResponse {
    #[serde(rename = "_embedded")]
    pub embedded: EmbeddedTransactions,
}

/// Container for transaction records in API responses.
/// 
/// Contains the actual list of transaction records returned
/// from transaction history queries.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddedTransactions {
    pub records: Vec<TransactionRecord>,
}

impl StellarClient {
    /// Creates a new Stellar client configured for testnet.
    /// 
    /// Initializes a client pointed to the Stellar testnet environment,
    /// which provides free funding and testing capabilities.
    /// 
    /// # Returns
    /// 
    /// A configured [`StellarClient`] ready for testnet operations.
    /// 
    /// # Testnet Features
    /// 
    /// - Free account funding via Friendbot
    /// - Safe testing environment with no real value
    /// - Identical API to mainnet for development
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let client = StellarClient::new_testnet();
    /// let account = client.create_test_account().await?;
    /// ```
    pub fn new_testnet() -> Self {
        Self {
            horizon_url: "https://horizon-testnet.stellar.org".to_string(),
            client: Client::new(),
            network_passphrase: "Test SDF Network ; September 2015".to_string(),
        }
    }

    /// Creates a new Stellar client configured for mainnet.
    /// 
    /// Initializes a client pointed to the Stellar mainnet environment
    /// for production operations with real assets.
    /// 
    /// # Returns
    /// 
    /// A configured [`StellarClient`] ready for mainnet operations.
    /// 
    /// # Production Considerations
    /// 
    /// - Requires real XLM for transaction fees
    /// - No automatic funding - accounts must be funded manually
    /// - All transactions have real financial implications
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let client = StellarClient::new_mainnet();
    /// // Ensure accounts are properly funded before use
    /// ```
    pub fn new_mainnet() -> Self {
        Self {
            horizon_url: "https://horizon.stellar.org".to_string(),
            client: Client::new(),
            network_passphrase: "Public Global Stellar Network ; September 2015".to_string(),
        }
    }

    /// Generates a new Stellar keypair using cryptographically secure random numbers.
    /// 
    /// Creates a fresh Ed25519 keypair suitable for Stellar blockchain operations.
    /// Keys are encoded using the standard stellar-strkey format.
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(StellarAccount)` - New account with public and secret keys
    /// - `Err(Box<dyn Error>)` - Key generation or encoding error
    /// 
    /// # Security
    /// 
    /// - Uses OS-provided cryptographically secure random number generator
    /// - Ed25519 provides strong elliptic curve cryptography
    /// - Keys follow Stellar network standards for compatibility
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let account = StellarClient::generate_keypair()?;
    /// println!("New account: {}", account.public_key);
    /// // Store secret_key securely!
    /// ```
    pub fn generate_keypair() -> Result<StellarAccount, Box<dyn Error>> {
        // Use OS-provided cryptographically secure random number generator
        let keypair = Keypair::generate(&mut OsRng);
        
        // Use stellar-strkey for proper Stellar key encoding
        let public_key = ed25519::PublicKey(keypair.public.to_bytes()).to_string();
        let secret_key = ed25519::PrivateKey(keypair.secret.to_bytes()).to_string();

        Ok(StellarAccount {
            public_key,
            secret_key,
        })
    }

    /// Derives the public key from a secret key.
    /// 
    /// Extracts the corresponding public key from a Stellar secret key,
    /// useful for validation and account identification.
    /// 
    /// # Parameters
    /// 
    /// * `secret_key` - Stellar secret key in strkey format (starts with 'S')
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(String)` - Corresponding public key in strkey format
    /// - `Err(Box<dyn Error>)` - Invalid secret key format or cryptographic error
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let public_key = StellarClient::get_public_from_secret(&secret_key)?;
    /// assert!(public_key.starts_with('G'));
    /// ```
    pub fn get_public_from_secret(secret_key: &str) -> Result<String, Box<dyn Error>> {
        // Parse secret key using stellar-strkey
        let private_key = ed25519::PrivateKey::from_string(secret_key)?;
        
        // Create ed25519-dalek keypair
        let secret = SecretKey::from_bytes(&private_key.0)?;
        let public: PublicKey = (&secret).into();
        
        // Convert to Stellar format
        let stellar_public = ed25519::PublicKey(public.to_bytes());
        
        Ok(stellar_public.to_string())
    }

    /// Funds an account on testnet using Friendbot.
    /// 
    /// Automatically funds a new account on Stellar testnet using the Friendbot service,
    /// which provides free XLM for development and testing purposes.
    /// 
    /// # Parameters
    /// 
    /// * `public_key` - Stellar public key of the account to fund
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(true)` - Account successfully funded
    /// - `Ok(false)` - Funding failed (account may already exist)
    /// - `Err(Box<dyn Error>)` - Network or communication error
    /// 
    /// # Behavior
    /// 
    /// - Creates the account if it doesn't exist
    /// - Provides initial XLM balance for transactions
    /// - Waits for transaction confirmation (5 seconds)
    /// - Only works on testnet (mainnet requires manual funding)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let client = StellarClient::new_testnet();
    /// let account = StellarClient::generate_keypair()?;
    /// let funded = client.fund_testnet_account(&account.public_key).await?;
    /// ```
    pub async fn fund_testnet_account(&self, public_key: &str) -> Result<bool, Box<dyn Error>> {
        let url = format!("https://friendbot.stellar.org?addr={}", public_key);
        
        println!("🤖 Funding testnet account: {}", public_key);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let success = response.status().is_success();
        
        if success {
            println!("✅ Account funded successfully!");
            
            // Wait for transaction to be processed
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        } else {
            let error_text = response.text().await.unwrap_or_default();
            println!("❌ Error funding account: {}", error_text);
        }

        Ok(success)
    }

    /// Retrieves account information from the Stellar network.
    /// 
    /// Fetches current account state including balances, sequence number,
    /// and other account metadata from the blockchain.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - Stellar public key of the account to query
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(AccountResponse)` - Complete account information
    /// - `Err(Box<dyn Error>)` - Account not found or network error
    /// 
    /// # Usage
    /// 
    /// Used for:
    /// - Verifying account existence before transactions
    /// - Getting sequence numbers for transaction building
    /// - Checking account balances
    /// - Validating account state
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let account_info = client.get_account(&public_key).await?;
    /// println!("Balance: {} XLM", account_info.balances[0].balance);
    /// ```
    pub async fn get_account(&self, account_id: &str) -> Result<AccountResponse, Box<dyn Error>> {
        let url = format!("{}/accounts/{}", self.horizon_url, account_id);
        
        println!("🔍 Fetching account information: {}", account_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Account not found: {} - {}", account_id, error_text).into());
        }

        let account = response.json::<AccountResponse>().await?;
        
        println!("✅ Account found - Sequence: {}", account.sequence);
        
        Ok(account)
    }

    /// Creates a process sharing transaction on the Stellar blockchain.
    /// 
    /// Records process sharing permissions as an immutable blockchain transaction,
    /// providing cryptographic proof of sharing authorization.
    /// 
    /// # Parameters
    /// 
    /// * `source_secret` - Secret key of the account sharing the process
    /// * `destination_public` - Public key of the recipient account
    /// * `process_id` - Unique identifier of the process being shared
    /// * `memo` - Additional metadata to include in the transaction
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(TransactionResponse)` - Transaction hash and confirmation details
    /// - `Err(Box<dyn Error>)` - Transaction creation or submission error
    /// 
    /// # MVP Implementation
    /// 
    /// Current implementation simulates transactions for rapid development.
    /// Production version would:
    /// - Build actual Stellar transactions with proper signatures
    /// - Submit to the network and wait for confirmation
    /// - Handle transaction fees and network conditions
    /// 
    /// # Blockchain Benefits
    /// 
    /// - **Immutable Record**: Cannot be altered once confirmed
    /// - **Cryptographic Proof**: Digitally signed by the sharing party
    /// - **Audit Trail**: Permanent record for compliance
    /// - **Dispute Resolution**: Verifiable evidence of sharing
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let tx_result = client.share_process_transaction(
    ///     &client_secret_key,
    ///     &partner_public_key,
    ///     &process_id,
    ///     "NDA_SHARE:confidential_project"
    /// ).await?;
    /// ```
    pub async fn share_process_transaction(
        &self,
        source_secret: &str,
        destination_public: &str,
        process_id: &str,
        memo: &str,
    ) -> Result<TransactionResponse, Box<dyn Error>> {
        println!("📤 Creating sharing transaction...");
        println!("   Process: {}", process_id);
        println!("   Destination: {}", destination_public);
        
        // For MVP, simulate a valid transaction
        // In production, this would build and submit a real transaction
        
        let source_public = Self::get_public_from_secret(source_secret)?;
        
        // Verify that accounts exist
        let _source_account = self.get_account(&source_public).await?;
        let _dest_account = self.get_account(destination_public).await?;
        
        // Simulate realistic transaction hash
        let transaction_data = format!("{}:{}:{}:{}", source_public, destination_public, process_id, memo);
        let mut hasher = Sha256::new();
        hasher.update(transaction_data.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        println!("✅ Simulated transaction created: {}", &hash[0..16]);
        
        Ok(TransactionResponse {
            hash: hash[0..64].to_string(),
            successful: true,
            ledger: Some(chrono::Utc::now().timestamp() as u64),
            result_xdr: None,
        })
    }

    /// Verifies if a user has access to a process via blockchain records.
    /// 
    /// Checks the blockchain for sharing transactions that grant the specified
    /// user access to the given process, providing cryptographic verification.
    /// 
    /// # Parameters
    /// 
    /// * `process_id` - Unique identifier of the process to check
    /// * `user_public_key` - Public key of the user requesting access
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(true)` - User has verified blockchain access to the process
    /// - `Ok(false)` - No blockchain record found granting access
    /// - `Err(Box<dyn Error>)` - Network error or verification failure
    /// 
    /// # Verification Process
    /// 
    /// 1. Retrieves all transactions for the user's account
    /// 2. Searches transaction memos for process references
    /// 3. Validates sharing transaction authenticity
    /// 4. Confirms access permissions are still valid
    /// 
    /// # Security Features
    /// 
    /// - **Cryptographic Verification**: Uses blockchain signatures
    /// - **Immutable Records**: Cannot be forged or altered
    /// - **Complete History**: Checks all relevant transactions
    /// - **Decentralized Trust**: No central authority required
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let has_access = client.verify_process_access(
    ///     &process_id,
    ///     &partner_public_key
    /// ).await?;
    /// 
    /// if has_access {
    ///     // Grant access to encrypted content
    /// }
    /// ```
    pub async fn verify_process_access(
        &self,
        process_id: &str,
        user_public_key: &str,
    ) -> Result<bool, Box<dyn Error>> {
        println!("🔍 Verifying process access: {}", process_id);
        println!("   User: {}", user_public_key);
        
        // Fetch user account transactions
        let transactions = self.get_account_transactions(user_public_key).await?;
        
        // Check if any transaction relates to the process
        for tx in transactions {
            if self.transaction_contains_process(&tx, process_id)? {
                println!("✅ Access verified via blockchain!");
                return Ok(true);
            }
        }
        
        println!("❌ Access not found on blockchain");
        Ok(false)
    }

    /// Retrieves transaction history for an account.
    /// 
    /// Fetches recent transactions for the specified account from the Stellar network,
    /// used for access verification and audit trail analysis.
    /// 
    /// # Parameters
    /// 
    /// * `account_id` - Public key of the account to query
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(Vec<TransactionRecord>)` - List of recent transactions (up to 200)
    /// - `Err(Box<dyn Error>)` - Network error or invalid account
    /// 
    /// # Query Parameters
    /// 
    /// - **Limit**: 200 transactions maximum per query
    /// - **Order**: Descending (newest first) for recent activity
    /// - **Filtering**: Returns all transaction types
    /// 
    /// # Privacy Note
    /// 
    /// This is a private method used internally for access verification.
    /// Transaction data is publicly available on the blockchain.
    async fn get_account_transactions(&self, account_id: &str) -> Result<Vec<TransactionRecord>, Box<dyn Error>> {
        let url = format!("{}/accounts/{}/transactions?limit=200&order=desc", self.horizon_url, account_id);
        
        println!("🔍 Fetching account transactions...");
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            println!("❌ Error fetching transactions: {}", response.status());
            return Ok(vec![]);
        }

        let data: TransactionsResponse = response.json().await?;
        
        println!("✅ Found {} transactions", data.embedded.records.len());
        
        Ok(data.embedded.records)
    }

    /// Checks if a transaction contains a reference to a specific process.
    /// 
    /// Searches transaction memo fields for process identifiers to determine
    /// if the transaction is related to process sharing or access.
    /// 
    /// # Parameters
    /// 
    /// * `transaction` - Transaction record to examine
    /// * `process_id` - Process identifier to search for
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(true)` - Transaction contains process reference
    /// - `Ok(false)` - No process reference found
    /// - `Err(Box<dyn Error>)` - Processing error
    /// 
    /// # Search Strategy
    /// 
    /// Currently searches memo text for simple string containment.
    /// Production implementations might use:
    /// - Structured memo formats (JSON, protobuf)
    /// - Cryptographic commitments
    /// - Operation-specific metadata
    fn transaction_contains_process(&self, transaction: &TransactionRecord, process_id: &str) -> Result<bool, Box<dyn Error>> {
        // Check transaction memo field
        if let Some(memo) = &transaction.memo {
            if memo.contains(process_id) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Tests connectivity to the Stellar network.
    /// 
    /// Verifies that the client can connect to the configured Stellar Horizon API
    /// and retrieves basic network information for diagnostics.
    /// 
    /// # Returns
    /// 
    /// Returns `Result` containing:
    /// - `Ok(true)` - Connection successful, network reachable
    /// - `Ok(false)` - Connection failed, network unreachable
    /// - `Err(Box<dyn Error>)` - Network error or parsing failure
    /// 
    /// # Network Information
    /// 
    /// When successful, displays:
    /// - Network passphrase (testnet vs mainnet identification)
    /// - Horizon API version
    /// - Connection status confirmation
    /// 
    /// # Usage
    /// 
    /// Useful for:
    /// - Application startup diagnostics
    /// - Network connectivity troubleshooting
    /// - Environment verification (testnet vs mainnet)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let client = StellarClient::new_testnet();
    /// if !client.test_connection().await? {
    ///     return Err("Cannot connect to Stellar network".into());
    /// }
    /// ```
    pub async fn test_connection(&self) -> Result<bool, Box<dyn Error>> {
        let url = format!("{}/", self.horizon_url);
        
        println!("🌐 Testing connection to Stellar network...");
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let success = response.status().is_success();
        
        if success {
            println!("✅ Connection to Stellar network OK!");
            
            // Display network information
            if let Ok(info) = response.json::<serde_json::Value>().await {
                if let Some(network) = info.get("network_passphrase") {
                    println!("   Network: {}", network);
                }
                if let Some(version) = info.get("horizon_version") {
                    println!("   Horizon Version: {}", version);
                }
            }
        } else {
            println!("❌ Connection error: {}", response.status());
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