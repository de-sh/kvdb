use crate::parser::{Row, Statement, StatementType};

pub enum ExecResult {
    Success,
    Failed,
}

pub struct Store {
    storage: Vec<Row>,
}

impl Store {
    pub fn new() -> Self {
        Self { storage: vec![] }
    }

    pub fn execute(&mut self, st: Statement) -> ExecResult {
        match st.stype {
            StatementType::Insert => self.insert(st.row_to_insert.unwrap()),
            StatementType::Select => ExecResult::Success,
            _ => ExecResult::Failed,
        }
    }

    fn insert(&mut self, row: Row) -> ExecResult {
        self.storage.push(row);
        ExecResult::Success
    }
}
