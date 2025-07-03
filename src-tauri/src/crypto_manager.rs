use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use base64::{engine::general_purpose, Engine as _};
use aes_gcm::aead::rand_core::RngCore;
use std::fs;
use std::path::PathBuf;

const MASTER_PASSWORD: &[u8] = b"your_super_secret_master_password_here"; // CHANGE THIS IN PRODUCTION
const API_KEY_FILE_NAME: &str = ".api_key.enc";

pub fn encrypt_and_save_api_key(app_data_dir: PathBuf, api_key: &str) -> Result<(), String> {
    let mut salt_bytes = [0u8; 16];
    OsRng.fill_bytes(&mut salt_bytes);
    let salt = SaltString::encode_b64(&salt_bytes).map_err(|e| format!("Failed to encode salt: {}", e))?;

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(MASTER_PASSWORD, &salt)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    let key_bytes = password_hash.hash.ok_or("No hash found".to_string())?.as_bytes()[..32].to_vec();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, api_key.as_bytes().as_ref())
        .map_err(|e| format!("Failed to encrypt: {}", e))?;

    let encrypted_data = EncryptedData {
        salt: salt.to_string(),
        nonce: general_purpose::STANDARD.encode(&nonce_bytes),
        ciphertext: general_purpose::STANDARD.encode(&ciphertext),
    };

    let file_path = app_data_dir.join(API_KEY_FILE_NAME);
    let json_data = serde_json::to_string(&encrypted_data)
        .map_err(|e| format!("Failed to serialize data: {}", e))?;

    fs::write(file_path, json_data)
        .map_err(|e| format!("Failed to write encrypted data to file: {}", e))?;

    Ok(())
}

pub fn decrypt_api_key(app_data_dir: PathBuf) -> Result<String, String> {
    let file_path = app_data_dir.join(API_KEY_FILE_NAME);
    let json_data = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read encrypted data from file: {}", e))?;

    let encrypted_data: EncryptedData = serde_json::from_str(&json_data)
        .map_err(|e| format!("Failed to deserialize data: {}", e))?;

    let salt = SaltString::from_b64(&encrypted_data.salt)
        .map_err(|e| format!("Failed to decode salt: {}", e))?;

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(MASTER_PASSWORD, &salt)
        .map_err(|e| format!("Failed to hash password for decryption: {}", e))?;

    let key_bytes = password_hash.hash.ok_or("No hash found".to_string())?.as_bytes()[..32].to_vec();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce_bytes = general_purpose::STANDARD.decode(&encrypted_data.nonce)
        .map_err(|e| format!("Failed to decode nonce: {}", e))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = general_purpose::STANDARD.decode(&encrypted_data.ciphertext)
        .map_err(|e| format!("Failed to decode ciphertext: {}", e))?;

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Failed to decrypt: {}", e))?;

    Ok(String::from_utf8(plaintext)
        .map_err(|e| format!("Failed to convert decrypted bytes to string: {}", e))?)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EncryptedData {
    salt: String,
    nonce: String,
    ciphertext: String,
}