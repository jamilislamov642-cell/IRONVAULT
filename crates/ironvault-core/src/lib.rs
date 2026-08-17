*** Begin Patch
*** Update File: crates/ironvault-core/src/lib.rs
@@
 pub fn import_file(session: &VaultSession, src: &str, parent_id: Option<&str>) -> Result<String> {
@@
 }
+
+use std::sync::mpsc::Sender;
+use tempfile::NamedTempFile;
+use std::io::Read;
+use uuid::Uuid;
+use std::fs::File;
+
+/// Import a file with progress reporting. If `progress_tx` is Some(sender) the function will
+/// send percentage (0..=100) updates as u8 values during the import.
+pub fn import_file_with_progress(session: &VaultSession, src: &str, parent_id: Option<&str>, progress_tx: Option<Sender<u8>>) -> Result<String> {
+    let src_path = Path::new(src);
+    if !src_path.exists() { return Err(anyhow::anyhow!("source file not found")); }
+    let blobs_dir = session.vault_path.join("blobs");
+    std::fs::create_dir_all(&blobs_dir)?;
+
+    let mut src_f = File::open(src_path).with_context(|| format!("open source file: {}", src))?;
+    let metadata = src_f.metadata()?;
+    let total_size = metadata.len();
+
+    let file_id = Uuid::new_v4().to_string();
+    // derive per-file key
+    let mut key = [0u8; crypto::MASTER_KEY_LEN];
+    let info = format!("ironvault-file-key:{}", file_id);
+    crypto::hkdf_expand(&session.master_key, info.as_bytes(), &mut key);
+
+    // write to temp file
+    let mut tmp = NamedTempFile::new_in(&blobs_dir).context("create temp file in blobs")?;
+
+    const CHUNK_SIZE: usize = 1024 * 1024;
+    let mut buf = vec![0u8; CHUNK_SIZE];
+    let mut processed: u64 = 0;
+    loop {
+        let n = src_f.read(&mut buf)?;
+        if n == 0 { break; }
+        let pt = &buf[..n];
+        let ct = crypto::encrypt(&key, pt, Some(&file_id.as_bytes()));
+        let ct_len = ct.len() as u64;
+        tmp.write_all(&ct_len.to_be_bytes())?;
+        tmp.write_all(&ct)?;
+        processed += n as u64;
+        if let Some(ref tx) = progress_tx {
+            let pct = ((processed as f64 / total_size as f64) * 100.0).min(100.0).round() as u8;
+            let _ = tx.send(pct);
+        }
+    }
+
+    tmp.flush()?;
+    tmp.as_file_mut().sync_all()?;
+    let dest = blobs_dir.join(&file_id);
+    tmp.persist(&dest).with_context(|| format!("persisting blob to {}", dest.display()))?;
+
+    // write metadata to DB with encrypted name
+    let mut meta_key = [0u8; crypto::MASTER_KEY_LEN];
+    crypto::hkdf_expand(&session.master_key, b"ironvault-meta-key", &mut meta_key);
+    let fname = src_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
+    let name_cipher = crypto::encrypt(&meta_key, fname.as_bytes(), Some(file_id.as_bytes()));
+
+    let db_path = session.vault_path.join("metadata.sqlite");
+    let conn = rusqlite::Connection::open(db_path.to_str().unwrap())?;
+    let id = Uuid::new_v4().to_string();
+    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
+    conn.execute("INSERT INTO nodes (id, parent_id, name_cipher, metadata_cipher, blob_name, size, created_at, imported_at, modified_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
+        rusqlite::params![id, parent_id, name_cipher, Vec::<u8>::new(), file_id, total_size as i64, now, now, now])?;
+
+    // final progress
+    if let Some(ref tx) = progress_tx { let _ = tx.send(100); }
+
+    Ok(id)
+}
+
+/// Produce a safe text preview for a node by decrypting up to `limit` bytes from the blob.
+pub fn preview_node(session: &VaultSession, node_id: &str, limit: usize) -> Result<String> {
+    let db_path = session.vault_path.join("metadata.sqlite");
+    let conn = rusqlite::Connection::open(db_path.to_str().unwrap())?;
+    let mut stmt = conn.prepare("SELECT blob_name FROM nodes WHERE id = ?1")?;
+    let blob_name: String = stmt.query_row(rusqlite::params![node_id], |r| r.get(0))?;
+
+    let blob_path = session.vault_path.join("blobs").join(&blob_name);
+    let mut f = File::open(&blob_path)?;
+    // read first chunked ciphertext block(s) until we have decrypted limit bytes or reach EOF
+    let key_info = format!("ironvault-file-key:{}", blob_name);
+    let mut key = [0u8; crypto::MASTER_KEY_LEN];
+    crypto::hkdf_expand(&session.master_key, key_info.as_bytes(), &mut key);
+
+    let mut out = Vec::new();
+    loop {
+        let mut lenb = [0u8;8];
+        if let Err(e) = f.read_exact(&mut lenb) {
+            if e.kind() == std::io::ErrorKind::UnexpectedEof { break; } else { return Err(e.into()); }
+        }
+        let ct_len = u64::from_be_bytes(lenb) as usize;
+        let mut nonce_and_ct = vec![0u8; ct_len];
+        f.read_exact(&mut nonce_and_ct)?;
+        let pt = crypto::decrypt(&key, &nonce_and_ct, Some(blob_name.as_bytes())).map_err(|e| anyhow::anyhow!(e))?;
+        out.extend_from_slice(&pt);
+        if out.len() >= limit { out.truncate(limit); break; }
+    }
+
+    // Attempt to interpret as UTF-8 text, else return hex summary
+    if let Ok(s) = String::from_utf8(out.clone()) {
+        Ok(s)
+    } else {
+        // return hex of first bytes
+        Ok(hex::encode(&out[..std::cmp::min(out.len(), 64)]))
+    }
+}
*** End Patch
