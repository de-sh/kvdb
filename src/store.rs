use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;


/// Depicts whether an operation was successfully executed or not.
pub enum ExecResult {
    Success,
    Failed,
}

/// The Storage Engine
pub struct Store<A,B> {
    /// A KV store in the form of in-memory HashMap.
    /// Types A and B can be defined by the use case.
    storage: HashMap<A, B>,
}

/// As is clear from the implementation, types A and B must implement Display 
/// to be 'printable'. While A must also implement Hash and Eq traits
impl<A: Hash + Eq + Display, B: Display> Store<A,B> {
    /// Creates a new Storage Engine.
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Operates HashMap::insert()
    pub fn set(&mut self, key: A, value: B) -> ExecResult {
        // Fails if key already points to another value, else stores key-value pair and returns success.
        if self.storage.contains_key(&key) {
            eprintln!(
                "Error: `{}` already associated with value `{}`.",
                self.storage.get(&key).unwrap(),
                key
            );
            ExecResult::Failed
        } else {
            self.storage.insert(key, value);
            ExecResult::Success
        }
    }

    /// Operates HashMap::get() and fails if key-value pair doesn't
    /// exist, else returns (value, success).
    pub fn get(&self, key: A) -> Result<B, ExecResult> {
        match self.storage.get(&key) {
            None => Err(ExecResult::Failed),
            Some(s) => Ok(B::from(s)),
        }
    }

    /// Operates HashMap::remove() and fails if the key-value pair
    /// doesn't exist, else removes it and returns success.
    pub fn rem(&mut self, key: A) -> ExecResult {
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
