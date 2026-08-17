*** Begin Patch
*** Update File: app/src-tauri/src/main.rs
@@
 #[tauri::command]
 fn import_file(state: tauri::State<'_, BackendState>, src: String, parent: Option<String>) -> Result<String, String> {
-  // Emit start event (0%)
-  if let Some(win) = tauri::Window::get(&tauri::AppHandle::current(), "main") {
-    let _ = win.emit("import-progress", serde_json::json!({"progress": 0}));
-  }
-  let id = ironvault_core::import_file(sess, &src, parent.as_deref()).map_err(|e| e.to_string())?;
-  // Emit finished event (100%)
-  if let Some(win) = tauri::Window::get(&tauri::AppHandle::current(), "main") {
-    let _ = win.emit("import-progress", serde_json::json!({"progress": 100}));
-  }
-  Ok(id)
+  // Spawn import on a new thread and stream progress via mpsc
+  let app_handle = tauri::AppHandle::current();
+  let (tx, rx) = std::sync::mpsc::channel();
+  let sess_clone = sess.clone();
+  let parent_clone = parent.clone();
+  std::thread::spawn(move || {
+    let _ = ironvault_core::import_file_with_progress(&sess_clone, &src, parent_clone.as_deref(), Some(tx));
+  });
+  // forward progress events to the frontend
+  std::thread::spawn(move || {
+    for p in rx.iter() {
+      if let Some(win) = tauri::Window::get(&app_handle, "main") {
+        let _ = win.emit("import-progress", serde_json::json!({"progress": p}));
+      }
+    }
+  });
+  // Note: import runs async; return a placeholder (caller should refresh list when import completes)
+  Ok("import_started".to_string())
 }
+
+#[tauri::command]
+fn preview_node(state: tauri::State<'_, BackendState>, node_id: String) -> Result<serde_json::Value, String> {
+  let guard = state.0.lock().map_err(|e| e.to_string())?;
+  let sess = guard.as_ref().ok_or_else(|| "vault is locked".to_string())?;
+  let preview = ironvault_core::preview_node(sess, &node_id, 128 * 1024).map_err(|e| e.to_string())?;
+  Ok(serde_json::json!({"preview": preview}))
+}
*** End Patch
