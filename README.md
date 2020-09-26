<p align="center"><img src="docs/kvdb.png" width="350px"/></p>

<div align="center">

![GitHub Workflow - Rust](https://github.com/de-sh/kvdb/workflows/Rust/badge.svg)

</div>

KVDB is an experimental KeyValue database, built with rust .

## Components
Currently KVDB lets you store *key-value* pairs and operate it with a *REPL* interface, using speical keywords for operations that are parsed along with the key and associated value positions in the statements by the *parser* before being executed on a *store* object.

Find below an abstract definition of what the code is meant to implement, separating the components into modules named the same.

### REPL
The *read-evalute-print-loop* is a command line program that acts as a front-end, interfacing with a database's back-end. It consists of a prompt, where the user input commands are *read* from and subsequently *evaluated* by the Parser which then executes valid commands, *printing* it's output back to the REPL, continuing to *loop* throughout the lifetime of the program, i.e. until a user decides to terminate an instance.

The REPL allows execution of Meta commands to work with the environment, in this case, one can exit the REPL with the `.exit` command, but a REPL can also be exit using the `CTRL+C` key combination. The meta command `.version` prints the version of KVDB that you are currently using.

> NOTE: Due to various constraints using KVDB through the REPL for now only supports storing the (key->value) pair with the data type (`String`->`String`), even though it is possible to use the Storage engine with other datatypes.

### Parser
The parser is what takes the commands passed to the REPL and converts into sensible data structures that can be used to show or alter the state of our database. Currently we only support the following operations:
1. `GET` - Outputs the value associated with a key, already stored in the database.
    - *Keywords:* _get_, _select_, _output_, _out_, _o_.
    - *Syntax:* `GET <key>`.
2. `SET` - Stores a key-value row where the key isn't already associated with another value in the database.
    - *Keywords:* _set_, _put_, _insert_, _in_, _i_.
    - *Syntax:* `SET <key> <value>`.
3. `DEL` - Deletes a key-value pair from the data store when passed a key, if such a pair exists.
    - *Keywords:* _del_, _delete_, _rem_, _remove_, _rm_, _d_.
    - *Syntax:* `DEL <key>`.

### Storage
Since the database is for experimental purposes and though the idea is to support KeyValue data storage, currently we are using a HashMap for the the Proof of Concept implementation, containing `key` and `value` fields that are used in setting or getting data, the database is entirely in-memory right now. We intend to develop into using a full fledge Log-Structured Merge-Tree based storage engine in the future.

The struct `Store` has been coded to be as generic to key-value data types as possible. The only requirement is that the data type associated with key implements the traits [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html), [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) while value implements `Display` and [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).

## Installation
While this is a very experimental project and we don't intend to create a fully functional application, the program is intended to emulate the actual building of a KV database from scratch.

To install and try out the program:
```bash
git clone https://github.com/de-sh/kvdb && cd kvdb
cargo run
```
## Contribution

Please follow the **rust-lang [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)** when interacting with other contributors on the project's social media. If you are new to the code-base, please contact me on [telegram @DevduttShenoi](https://t.me/DevduttShenoi), I don't intend to start a group right now.