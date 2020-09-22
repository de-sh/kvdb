mod repl;
pub mod parser;
use repl::REPL;

fn main() {
    REPL::new().repl();
}
