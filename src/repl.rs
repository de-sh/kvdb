use std::fmt;
use std::io;
use std::io::Write;

#[derive(Clone)]
pub struct REPL {
    cmd: String,
}

impl REPL {
    pub fn new() -> Self {
        REPL { cmd: "".to_owned() }
    }

    pub fn repl(&mut self) {
        loop {
            print!("db > "); // Prompt
            self.read_input();
            self.parse();
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

    fn parse(&mut self) {
        match self.cmd.as_ref() {
            ".exit" => std::process::exit(0),
            cmd => println!("db: command not found: {}", cmd),
        }
    }
}

impl fmt::Display for REPL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.cmd)
    }
}
