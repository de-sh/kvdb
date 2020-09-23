use crate::parser::{Statement, StatementType};
use std::collections::HashMap;

pub enum ExecResult {
    Success,
    Failed,
}

pub struct Store {
    storage: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn execute(&mut self, st: Statement) -> ExecResult {
        let key = st.key.unwrap();
        match st.stype {
            StatementType::Set => self.set(key, st.value.unwrap()),
            StatementType::Get => self.get(key),
            _ => ExecResult::Failed,
        }
    }

    fn set(&mut self, key: String, value: String) -> ExecResult {
        self.storage.insert(key, value);
        ExecResult::Success
    }

    fn get(&self, key: String) -> ExecResult {
        match self.storage.get(&key) {
            Some(s) => {
                println!("{}", s);
                ExecResult::Success
            },
            None => {
                eprintln!("Error: no value associated with key {}.", key);
                ExecResult::Failed
            },
        }
    }
}
