*** Begin Patch
*** Update File: app/src-tauri/src/main.rs
@@
 #[tauri::command]
 fn import_file(state: tauri::State<'_, BackendState>, src: String, parent: Option<String>) -> Result<String, String> {
   let guard = state.0.lock().map_err(|e| e.to_string())?;
   let sess = guard.as_ref().ok_or_else(|| "vault is locked".to_string())?;
-  let id = ironvault_core::import_file(sess, &src, parent.as_deref()).map_err(|e| e.to_string())?;
-  Ok(id)
+  // Emit start event (0%)
+  if let Some(win) = tauri::Window::get(&tauri::AppHandle::current(), "main") {
+    let _ = win.emit("import-progress", serde_json::json!({"progress": 0}));
+  }
+  let id = ironvault_core::import_file(sess, &src, parent.as_deref()).map_err(|e| e.to_string())?;
+  // Emit finished event (100%)
+  if let Some(win) = tauri::Window::get(&tauri::AppHandle::current(), "main") {
+    let _ = win.emit("import-progress", serde_json::json!({"progress": 100}));
+  }
+  Ok(id)
 }
*** End Patch
