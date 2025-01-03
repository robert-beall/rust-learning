# Rust Basics Cheatsheet

- [Rust Basics Cheatsheet](#rust-basics-cheatsheet)
  - [Installation](#installation)
    - [Update rust to stable version of Rust](#update-rust-to-stable-version-of-rust)
  - [Basics](#basics)
    - [Compile a rust file](#compile-a-rust-file)
    - [Run compiled rust code](#run-compiled-rust-code)
      - [Linux/MacOS](#linuxmacos)
      - [Windows](#windows)
  - [Cargo](#cargo)
    - [Initialize a project with cargo](#initialize-a-project-with-cargo)
    - [Compile a cargo project](#compile-a-cargo-project)
    - [Build a project for production](#build-a-project-for-production)
    - [Run a cargo project](#run-a-cargo-project)
    - [Check that a project will compile](#check-that-a-project-will-compile)
  - [Variables](#variables)
    - [Define a (constant) variable](#define-a-constant-variable)
    - [Define a (mutable) variable](#define-a-mutable-variable)
  - [Comments](#comments)
    - [Single line comments](#single-line-comments)
    - [Multiline comments](#multiline-comments)
  - [Printing Text](#printing-text)
    - [Basic print macro](#basic-print-macro)
    - [Print with empty placeholder](#print-with-empty-placeholder)
    - [Print with a filled placeholder](#print-with-a-filled-placeholder)


For more information, consult the rust [book](https://doc.rust-lang.org/book).
## Installation
### Update rust to stable version of Rust
```
rustup update
```
## Basics 
### Compile a rust file
```
rustc <filename>.rs
```
### Run compiled rust code
#### Linux/MacOS
```
./<filename>
```
#### Windows
```
.\<filename>.exe
```
## Cargo
### Initialize a project with cargo
```
cargo new <project_name>
```
### Compile a cargo project
```
cargo build
```
### Build a project for production
```
cargo build --release
```
### Run a cargo project 
```
cargo run
```
### Check that a project will compile
```
cargo check
```
## Variables
### Define a (constant) variable
```
let x = 5;
```
### Define a (mutable) variable
```
let mut y = 2;
```
## Comments
### Single line comments
```
// a single line comment
```
### Multiline comments
```
/* A multiline comment */
```
## Printing Text
### Basic print macro
```
println!("my text");
```
### Print with empty placeholder
```
println!("My text: {}", mytext);
```
### Print with a filled placeholder
**Note:** this should be used when printing a result of an operation on one or more variables.
```
println!("My text: {mytext}", mytext);
```