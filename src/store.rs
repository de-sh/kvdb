use crate::parser::{Statement, StatementType};
use std::collections::HashMap;

pub enum ExecResult {
    Success,
    Failed,
}

/// The Storage Engine
pub struct Store {
    storage: HashMap<String, String>, // A KV store in the form of in-memory HashMap.
}

impl Store {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// The Storage Engine's only method to execute statements.
    pub fn execute(&mut self, st: Statement) {
        // If type of statement is legit, execute, else fail.
        match match st.stype {
            StatementType::Set => self.set(st.key.unwrap(), st.value.unwrap()),
            StatementType::Get => self.get(st.key.unwrap()),
            StatementType::Rem => self.rem(st.key.unwrap()),
            _ => ExecResult::Failed,
        } {
            ExecResult::Failed => eprintln!("Command Execution Failed."),
            ExecResult::Success => println!("Success: OK"),
        }
    }

    /// Operates HashMap::insert()
    fn set(&mut self, key: String, value: String) -> ExecResult {
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

    /// Operates HashMap::get()
    fn get(&self, key: String) -> ExecResult {
        // Fails if key-value pair doesn't exist, else prints value and returns success.
        match self.storage.get(&key) {
            Some(s) => {
                println!("{}", s);
                ExecResult::Success
            }
            None => {
                eprintln!("Error: No value associated with key `{}`.", key);
                ExecResult::Failed
            }
        }
    }

    /// Operates HashMap::remove()
    fn rem(&mut self, key: String) -> ExecResult {
        // Fails if key-value pair doesn't exist, else removes it and returns success.
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
