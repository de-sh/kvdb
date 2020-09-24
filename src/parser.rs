pub enum MetaCmdResult {
    Success,
    Unrecognized,
}

impl MetaCmdResult {
    pub fn run(cmd: &String) -> Self {
        match cmd.as_ref() {
            ".exit" => std::process::exit(0),
            _ => Self::Unrecognized,
        }
    }
}

/// There are 3 types of statement in KVDB, GET/SET/REM.
pub enum StatementType {
    Set,
    Get,
    Rem,
    Unrecognized,
}

impl StatementType {
    // Convert written commands into enum symbols.
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
pub struct Statement {
    pub stype: StatementType,
    pub key: Option<String>,   // only used in get/set/rem statements.
    pub value: Option<String>, // only used in an set statement.
}

impl Statement {
    fn new(cmd_words: Vec<&str>) -> Self {
        let stype = StatementType::check(cmd_words[0]);
        let cmd_val = cmd_words[2..].to_vec().join(" ").trim().to_string();

        Self {
            key: match stype {
                StatementType::Get | StatementType::Set | StatementType::Rem => {
                    Some(cmd_words[1].to_string())
                }
                _ => None,
            },
            value: match stype {
                StatementType::Set => Some(cmd_val),
                _ => {
                    if cmd_words.len() > 2 {
                        eprintln!("Warning: Too many inputs, `{}` was ignored.", cmd_val)
                    }
                    None
                }
            },
            stype,
        }
    }

    pub fn prep(cmd: &String) -> Self {
        let cmd_words: Vec<&str> = cmd.split(|c| c == ' ' || c == '\t').collect();
        Self::new(cmd_words)
    }
}
