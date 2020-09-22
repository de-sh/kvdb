pub struct Parser {
    
}

impl Parser {
    pub fn parse(cmd: &String) {
        match cmd.as_ref() {
            ".exit" => std::process::exit(0),
            cmd => println!("db: command not found: {}", cmd),
        }
    }
}