pub mod parser;
mod repl;
pub mod store;

use repl::REPL;

fn main() {
    // Create and start an instance of the REPL.
    REPL::new().repl();
}
