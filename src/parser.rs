pub enum MetaCmdResult {
    Success,
    Unrecognized
}

impl MetaCmdResult {
    pub fn new(cmd: String) -> Self {
        match cmd.as_ref() {
            ".exit" => std::process::exit(0),
            cmd => println!("db: command not found: {}", cmd),
        }
    }
}
  
pub enum PrepResult { 
    Success,
    Unrecognized 
}

impl PrepResult {
    pub fn new(cmd: String) -> Self {
        match cmd.as_ref() {
            ".exit" => std::process::exit(0),
            cmd => println!("db: command not found: {}", cmd),
        }
    }
}