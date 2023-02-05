# Rust Essentials

- Rust deals with low-level Details of Memory Management, Data Representation, and Concurrency

## Environment

- **rustc**: Compiler which takes Rust Code and compiles it into Binary (Machine-readable Code)
- **rustup**: Command Line Utility to install and update Rust
- **cargo**: Rust Build System and Package Manager

## Errors

- Panicking in Rust means Errors during the Runtime

## Struct

- A Struct allow to group up related Data together

## Ownership

- Ownership allows to manage the Memory (by checking for Memory Problems wit the Borrow Checker at Compile Time)
- It results in a fast Runtime and smaller Program Size, since the Garbage Collection does not exist
- It allows Rust to make Memory-safety Guarantees without the Use of Garbage Collector

### Ownership Rules

- Each Value in Rust has a Variable that is called its Owner
- There can only be **one Owner** at a Time
- When the Owner goes out of Scope, the Value will be dropped

### References Rules

- At any given time, either one mutable Reference or any Number of immutable References can be used
- References must always point to valid Data

### Slices

- Slices do not take Ownership of the underlying Data

### Crates

- Packages contain Crates (Libraries) that are created by Cargo
- A Crate contains Modules that allow to organize the Code

### Errors

- The Function Main() can return void or a Result Type: Result<(), Box<dyn Error>>

### Generics

- The Compiler will generate at Compile Time the concrete Implementation of a Generic Method for each used Primitive Type

### Traits

- Traits are like Interfaces in other Programming Languages
- Traits allow to define a Set of Methods that are shared between difference Types

# Cargo Commands

| Command                                                         | Description                                       |
| --------------------------------------------------------------- | ------------------------------------------------- |
| cargo run -- 2 + 4                                              | Run the `main.rs` with Arguments for `calculator` |
| cargo run -- images/github.png images/plus.png image/result.png | Run the `main.rs` with Arguments for `combiner`   |
| cargo build                                                     | Install Crates (Dependencies) and build Program   |
