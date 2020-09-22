# DB rust
An experimental database inspired from SQLite, built in rust.

### REPL
The *read-evalute-print-loop* is a command line program that acts as a front-end, interfacing with a database's back-end. It consists of a prompt, where the user input commands are *read* from and subsequently *evaluated* by the Parser which then executes valid commands, *printing* it's output back to the REPL, continuing to *loop* throughout the lifetime of the program, i.e. until a user decides to terminate an instance, in this case, either with the `.exit` command or `CTRL+C` key combination.