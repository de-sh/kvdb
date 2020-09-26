use std::io;
use std::io::Write;

use crate::parser::{Statement, StatementType};
use crate::store::{ExecResult, Store};

/// The REPL struct is used to hold environment variables relating to the REPL.
pub struct REPL {
    /// User input read from the CLI, in string form.
    cmd: String,
    /// Storage Engine used by the REPL with string based storage.
    store: Store<String, String>,
}

impl REPL {
    /// Create a new instance of the REPL.
    pub fn new() -> Self {
        REPL {
            cmd: "".to_owned(),
            store: Store::new(),
        }
    }

    /// Starts REPL execution in earnest.
    pub fn repl(&mut self) {
        loop {
            // Prompt
            print!("db > ");
            // Read
            self.read_input();
            // Evaluate and Print/Execute
            self.parse_input();
        }
    }

    /// Takes input from the REPL user.
    fn read_input(&mut self) {
        let mut cmd = "".to_owned();
        io::stdout().flush().expect("Error");
        io::stdin()
            .read_line(&mut cmd)
            .expect("Error in reading command, exiting REPL.");
        self.cmd = cmd.trim().to_string();
    }

    /// Parses Commands from the REPL. If Meta, executes on REPL environment,
    /// otherwise executes them on the Storage Engine.
    fn parse_input(&mut self) {
        // Meta commands start with `.`.
        if self.cmd.starts_with(".") {
            match MetaCmdResult::run(&self.cmd) {
                MetaCmdResult::Unrecognized => println!("db: meta command not found: {}", self.cmd),
                MetaCmdResult::Success => {}
            }
        } else {
            let st = Statement::prep(&self.cmd);
            let key = st.key.unwrap_or("".to_string());
            // If type of statement is legit, execute, else fail.
            match match st.stype {
                StatementType::Set => self.store.set(key, st.value.unwrap()),
                StatementType::Get => {
                    match self.store.get(key.clone()) {
                        Ok(res) => {
                            println!("{}", res);
                            ExecResult::Success
                        }
                        Err(ExecResult::Failed) => {
                            // If the key doesn't exist, get() explicitly returns this,
                            // so print the desired Error message.
                            eprintln!("Error: No value associated with key `{}`.", key);
                            ExecResult::Failed
                        }
                        _ => ExecResult::Failed,
                    }
                }
                StatementType::Del => self.store.del(key),
                StatementType::Unk => {
                    eprintln!("db: command not found: {}", self.cmd);
                    ExecResult::Failed
                }
                StatementType::Fail => ExecResult::Failed,
            } {
                ExecResult::Failed => eprintln!("Command Execution Failed."),
                ExecResult::Success => println!("Success: OK"),
            }
        }
    }
}

/// Used in executing meta commands on the REPL.
pub enum MetaCmdResult {
    Success,
    Unrecognized,
}

impl MetaCmdResult {
    /// Execute Meta commands on the REPL.
    pub fn run(cmd: &String) -> Self {
        match cmd.as_ref() {
            ".exit" => std::process::exit(0),
            _ => Self::Unrecognized,
        }
    }
}
