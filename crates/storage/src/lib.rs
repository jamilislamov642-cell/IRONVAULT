*** Begin Patch
*** Update File: crates/storage/src/lib.rs
@@
 pub fn retrieve_file(vault_path: &Path, master_key: &[u8; MASTER_KEY_LEN], file_id: &str, dest_path: &Path) -> Result<()> {
     let blobs = ensure_blobs_dir(vault_path)?;
     let blob_path = blobs.join(file_id);
     let mut f = File::open(&blob_path).with_context(|| format!("open blob {}", blob_path.display()))?;
     let key = derive_file_key(master_key, file_id);
@@
-    let dest_dir = dest_path.parent().ok_or_else(|| anyhow::anyhow!("destination has no parent"))?;
-    fs::create_dir_all(dest_dir)?;
-    let mut tmp = NamedTempFile::new_in(dest_dir)?;
+    let dest_dir = dest_path.parent().ok_or_else(|| anyhow::anyhow!("destination has no parent"))?;
+    fs::create_dir_all(dest_dir)?;
+    // Ensure destination stays within the intended directory (prevent path traversal via filename)
+    let dest_dir_canon = fs::canonicalize(dest_dir).with_context(|| format!("canonicalize dest dir {}", dest_dir.display()))?;
+    let mut tmp = NamedTempFile::new_in(&dest_dir)?;
@@
-    tmp.flush()?;
-    tmp.as_file_mut().sync_all()?;
-    let dest_final = dest_path;
-    tmp.persist(dest_final).with_context(|| format!("persisting to {}", dest_final.display()))?;
+    tmp.flush()?;
+    tmp.as_file_mut().sync_all()?;
+    // finalize: ensure final path is within dest_dir
+    let dest_final = dest_path;
+    let dest_parent_canon = fs::canonicalize(dest_final.parent().unwrap()).with_context(|| format!("canonicalize parent {}", dest_final.display()))?;
+    if !dest_parent_canon.starts_with(&dest_dir_canon) {
+        return Err(anyhow::anyhow!("destination outside allowed directory"));
+    }
+    tmp.persist(dest_final).with_context(|| format!("persisting to {}", dest_final.display()))?;
     Ok(())
 }
@@
     fn store_and_retrieve_roundtrip() {
@@
         retrieve_file(&vault, &master, &file_id, &dest).expect("retrieve");
@@
         assert_eq!(h1.finalize().as_slice(), h2.finalize().as_slice());
     }
+
+    #[test]
+    fn detect_corrupted_blob() {
+        let d = tempdir().unwrap();
+        let vault = d.path().join("vault");
+        std::fs::create_dir_all(&vault).unwrap();
+        let src = d.path().join("hello2.txt");
+        let mut f = File::create(&src).unwrap();
+        write!(f, "corrupt test").unwrap();
+        f.sync_all().unwrap();
+
+        let password = "test-password";
+        let salt = crypto::generate_salt();
+        let params = crypto::KdfSettings::default();
+        let master = derive_master_key(password, &salt, &params);
+        let (file_id, _size) = store_file(&vault, &master, &src).expect("store");
+        // flip a byte in the blob
+        let blob_path = vault.join("blobs").join(&file_id);
+        let mut data = std::fs::read(&blob_path).expect("read blob");
+        if !data.is_empty() { data[40usize % data.len()] ^= 0xFF; }
+        std::fs::write(&blob_path, &data).expect("write blob");
+
+        let dest = d.path().join("out2.txt");
+        let res = retrieve_file(&vault, &master, &file_id, &dest);
+        assert!(res.is_err(), "expected decryption error for corrupted blob");
+    }
+
+    #[test]
+    fn wrong_master_key_fails() {
+        let d = tempdir().unwrap();
+        let vault = d.path().join("vault");
+        std::fs::create_dir_all(&vault).unwrap();
        let src = d.path().join("hello3.txt");
+        let mut f = File::create(&src).unwrap();
+        write!(f, "secret data").unwrap();
+        f.sync_all().unwrap();
+
+        let password = "correct-password";
+        let salt = crypto::generate_salt();
+        let params = crypto::KdfSettings::default();
+        let master = derive_master_key(password, &salt, &params);
+        let (file_id, _size) = store_file(&vault, &master, &src).expect("store");
+
+        // derive wrong master
+        let wrong = derive_master_key("wrong-password", &salt, &params);
+        let dest = d.path().join("out3.txt");
+        let res = retrieve_file(&vault, &wrong, &file_id, &dest);
+        assert!(res.is_err(), "expected decryption error with wrong master key");
+    }
*** End Patch
