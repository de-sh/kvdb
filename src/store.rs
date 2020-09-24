/// Code relating to the heart of a database, the storage engine.

use std::collections::HashMap;

/// Depicts whether an operation was successfully executed or not.
pub enum ExecResult {
    Success,
    Failed,
}

/// The Storage Engine
pub struct Store {
    /// A KV store in the form of in-memory HashMap.
    storage: HashMap<String, String>,
}

impl Store {
    /// Creates a new Storage Engine.
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Operates HashMap::insert()
    pub fn set(&mut self, key: String, value: String) -> ExecResult {
        // Fails if key already points to another value, else stores key-value pair and returns success.
        match self.storage.contains_key(&key) {
            true => {
                eprintln!(
                    "Error: `{}` already associated with value `{}`.",
                    self.storage.get(&key).unwrap(),
                    key
                );
                ExecResult::Failed
            }
            _ => {
                self.storage.insert(key, value);
                ExecResult::Success
            }
        }
    }

    /// Operates HashMap::get() and fails if key-value pair doesn't
    /// exist, else returns (string, success).
    pub fn get(&self, key: String) -> Result<String, ExecResult> {
        match self.storage.get(&key) {
            None => Err(ExecResult::Failed),
            Some(s) => Ok(s.to_string())
        }
    }

    /// Operates HashMap::remove() and fails if the key-value pair
    /// doesn't exist, else removes it and returns success.
    pub fn rem(&mut self, key: String) -> ExecResult {
        match self.storage.remove(&key) {
            Some(val) => {
                println!("Removed: {} -> {}", key, val);
                ExecResult::Success
            }
            None => {
                eprintln!(
                    "Error: Can't remove, as no value associated with key `{}`.",
                    key
                );
                ExecResult::Failed
            }
        }
    }
}
