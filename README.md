# Rust Basics Cheatsheet
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