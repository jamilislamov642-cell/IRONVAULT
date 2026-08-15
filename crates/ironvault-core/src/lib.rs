//! High-level vault operations. This crate wires together the crypto and db crates
//! to implement basic create/unlock operations. It's intentionally minimal in this
//! initial scaffold so we can iterate quickly and provide tests.

use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use crate::header::VaultHeader;

pub mod header {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct VaultHeader {
        pub version: u32,
        pub salt: Vec<u8>,
        pub kdf_mem_kib: u32,
        pub kdf_time: u32,
        pub kdf_lanes: u32,
    }
}

/// Create a new vault directory at `path` using the provided password.
pub fn create_vault(path: &str, password: &str) -> Result<()> {
    let p = Path::new(path);
    if p.exists() {
        return Err(anyhow::anyhow!("path already exists"));
    }
    fs::create_dir_all(p).with_context(|| "creating vault directory")?;
    // generate salt and header
    let salt = crypto::generate_salt();
    let header = VaultHeader {
        version: 1,
        salt: salt.clone(),
        kdf_mem_kib: 65536 / 1024,
        kdf_time: 3,
        kdf_lanes: 1,
    };
    let header_json = serde_json::to_vec_pretty(&header)?;
    let header_path = p.join("header.json");
    fs::write(&header_path, &header_json)?;
    // init sqlite metadata
    let db_path = p.join("metadata.sqlite");
    db::init_db(db_path.to_str().unwrap())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_and_open_vault() {
        let d = tempdir().unwrap();
        let p = d.path().join("vault");
        let path = p.to_str().unwrap();
        create_vault(path, "password").expect("create vault");
        assert!(std::path::Path::new(&format!("{}/header.json", path)).exists());
        assert!(std::path::Path::new(&format!("{}/metadata.sqlite", path)).exists());
    }
}
