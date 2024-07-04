# Rust Basics Cheatsheet
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