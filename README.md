[![crates.io](https://img.shields.io/crates/v/execute-command.svg)](https://crates.io/crates/execute-command)
[![Documentation](https://docs.rs/execute-command/badge.svg)](https://docs.rs/execute-command)
[![CI](https://github.com/krazijames/execute-command/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/krazijames/execute-command/actions/workflows/ci.yml)

# execute-command

A simple Rust package that wraps `Command` to simplify the execution of programs.

## Usage

Basic functions:

```rust
use execute_command as exec;

let mut command = exec::parse("echo 1").unwrap();
let status = exec::status("echo 1").unwrap();
let output = exec::output("echo 1").unwrap();
let string = exec::string("echo 1").unwrap();
```

Extending `Command`:

```rust
use execute_command::ExecuteCommand;
use std::process::Command;

let mut command = Command::parse("echo 1").unwrap();
let status = Command::parse("echo 1").unwrap().execute_status().unwrap();
let output = Command::parse("echo 1").unwrap().execute_output().unwrap();
let string = Command::parse("echo 1").unwrap().execute_string().unwrap();
```

Note that these functions will return an error if the program exits with a non-zero status code.
