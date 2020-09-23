pub mod parser;
mod repl;
pub mod store;

use repl::REPL;

fn main() {
    REPL::new().repl();
}
