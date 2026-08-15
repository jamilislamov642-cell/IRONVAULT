*** Begin Patch
*** Update File: crates/ironvault-core/src/lib.rs
@@
 use crypto::{derive_master_key, KdfSettings};
 use storage;
 use std::time::{SystemTime, UNIX_EPOCH};
+use rusqlite;
+use uuid;
*** End Patch
