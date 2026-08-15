*** Begin Patch
*** Update File: README.md
@@
 - README, MIT LICENSE, .gitignore, CI workflow (runs cargo test on Linux)
 +README, MIT LICENSE, .gitignore, CI workflow (runs cargo test on Linux)
@@
 3) Run the test suite:
    cargo test --workspace
+
+Notes on development progress:
+- Milestone 2 in progress: secure storage crate, streaming encryption, import/export, atomic writes, path traversal guards, expanded tests, and Tauri + Svelte skeleton under /app.
+- To run the CLI for vault creation with password prompts:
+  cargo run --package ironvault-cli -- create /path/to/vault
*** End Patch
