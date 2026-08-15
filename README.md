# IRONVAULT

IRONVAULT is a local-first encrypted file vault. This repository contains the initial Rust backend scaffold and project layout for the IRONVAULT project. The full desktop application will use a Tauri frontend (Svelte) and a Rust backend for cryptography, storage, and vault management.

This initial commit provides:

- A Rust workspace with core crates:
  - crates/crypto — cryptographic utilities (Argon2id + XChaCha20-Poly1305 + HKDF)
  - crates/db — small SQLite schema initializer for vault metadata
  - crates/ironvault-core — high-level vault operations (scaffold)
  - crates/cli — minimal CLI to create a vault (useful for testing without the UI)
- CI workflow that runs cargo test on Linux
- MIT LICENSE
- Project README and basic UI/tauri notes

Next steps (planned):

- Implement full Tauri + Svelte frontend in /app
- Implement streaming chunked encryption for large files
- Implement secure in-memory key handling and auto-lock
- Implement encrypted SQLite fields and search index
- Add tests and packaging for macOS/Windows/Linux

This is the starting point. See crates/* for code and tests.
