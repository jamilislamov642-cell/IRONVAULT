# UI / Tauri notes

This directory will contain the Tauri + Svelte frontend. For now this repository contains the Rust backend scaffold and a minimal CLI. The full desktop application will be implemented with Tauri (Rust backend) + Svelte (frontend) and will live under /app.

Planned UI structure:
- app/
  - src/
  - tauri.conf.json
  - package.json
  - svelte app skeleton

Next steps for UI:
- Initialize Tauri app with `cargo tauri init` and Svelte template
- Wire IPC endpoints to the Rust backend crates (vault operations)
- Implement dark, compact file explorer, preview pane, command palette, and settings
