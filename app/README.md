*** Begin Patch
*** Update File: app/README.md
@@
 - Initialize a Svelte + Tauri project in this directory and wire IPC to the Rust backend crates
+This placeholder contains a minimal Svelte app and Tauri config file. To finish the UI wiring locally:
+
+1) Install Node.js and a package manager (pnpm or npm)
+2) From app/: initialize dependencies (example using npm):
+   npm init -y
+   npm install svelte
+   npm install -D @tauri-apps/cli
+3) Follow Tauri docs to initialize the tauri project and connect the Rust crates as the backend. The IPC endpoints will call functions in crates/ironvault-core via the Tauri command API.
*** End Patch
