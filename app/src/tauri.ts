import { invoke } from '@tauri-apps/api/tauri'

export async function createVault(path, password) {
  return await invoke('create_vault', { path, password })
}

export async function unlockVault(path, password) {
  return await invoke('unlock_vault', { path, password })
}

export async function lockVault() {
  return await invoke('lock_vault')
}

export async function listNodes() {
  return await invoke('list_nodes')
}

export async function importFile(src, parent) {
  return await invoke('import_file', { src, parent })
}

export async function exportFile(nodeId, dest) {
  return await invoke('export_file', { nodeId, dest })
}
