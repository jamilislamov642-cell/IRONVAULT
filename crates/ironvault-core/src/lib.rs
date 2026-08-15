*** Begin Patch
*** Update File: crates/ironvault-core/src/lib.rs
@@
 pub fn export_file(session: &VaultSession, node_id: &str, dest_path: &str) -> Result<()> {
@@
 }
+
+/// List nodes in the vault and return decrypted names. Returns Vec<(id, parent_id, name)>
+pub fn list_nodes(session: &VaultSession) -> Result<Vec<(String, Option<String>, String)>> {
+    let db_path = session.vault_path.join("metadata.sqlite");
+    let conn = rusqlite::Connection::open(db_path.to_str().unwrap())?;
+    let mut stmt = conn.prepare("SELECT id, parent_id, name_cipher, blob_name FROM nodes")?;
+    let mut rows = stmt.query([])?;
+    let mut out = Vec::new();
+    while let Some(row) = rows.next()? {
+        let id: String = row.get(0)?;
+        let parent: Option<String> = row.get(1)?;
+        let name_cipher: Vec<u8> = row.get(2)?;
+        let blob_name: String = row.get(3)?;
+        let mut meta_key = [0u8; crypto::MASTER_KEY_LEN];
+        crypto::hkdf_expand(&session.master_key, b"ironvault-meta-key", &mut meta_key);
+        let name = crypto::decrypt(&meta_key, &name_cipher, Some(blob_name.as_bytes())).unwrap_or_else(|_| b"<decryption error>".to_vec());
+        let name_str = String::from_utf8_lossy(&name).to_string();
+        out.push((id, parent, name_str));
+    }
+    Ok(out)
+}
*** End Patch
