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

// Fixed schema for insert
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Row {
    pub fn new(inst: Vec<&str>) -> Self {
        Self {
            id: match inst[0].parse() {
                Err(e) => {
                    eprintln!("Error: {}, Schema requires an integer. Defaulting to 0.", e);
                    0
                }
                Ok(r) => r,
            },
            username: inst[1].to_string(),
            email: inst[2].to_string(),
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
    pub stype: StatementType,
    pub row_to_insert: Option<Row>, // only used in an insert statement
}

impl Statement {
    fn new(cmd_words: Vec<&str>) -> Self {
        let stype = StatementType::get(cmd_words[0]);
        let row_to_insert = match stype {
            StatementType::Insert => Some(Row::new(cmd_words[1..].to_vec())),
            _ => None,
        };
        
        Self {
            stype,
            row_to_insert,
        }
    }

    pub fn prep(cmd: &String) -> Self {
        let cmd_words: Vec<&str> = cmd.split(|c| c == ' ' || c == '\t').collect();
        Self::new(cmd_words)
    }
}
