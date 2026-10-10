# Rust Hex Dump Utility

A simple command-line hex dump utility written in Rust that reads a file and displays its contents as hexadecimal byte values.

## Features

- Reads file contents as raw bytes
- Converts bytes to hexadecimal (uppercase)
- Writes the output to `output.txt`

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo to build the project

## Usage

Run the program from the project directory:

```bash
cargo run -- test.txt
```

Replace `test.txt` with the path to the file you want to inspect.

## Building

To compile an optimized executable:

```bash
cargo build --release
```

The executable will be located in `target/release/`.
