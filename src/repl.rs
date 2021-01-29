use std::io;
use std::io::{stdin, BufRead, Write};
use tonic::{transport::Channel, Request};

use crate::{
    kvdb_proto::{kvdb_client::KvdbClient, Byte, KeyValue},
    parser::{Statement, StatementType},
    store::ExecResult,
};

/// The REPL struct is used to hold environment variables relating to the REPL.
pub struct REPL {
    /// User input read from the CLI, in string form.
    cmd: String,
    /// Storage Engine used by the REPL with string based storage.
    store: KvdbClient<Channel>,
}

impl REPL {
    /// Create a new instance of the REPL.
    pub async fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(REPL {
            cmd: "".to_owned(),
            store: KvdbClient::connect(addr).await?,
        })
    }

    pub async fn start(addr: String) -> Result<(), Box<dyn std::error::Error>> {
        Self::new(addr).await?.repl().await;

        Ok(())
    }

    /// Starts REPL execution in earnest.
    pub async fn repl(&mut self) {
        // Initial prompt
        print!("KVDBv0.1.0 \nThis is an experimental database, do contribute to further developments at https://github.com/de-sh/kvdb. \nUse `.exit` to exit the repl\ndb > ");
        io::stdout().flush().expect("Error");
        // Read
        for cmd in stdin().lock().lines() {
            match cmd {
                Ok(cmd) => self.cmd = cmd.trim().to_string(),
                Err(_) => print!("Error in reading command, exiting REPL."),
            }
            // Evaluate and Print/Execute
            self.parse_input().await;
            // Prompt
            print!("db > ");
            io::stdout().flush().expect("Error");
        }
    }

    /// Parses Commands from the REPL. If Meta, executes on REPL environment,
    /// otherwise executes them on the Storage Engine.
    async fn parse_input(&mut self) {
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
                StatementType::Set => match self
                    .store
                    .set(Request::new(KeyValue {
                        key: key.as_bytes().to_vec(),
                        value: st.value.unwrap().as_bytes().to_vec(),
                    }))
                    .await
                {
                    Ok(_) => ExecResult::Success,
                    Err(e) => {
                        eprintln!("{}", e.message());
                        ExecResult::Failed
                    }
                },
                StatementType::Get => match self
                    .store
                    .get(Request::new(Byte {
                        body: key.as_bytes().to_vec(),
                    }))
                    .await
                {
                    Ok(res) => {
                        println!("{}", String::from_utf8(res.into_inner().body).unwrap());
                        ExecResult::Success
                    }
                    Err(e) => {
                        // If the key doesn't exist, get() explicitly returns this,
                        // so print the desired Error message.
                        eprintln!("{}", e.message());
                        ExecResult::Failed
                    }
                },
                StatementType::Del => match self
                    .store
                    .del(Request::new(Byte {
                        body: key.as_bytes().to_vec(),
                    }))
                    .await
                {
                    Ok(_) => ExecResult::Success,
                    Err(e) => {
                        eprintln!("{}", e.message());
                        ExecResult::Failed
                    }
                },
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
            ".version" => {
                if let Some(ver) = option_env!("CARGO_PKG_VERSION") {
                    println!("You are using KVDB v{}", ver);
                }
                Self::Success
            }
            _ => Self::Unrecognized,
        }
    }
}
