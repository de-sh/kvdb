use std::fmt;
use std::io;
use std::io::Write;

use crate::parser;

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
                parser::MetaCmdResult::new(self.cmd);
            } else {
                parser::PrepResult::new(self.cmd);
            }
        }
    }

    fn read_input(&mut self) {
        let mut cmd = "".to_owned();
        io::stdout().flush().expect("Error");
        io::stdin()
            .read_line(&mut cmd)
            .expect("Error in reading command, exiting REPL.");
        self.cmd = cmd.trim_end().to_string();
    }
}

impl fmt::Display for REPL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.cmd)
    }
}
