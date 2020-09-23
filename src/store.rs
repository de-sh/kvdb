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
        match st.stype {
            StatementType::Set => self.set(st.key.unwrap(), st.value.unwrap()),
            StatementType::Get => match self.get(&st.key.unwrap()) {
                Some(s) => {
                    println!("{}", s);
                    ExecResult::Success
                }
                None => ExecResult::Failed,
            },
            _ => ExecResult::Failed,
        }
    }

    fn set(&mut self, key: String, value: String) -> ExecResult {
        self.storage.insert(key, value);
        ExecResult::Success
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }
}
