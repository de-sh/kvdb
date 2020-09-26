/// The parser handles the task of converting commands passed to the repl
/// into something that can be used by it to understand and put into
/// operational use with the storage engine.
pub mod parser;

/// Code pertaining to environment management of the REPL.
/// Provides an interface to operate on the Storage Engine.
mod repl;

/// Code related to the heart of a database, the storage engine.
pub mod store;

use repl::REPL;

/// Create and start an instance of the REPL.
fn main() {
    REPL::new().repl();
}
