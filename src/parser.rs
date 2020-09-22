pub enum MetaCmdResult {
    Success,
    Unrecognized,
}

impl MetaCmdResult {
    pub fn run(cmd: &String) -> Self {
        match cmd.as_ref() {
            ".exit" => std::process::exit(0),
            cmd => Self::Unrecognized,
        }
    }
}

#[derive(PartialEq)]
pub enum StatementType {
    Insert,
    Select,
    Unrecognized,
}

impl StatementType {
    fn get(word: &str) -> Self {
        match word {
            "insert" => Self::Insert,
            "select" => Self::Select,
            _ => Self::Unrecognized,
        }
    }
}

pub struct Statement {
    stype: StatementType,
}

impl Statement {
    fn new(cmd_words: Vec<&str>) -> Self {
        Self { stype: StatementType::get(cmd_words[0]) }
    }

    pub fn prep(cmd: &String) -> Self {
        let cmd_words: Vec<&str> = cmd.split(|c| c == ' ' || c == '\t').collect();
        Self::new(cmd_words)
    }

    pub fn is_unrec(self) -> bool {
        self.stype == StatementType::Unrecognized
    }
}
