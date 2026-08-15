use rusqlite::{params, Connection, Result};

/// Initialize the metadata database for a vault. This creates the tables used to store
/// encrypted metadata pointers and activity logs.
pub fn init_db(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;
    conn.execute_batch("BEGIN;

CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    parent_id TEXT,
    name_cipher BLOB,
    metadata_cipher BLOB,
    blob_name TEXT,
    size INTEGER,
    created_at INTEGER,
    imported_at INTEGER,
    modified_at INTEGER
);

CREATE TABLE IF NOT EXISTS activity_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts INTEGER,
    event_type TEXT,
    data_cipher BLOB
);

COMMIT;")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_db() {
        let d = tempdir().unwrap();
        let p = d.path().join("meta.sqlite");
        let path = p.to_str().unwrap();
        init_db(path).expect("init");
    }
}
