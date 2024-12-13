use sha2::{Sha512, Digest};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::fs::{OpenOptions, File, metadata};
use std::io::{BufRead, BufReader, Write};
use rusqlite::{Connection, Result};
use rusqlite::params;
use std::fs;

/// Helper function to hash a password with a salt
pub fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password);
    hasher.update(salt);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Generate a random salt
pub fn generate_salt() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}


pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("users.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            salt TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn db_exists() -> bool {
    fs::metadata("users.db").is_ok()
}

/// Store user in the database file
pub fn store_user(conn: &Connection, username: &str, password: &str) -> Result<()> {
    let salt = generate_salt();
    let hash = hash_password(password, &salt);

    conn.execute(
        "INSERT INTO users (username, password_hash, salt) VALUES (?1, ?2, ?3)",
        params![username, hash, salt],
    )?;
    Ok(())
}

/// Validate user credentials against the database
pub fn validate_user(conn: &Connection, username: &str, password: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT password_hash, salt FROM users WHERE username = ?1")?;
    let mut rows = stmt.query(params![username])?;

    if let Some(row) = rows.next()? {
        let db_hash: String = row.get(0)?;
        let db_salt: String = row.get(1)?;

        let hash = hash_password(password, &db_salt);
        Ok(hash == db_hash)
    } else {
        Ok(false)
    }
}