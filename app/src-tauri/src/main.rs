use std::sync::Mutex;
use tauri::Manager;
use serde::{Serialize};

// Re-export the VaultSession type for use here
use ironvault_core::VaultSession;

struct BackendState(Mutex<Option<VaultSession>>);

#[derive(Serialize)]
struct NodeInfo {
  id: String,
  parent: Option<String>,
  name: String,
}

#[tauri::command]
fn create_vault(path: String, password: String) -> Result<(), String> {
  ironvault_core::create_vault(&path, &password).map_err(|e| e.to_string())
}

#[tauri::command]
fn unlock_vault(state: tauri::State<'_, BackendState>, path: String, password: String) -> Result<(), String> {
  let session = ironvault_core::unlock_vault(&path, &password).map_err(|e| e.to_string())?;
  let mut guard = state.0.lock().map_err(|e| e.to_string())?;
  *guard = Some(session);
  Ok(())
}

#[tauri::command]
fn lock_vault(state: tauri::State<'_, BackendState>) -> Result<(), String> {
  let mut guard = state.0.lock().map_err(|e| e.to_string())?;
  if let Some(sess) = guard.take() {
    sess.lock(); // zeroize master key
  }
  Ok(())
}

#[tauri::command]
fn list_nodes(state: tauri::State<'_, BackendState>) -> Result<Vec<NodeInfo>, String> {
  let guard = state.0.lock().map_err(|e| e.to_string())?;
  let sess = guard.as_ref().ok_or_else(|| "vault is locked".to_string())?;
  let nodes = ironvault_core::list_nodes(sess).map_err(|e| e.to_string())?;
  let out = nodes.into_iter().map(|(id, parent, name)| NodeInfo { id, parent, name }).collect();
  Ok(out)
}

#[tauri::command]
fn import_file(state: tauri::State<'_, BackendState>, src: String, parent: Option<String>) -> Result<String, String> {
  let guard = state.0.lock().map_err(|e| e.to_string())?;
  let sess = guard.as_ref().ok_or_else(|| "vault is locked".to_string())?;
  let id = ironvault_core::import_file(sess, &src, parent.as_deref()).map_err(|e| e.to_string())?;
  Ok(id)
}

#[tauri::command]
fn export_file(state: tauri::State<'_, BackendState>, node_id: String, dest: String) -> Result<(), String> {
  let guard = state.0.lock().map_err(|e| e.to_string())?;
  let sess = guard.as_ref().ok_or_else(|| "vault is locked".to_string())?;
  ironvault_core::export_file(sess, &node_id, &dest).map_err(|e| e.to_string())?;
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .manage(BackendState(Mutex::new(None)))
    .invoke_handler(tauri::generate_handler![create_vault, unlock_vault, lock_vault, list_nodes, import_file, export_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
