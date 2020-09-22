pub mod parser;
mod repl;
use repl::REPL;

fn main() {
    REPL::new().repl();
}
