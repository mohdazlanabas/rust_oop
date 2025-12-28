# Rust OOP Concepts Demonstration

A comprehensive demonstration of Object-Oriented Programming (OOP) principles in Rust, showcasing how Rust achieves OOP concepts without traditional class-based inheritance.

## What This Project Demonstrates

- **Abstraction**: Using traits to define behavior contracts
- **Encapsulation**: Private fields with public getters/setters
- **Polymorphism**: Trait objects (dynamic dispatch) and generics (static dispatch)
- **Inheritance**: Composition + default trait methods (Rust's approach)

## Running the Project

```bash
cargo run
```

## Common Error Issue and Fix

### The Error

When initially running this project, you may encounter compilation errors like:

```
error: expected `,`, found `.`
   --> src/main.rs:218:17
    |
218 |     println!("=".repeat(60));
    |                 ^ expected `,`

error: argument never used
   --> src/main.rs:218:18
    |
218 |     println!("=".repeat(60));
    |              --- ^^^^^^^^^^ argument never used
    |              |
    |              formatting specifier missing
```

### Why This Error Occurs

The error happens because `println!` is a **macro**, not a regular function. It expects:
1. A format string as the first argument (like `"{}"`)
2. Values to be formatted as subsequent arguments

When you write:
```rust
println!("=".repeat(60));  // ❌ WRONG
```

The Rust compiler tries to interpret `"=".repeat(60)` as part of the format string syntax, which causes it to fail because:
- It expects a comma after the format string
- The `.repeat()` method is being called inside the macro, which doesn't work as expected
- The macro doesn't recognize `.repeat()` as a valid operation in that context

### The Fix

Always use a format placeholder when printing values that result from method calls:

```rust
println!("{}", "=".repeat(60));  // ✅ CORRECT
```

Here's what happens:
1. `"=".repeat(60)` is evaluated first, creating a string of 60 equal signs
2. That resulting string is then passed as an argument to `println!`
3. The `{}` placeholder in the format string tells `println!` where to insert the value

### All Fixed Lines

The following lines were corrected throughout `src/main.rs`:

```rust
// Line 218, 220
println!("{}", "=".repeat(60));

// Lines 231, 242, 250, 261, 269, 276, 283
println!("{}", "-".repeat(40));

// Lines 290, 292, 297
println!("{}", "=".repeat(60));
```

### Key Takeaway

When using `println!` in Rust:
- Use format placeholders (`{}`, `{:?}`, etc.) for dynamic values
- The first argument must be a string literal format string
- Method calls like `.repeat()`, `.to_string()`, etc., should be arguments, not part of the format string

## Project Structure

```
rust_oop/
├── Cargo.toml          # Project manifest
├── README.md           # This file
└── src/
    └── main.rs         # Main demonstration code
```

## Concepts Demonstrated in Code

### 1. Traits (Abstraction)
```rust
trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> &str;
    fn describe(&self) -> String { /* default implementation */ }
}
```

### 2. Encapsulation
```rust
pub struct Dog {
    name: String,    // Private field
    age: u8,         // Private field
}

impl Dog {
    pub fn get_age(&self) -> u8 { self.age }
    pub fn set_age(&mut self, age: u8) { /* validation */ }
}
```

### 3. Polymorphism
```rust
// Dynamic dispatch with trait objects
let animals: Vec<&dyn Animal> = vec![&dog, &cat, &duck];

// Static dispatch with generics
fn make_sound<T: Animal>(animal: &T) { }
```

### 4. Composition over Inheritance
```rust
struct Horse {
    base: AnimalBase,  // Composition
    speed_mph: u32,
}
```

## License

This is a demonstration project for educational purposes.
