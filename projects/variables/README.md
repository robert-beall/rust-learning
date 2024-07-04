# Variables
## Mutability
Variables are immutable by default, meaning that its value cannot be changed. Adding the `mut` keyword in the variable definition changes this behavior.
### Immutable variable (default behavior)
```
// cannot be changed
let x = 2;
```
### Mutable variable
```
// can be changed
let mut x = 2;
```
## Constants
Constants differ from (immutable) variables in a handful of way: 
* Cannot be made mutable
* Must have their type annotated (specified)
* Must be set to a constant expression, not a result of a value that is computed at runtime. 

The naming convention for constants is to use all capital letters. 
### Declaring a constant
```
/*
* All caps, type specified, and set to a 
* constant expression.
*/
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
Constants are valid in their specified scope for the entire run of a rust program.
## Shadowing
Shadowing allows you to redefine a variable, either performing a transformation on its data or adding a new value. It can even allow you to change the type of the data. Shadowing depends on the scope the shadowing was performed in.
### Basic shadowing example
```
let x = 5; 
let x = x + 1;
println!("{x}"); // outputs "6"
```
### Shadowing in scope
```
let x = 5; 

{
    let x = 6;
    println!("{x}"); // outputs "6"
}

println!("{x}"); // outputs "5"
```
### Changing type with shadowing
```
let x: &str = "5";
let x: u32 = x.parse().expect("failed to parse");
```