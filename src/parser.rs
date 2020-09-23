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

#[derive(PartialEq)]
pub enum StatementType {
    Set,
    Get,
    Unrecognized,
}

impl StatementType {
    fn check(word: &str) -> Self {
        match word.to_lowercase().as_ref() {
            "set" => Self::Set,
            "get" => Self::Get,
            _ => Self::Unrecognized,
        }
    }
}

pub struct Statement {
    pub stype: StatementType,
    pub key: Option<String>, // only used in get/set/rem statements.
    pub value: Option<String>, // only used in an set statement.
}

impl Statement {
    fn new(cmd_words: Vec<&str>) -> Self {
        let stype = StatementType::check(cmd_words[0]);

        Self {
            key: match stype {
                StatementType::Get | StatementType::Set => Some(cmd_words[1].to_string()),
                _ => None,
            },
            value: match stype {
                StatementType::Set => Some(cmd_words[2].to_string()),
                _ => None,
            },
            stype,
        }
    }

    pub fn prep(cmd: &String) -> Self {
        let cmd_words: Vec<&str> = cmd.split(|c| c == ' ' || c == '\t').collect();
        Self::new(cmd_words)
    }
}
