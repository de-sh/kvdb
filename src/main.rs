/// The parser handles the task of converting commands passed to the repl
/// into something that can be used by it to understand and put into
/// operational use with the storage engine.
pub mod parser;

/// Interfaces to operate on the Storage Engine.
/// 1. code pertaining to environment management of a REPL.
mod repl;
/// 2. code pertaining to environment management of a network server.
mod server;

/// Code related to the heart of a database, the storage engine.
pub mod store;

/// Defines an experimental LSMT based persistant storage API.
pub mod lsmt;

/// Defines available persistant storage configurations.
pub mod config;

use repl::REPL;

/// Create and start an instance of the REPL.
#[tokio::main]
async fn main() {
    match std::env::args().nth(1) {
        Some(s) => {
            if s == ("server".to_string()) {
                server::serve(match std::env::args().nth(2) {
                    Some(addr) => addr,
                    None => "127.0.0.1:6379".to_string(),
                })
                .await;
            }
        }
        None => REPL::new().repl(),
    }
}
