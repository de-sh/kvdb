# KVDB rust
An experimental KeyValue database, built with rust.

### REPL
The *read-evalute-print-loop* is a command line program that acts as a front-end, interfacing with a database's back-end. It consists of a prompt, where the user input commands are *read* from and subsequently *evaluated* by the Parser which then executes valid commands, *printing* it's output back to the REPL, continuing to *loop* throughout the lifetime of the program, i.e. until a user decides to terminate an instance, in this case, either with the `.exit` command or `CTRL+C` key combination.

### Parser
The parser is what takes the commands passed to the REPL and converts into sensible data structures that can be used to show or alter the state of our database. Currently we only support the following commands:
1. `GET` - Does nothing //TODO: Add Selection
2. `SET` - Adds a key-value row to the designated Store, a HashMap based in-memory storage engine that we intend to develop into a full fledge Log-Structured Merge-Tree based storage scheme.

### Storage
Since the database is for experimental purposes and though the idea is to support KeyValue data storage, currently we are using a HashMap for the the Proof of Concept implementation, containing `key` and `value` fields that are used in setting or getting data, the database is entirely in-memory at.

## Installation
While this is a very experimental project and we don't intend to create a fully functional application, the program is intended to emulate the actual building of a KV database from scratch.

To install and try out the program:
```bash
git clone https://github.com/de-sh/kvdb && cd kvdb
cargo run
```