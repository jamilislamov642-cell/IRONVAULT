//! Lightweight cryptography utilities used by the IRONVAULT project.
//!
//! This crate provides a minimal, well-tested implementation of the following:
//! - Argon2id-based master key derivation
//! - HKDF-based subkey derivation
//! - XChaCha20-Poly1305 AEAD encrypt/decrypt utilities

use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, PasswordHash, PasswordVerifier}};
use argon2::password_hash::rand_core::OsRng;
use argon2::Params as Argon2Params;
use chacha20poly1305::{XChaCha20Poly1305, aead::{Aead, KeyInit, OsRng as AeadOsRng, rand_core::RngCore}, XNonce, Key};
use hkdf::Hkdf;
use sha2::Sha256;
use rand::RngCore as _;
use zeroize::Zeroize;
use secrecy::{SecretVec, ExposeSecret};

pub const MASTER_KEY_LEN: usize = 32;

pub struct KdfSettings {
    pub mem_cost: u32,    // in KiB
    pub time_cost: u32,
    pub lanes: u32,
}

impl Default for KdfSettings {
    fn default() -> Self {
        Self { mem_cost: 65536 / 1024, time_cost: 3, lanes: 1 } // mem_cost used in KiB for argon2::Params convenience
    }
}

pub fn generate_salt() -> Vec<u8> {
    let mut s = vec![0u8; 16];
    OsRng.fill_bytes(&mut s);
    s
}

/// Derive a 32-byte master key from a password and salt using Argon2id.
pub fn derive_master_key(password: &str, salt: &[u8], params: &KdfSettings) -> [u8; MASTER_KEY_LEN] {
    // Note: argon2::Params constructor accepts mem_cost in KB
    let params_argon = Argon2Params::new(
        (params.mem_cost * 1024) as u32, // convert KiB to bytes for Params API where applicable
        params.time_cost,
        params.lanes,
        None
    ).expect("valid argon2 params");

    let argon = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params_argon);

    // We'll use HKDF below; for now produce raw bytes using password-hash API
    // Use password_hash::SaltString for formatting but we need raw bytes output
    let mut out = [0u8; MASTER_KEY_LEN];
    // The argon2 crate does not expose direct derive to raw bytes stable API in older versions.
    // We'll use the low-level hash_password_into method.
    argon.hash_password_into(password.as_bytes(), salt, &mut out).expect("argon2 derive");
    out
}

/// Derive named subkeys using HKDF-SHA256 from the master key.
pub fn hkdf_expand(master: &[u8], info: &[u8], out: &mut [u8]) {
    let hk = Hkdf::<Sha256>::new(None, master);
    hk.expand(info, out).expect("hkdf expand");
}

/// Encrypt plaintext with XChaCha20-Poly1305 and return nonce + ciphertext
pub fn encrypt(key: &[u8; MASTER_KEY_LEN], plaintext: &[u8], aad: Option<&[u8]>) -> Vec<u8> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let mut nonce_bytes = [0u8; 24];
    AeadOsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);
    let ct = match aad {
        Some(a) => cipher.encrypt(nonce, aead::Payload { msg: plaintext, aad: a }),
        None => cipher.encrypt(nonce, plaintext)
    };
    let ct = ct.expect("encryption failed");
    // return nonce || ciphertext
    let mut out = Vec::with_capacity(24 + ct.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct);
    out
}

/// Decrypt data produced by `encrypt` (expects nonce prefixed)
pub fn decrypt(key: &[u8; MASTER_KEY_LEN], data: &[u8], aad: Option<&[u8]>) -> Result<Vec<u8>, String> {
    if data.len() < 24 { return Err("ciphertext too short".into()); }
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = XNonce::from_slice(&data[..24]);
    let ct = &data[24..];
    let res = match aad {
        Some(a) => cipher.decrypt(nonce, aead::Payload { msg: ct, aad: a }),
        None => cipher.decrypt(nonce, ct)
    };
    res.map_err(|e| format!("decrypt error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kdf_and_encrypt_roundtrip() {
        let password = "correct horse battery staple";
        let salt = generate_salt();
        let params = KdfSettings::default();
        let master = derive_master_key(password, &salt, &params);
        let pt = b"hello ironvault";
        let ct = encrypt(&master, pt, None);
        let dt = decrypt(&master, &ct, None).expect("decrypt");
        assert_eq!(dt, pt);
        // zeroize master before dropping
        let mut master_owned = master;
        master_owned.zeroize();
    }
}
