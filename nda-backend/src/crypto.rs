// src/crypto.rs
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

#[derive(Debug)]
pub struct CryptoError(String);

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Crypto error: {}", self.0)
    }
}

impl std::error::Error for CryptoError {}

pub fn generate_key() -> String {
    let key = Aes256Gcm::generate_key(OsRng);
    general_purpose::STANDARD.encode(key)
}

pub fn encrypt_content(content: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key_bytes = general_purpose::STANDARD.decode(key)
        .map_err(|e| CryptoError(format!("Erro ao decodificar chave: {}", e)))?;
    
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, content.as_bytes())
        .map_err(|e| CryptoError(format!("Erro na criptografia: {:?}", e)))?;
    
    // Combinar nonce + ciphertext
    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);
    
    Ok(general_purpose::STANDARD.encode(encrypted_data))
}

pub fn decrypt_content(encrypted_content: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key_bytes = general_purpose::STANDARD.decode(key)
        .map_err(|e| CryptoError(format!("Erro ao decodificar chave: {}", e)))?;
    
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let encrypted_data = general_purpose::STANDARD.decode(encrypted_content)
        .map_err(|e| CryptoError(format!("Erro ao decodificar dados: {}", e)))?;
    
    if encrypted_data.len() < 12 {
        return Err(Box::new(CryptoError("Dados criptografados inválidos".to_string())));
    }

    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError(format!("Erro na descriptografia: {:?}", e)))?;
    
    String::from_utf8(plaintext)
        .map_err(|e| Box::new(CryptoError(format!("Erro UTF-8: {}", e))) as Box<dyn std::error::Error>)
}