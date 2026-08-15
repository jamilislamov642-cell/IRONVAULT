---
*** Begin Patch
*** Update File: crates/crypto/src/lib.rs
@@
-use chacha20poly1305::{XChaCha20Poly1305, aead::{Aead, KeyInit, OsRng as AeadOsRng, rand_core::RngCore}, XNonce, Key};
+use chacha20poly1305::{XChaCha20Poly1305, aead::{Aead, KeyInit, Payload, OsRng as AeadOsRng, rand_core::RngCore}, XNonce, Key};
@@
 pub fn hkdf_expand(master: &[u8], info: &[u8], out: &mut [u8]) {
     let hk = Hkdf::<Sha256>::new(None, master);
     hk.expand(info, out).expect("hkdf expand");
 }
@@
 pub fn encrypt_chunk(key: &[u8; MASTER_KEY_LEN], chunk: &[u8], aad: &[u8]) -> Vec<u8> {
     encrypt(key, chunk, Some(aad))
 }
@@
 pub fn decrypt_chunk(key: &[u8; MASTER_KEY_LEN], data: &[u8], aad: &[u8]) -> Result<Vec<u8>, String> {
     decrypt(key, data, Some(aad))
 }
+
+// Expose a convenience function to securely zeroize a key array
+pub fn zeroize_key(k: &mut [u8; MASTER_KEY_LEN]) {
+    k.zeroize();
+}
*** End Patch
