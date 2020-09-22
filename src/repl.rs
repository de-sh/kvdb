use std::fmt;
use std::io;
use std::io::Write;

use crate::parser::{MetaCmdResult, Statement};

pub struct REPL {
    cmd: String,
}

impl REPL {
    pub fn new() -> Self {
        REPL { cmd: "".to_owned() }
    }

    pub fn repl(&mut self) {
        loop {
            // Prompt
            print!("db > ");
            // Input
            self.read_input();
            // Parse Meta Commands or Prepare SQL
            if self.cmd.starts_with(".") {
                match MetaCmdResult::run(&self.cmd) {
                    MetaCmdResult::Unrecognized => println!("db: command not found: {}", self.cmd),
                    MetaCmdResult::Success => continue,
                }
            } else {
                let statement = Statement::prep(&self.cmd);
                if statement.is_unrec() {
                    println!("db: command not found: {}", self.cmd);
                } else {
                    continue // 
                }
            }
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
}

impl fmt::Display for REPL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.cmd)
    }
}
