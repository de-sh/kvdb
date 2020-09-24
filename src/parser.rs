/// The parser handles the task of converting commands passed to the repl
/// into something that can be used by it to understand and put into
/// operational use with the storage engine.

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

/// There are 3 types of statement in KVDB, GET/SET/REM.
#[derive(PartialEq)]
pub enum StatementType {
    /// Relates to the set() method of the Storage Engine.
    Set,
    /// Relates to the get() method of the Storage Engine.
    Get,
    /// Relates to the rem() method of the Storage Engine.
    Rem,
    /// The parser has failed to understand what the user 
    /// wants to do, No such operation exists.
    Unrecognized,
}

impl StatementType {
    /// Convert written operation keywords into enum symbols.
    fn check(word: &str) -> Self {
        match word.to_lowercase().as_ref() {
            "set" | "put" | "insert" | "i" => Self::Set,
            "get" | "select" | "o" => Self::Get,
            "rem" | "remove" | "del" | "delete" | "rm" => Self::Rem,
            _ => Self::Unrecognized,
        }
    }
}

/// Describes the structure of a REPL statement.
#[derive(PartialEq)]
pub struct Statement {
    /// Depicts the type of Operation the statement conveys.
    pub stype: StatementType,
    /// The key variable, only used in get/set/rem statements.
    pub key: Option<String>,
    /// The value variable, only used in set statements.
    pub value: Option<String>,
}

impl Statement {
    /// Creates a REPL statement from user input command.
    pub fn prep(cmd: &String) -> Self {
        // Divide user input into words.
        let cmd_words: Vec<&str> = cmd.split(|c| c == ' ' || c == '\t').collect();
        // Find statement type.
        let stype = StatementType::check(cmd_words[0]);
        // Collect rest of the words, if exists, into a single string.
        let cmd_val = match cmd_words.len() > 1 {
            true => cmd_words[2..].to_vec().join(" ").trim().to_string(),
            false => "".to_string(),
        };

        Self {
            key: match stype {
                StatementType::Get | StatementType::Set | StatementType::Rem => {
                    // The first word after the operation keyword is supposed to be
                    // the statement key, else the statement has failed to parse.
                    // TODO: Figure out how to backout of execution and fail.
                    Some(cmd_words[1].to_string())
                }
                _ => None,
            },
            value: match stype {
                StatementType::Set => {
                    // The string after the operation keyword and the statement key
                    // is the statement value. Parsing should fail if no such value
                    // for the `set` operation. Currently, the code sets value to an
                    // empty string value. TODO: Figure out how to backout and fail.
                    Some(cmd_val)
                },
                _ => {
                    if cmd_words.len() > 2 {
                        // Incase the user passes in too much data for an operation, warn them.
                        eprintln!("Warning: Too many inputs, `{}` was ignored.", cmd_val);
                    }
                    None
                }
            },
            stype,
        }
    }
}
