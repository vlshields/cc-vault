use std::fs;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::Argon2;
use rand::RngCore;
use zeroize::Zeroize;

use crate::card::Card;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

pub(crate) fn vault_path() -> PathBuf {
    let mut path = dirs_path();
    path.push("vault.enc");
    path
}

fn dirs_path() -> PathBuf {
    let mut path = PathBuf::from(std::env::var("HOME").expect("HOME not set"));
    path.push(".ccvault");
    path
}

fn derive_key(password: &[u8], salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(password, salt, &mut key)
        .expect("Argon2 key derivation failed");
    key
}

fn encrypt(data: &[u8], password: &str) -> Vec<u8> {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let mut key = derive_key(password.as_bytes(), &salt);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data).expect("Encryption failed");
    key.zeroize();

    let mut output = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    output
}

fn decrypt(blob: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if blob.len() < SALT_LEN + NONCE_LEN + 16 {
        return Err("Vault file too short / corrupted".into());
    }

    let salt = &blob[..SALT_LEN];
    let nonce_bytes = &blob[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &blob[SALT_LEN + NONCE_LEN..];

    let mut key = derive_key(password.as_bytes(), salt);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed — wrong password or corrupted vault".to_string());
    key.zeroize();
    plaintext
}

pub fn load_cards(password: &str) -> Result<Vec<Card>, String> {
    let path = vault_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let blob = fs::read(&path).map_err(|e| format!("Failed to read vault: {e}"))?;
    let plaintext = decrypt(&blob, password)?;
    let cards: Vec<Card> =
        serde_json::from_slice(&plaintext).map_err(|e| format!("Corrupt vault data: {e}"))?;
    Ok(cards)
}

pub fn load_cards_protected(password: &str) -> Result<Vec<Card>, String> {
    use crate::lockout;

    lockout::check_destroyed()?;

    // If no vault file exists, no authentication needed
    if !vault_path().exists() {
        return Ok(Vec::new());
    }

    match load_cards(password) {
        Ok(cards) => {
            lockout::record_success();
            Ok(cards)
        }
        Err(e) => {
            if e.contains("Decryption failed") {
                lockout::record_failure();
            }
            Err(e)
        }
    }
}

pub fn save_cards(cards: &[Card], password: &str) -> Result<(), String> {
    let dir = dirs_path();
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create ~/.ccvault: {e}"))?;

    let json = serde_json::to_vec(cards).unwrap();
    let encrypted = encrypt(&json, password);

    let path = vault_path();
    // Write with 0600 permissions
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&path)
        .and_then(|f| {
            use std::io::Write;
            let mut f = f;
            f.write_all(&encrypted)
        })
        .map_err(|e| format!("Failed to write vault: {e}"))?;

    Ok(())
}

pub fn ask_password(prompt: &str) -> String {
    rpassword::prompt_password(prompt).expect("Failed to read password")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;

    fn sample_card() -> Card {
        Card {
            label: "Test Visa".into(),
            number: "4111111111111111".into(),
            exp: "12/25".into(),
            cvv: "123".into(),
            name: "John Doe".into(),
            zip: "90210".into(),
        }
    }

    #[test]
    fn derive_key_deterministic() {
        let password = b"hunter2";
        let salt = [0u8; SALT_LEN];
        let k1 = derive_key(password, &salt);
        let k2 = derive_key(password, &salt);
        assert_eq!(k1, k2);
    }

    #[test]
    fn derive_key_different_salts_differ() {
        let password = b"hunter2";
        let salt1 = [0u8; SALT_LEN];
        let salt2 = [1u8; SALT_LEN];
        let k1 = derive_key(password, &salt1);
        let k2 = derive_key(password, &salt2);
        assert_ne!(k1, k2);
    }

    #[test]
    fn derive_key_different_passwords_differ() {
        let salt = [0u8; SALT_LEN];
        let k1 = derive_key(b"password1", &salt);
        let k2 = derive_key(b"password2", &salt);
        assert_ne!(k1, k2);
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let plaintext = b"hello, vault!";
        let password = "my-secret";
        let blob = encrypt(plaintext, password);
        let result = decrypt(&blob, password).unwrap();
        assert_eq!(result, plaintext);
    }

    #[test]
    fn encrypt_produces_different_ciphertext_each_time() {
        let plaintext = b"same data";
        let password = "same-password";
        let blob1 = encrypt(plaintext, password);
        let blob2 = encrypt(plaintext, password);
        // Random salt + nonce should make each encryption unique
        assert_ne!(blob1, blob2);
    }

    #[test]
    fn decrypt_wrong_password_fails() {
        let plaintext = b"secret stuff";
        let blob = encrypt(plaintext, "correct-password");
        let result = decrypt(&blob, "wrong-password");
        assert!(result.is_err());
    }

    #[test]
    fn decrypt_truncated_blob_fails() {
        let result = decrypt(&[0u8; 10], "password");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short"));
    }

    #[test]
    fn decrypt_corrupted_blob_fails() {
        let plaintext = b"secret stuff";
        let mut blob = encrypt(plaintext, "password");
        // Flip a byte in the ciphertext portion
        let last = blob.len() - 1;
        blob[last] ^= 0xFF;
        let result = decrypt(&blob, "password");
        assert!(result.is_err());
    }

    #[test]
    fn encrypt_decrypt_card_json_roundtrip() {
        let cards = vec![sample_card()];
        let json = serde_json::to_vec(&cards).unwrap();
        let password = "test-master-pw";

        let blob = encrypt(&json, password);
        let decrypted = decrypt(&blob, password).unwrap();
        let restored: Vec<Card> = serde_json::from_slice(&decrypted).unwrap();

        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].label, "Test Visa");
        assert_eq!(restored[0].number, "4111111111111111");
    }

    #[test]
    fn encrypt_decrypt_empty_data() {
        let blob = encrypt(b"", "pw");
        let result = decrypt(&blob, "pw").unwrap();
        assert!(result.is_empty());
    }
}
