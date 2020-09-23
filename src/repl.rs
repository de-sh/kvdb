use std::fmt;
use std::io;
use std::io::Write;

use crate::parser::{MetaCmdResult, Statement, StatementType};
use crate::store::Store;

pub struct REPL {
    cmd: String,
    store: Store,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            cmd: "".to_owned(),
            store: Store::new(),
        }
    }

    pub fn repl(&mut self) {
        loop {
            // Prompt
            print!("db > ");
            // Read
            self.read_input();
            // Evaluate
            self.parse_input();
        }
    }

    fn read_input(&mut self) {
        let mut cmd = "".to_owned();
        io::stdout().flush().expect("Error");
        io::stdin()
            .read_line(&mut cmd)
            .expect("Error in reading command, exiting REPL.");
        self.cmd = cmd.trim().to_string();
    }

    fn parse_input(&mut self) {
        // Parse Meta Commands or Prepare SQL
        if self.cmd.starts_with(".") {
            match MetaCmdResult::run(&self.cmd) {
                MetaCmdResult::Unrecognized => println!("db: command not found: {}", self.cmd),
                MetaCmdResult::Success => {}
            }
        } else {
            let st = Statement::prep(&self.cmd);
            match st.stype {
                StatementType::Unrecognized => println!("db: command not found: {}", self.cmd),
                _ => {
                    self.store.execute(st);
                }
            }
        }
    }
}

impl fmt::Display for REPL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.cmd)
    }
}
