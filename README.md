# stdin-helper

![Status](https://img.shields.io/badge/status-stable-green)
![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-lightgrey)

A simple, intuitive Rust library for reading typed user input from stdin. Stop fighting with `read_line()` and parsing - just ask for what you need.

## What is this?

`stdin-helper` provides straightforward functions for getting typed input from the user. If you've ever been frustrated writing the same `read_line()` boilerplate over and over, this library is for you.

The functions automatically retry when given invalid input. Ask for an integer and the user types "hello"? It will ask again until it gets a valid number.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
stdin-helper = "1.0.0"
```

Or use cargo:

```bash
cargo add stdin-helper
```

## Basic Usage

```rust
use stdin_helper::{get_string, get_i32, get_f64, get_bool};

fn main() {
    let name = get_string("What's your name? ");
    let age = get_i32("How old are you? ");
    let height = get_f64("Height in meters: ");
    let verified = get_bool("Are you human? ");

    println!("{} is {} years old and {:.2}m tall", name, age, height);
}
```

## Available Functions

### String Input

| Function             | Returns  | Description                         |
| -------------------- | -------- | ----------------------------------- |
| `get_string(prompt)` | `String` | Gets text input (any text is valid) |

### Integer Types

| Function            | Returns | Range                                                   |
| ------------------- | ------- | ------------------------------------------------------- |
| `get_i8(prompt)`    | `i8`    | -128 to 127                                             |
| `get_i16(prompt)`   | `i16`   | -32,768 to 32,767                                       |
| `get_i32(prompt)`   | `i32`   | -2,147,483,648 to 2,147,483,647                         |
| `get_i64(prompt)`   | `i64`   | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 |
| `get_i128(prompt)`  | `i128`  | -(2^127) to 2^127 - 1                                   |
| `get_isize(prompt)` | `isize` | Platform-dependent (32 or 64 bits)                      |

### Unsigned Integer Types

| Function            | Returns | Range                              |
| ------------------- | ------- | ---------------------------------- |
| `get_u8(prompt)`    | `u8`    | 0 to 255                           |
| `get_u16(prompt)`   | `u16`   | 0 to 65,535                        |
| `get_u32(prompt)`   | `u32`   | 0 to 4,294,967,295                 |
| `get_u64(prompt)`   | `u64`   | 0 to 18,446,744,073,709,551,615    |
| `get_u128(prompt)`  | `u128`  | 0 to 2^128 - 1                     |
| `get_usize(prompt)` | `usize` | Platform-dependent (32 or 64 bits) |

### Floating Point Types

| Function          | Returns | Description                         |
| ----------------- | ------- | ----------------------------------- |
| `get_f32(prompt)` | `f32`   | 32-bit floating point               |
| `get_f64(prompt)` | `f64`   | 64-bit floating point (recommended) |

### Boolean Input

| Function           | Returns | Accepts                                                          |
| ------------------ | ------- | ---------------------------------------------------------------- |
| `get_bool(prompt)` | `bool`  | true, false, 1, 0, yes, no, y, n, sim, s, nao (case insensitive) |

### Generic Input

If you need a type not listed above, use the generic function:

```rust
use stdin_helper::get_input;

let value: u64 = get_input("Enter a value: ");
```

## Examples

### Simple Calculator

```rust
use stdin_helper::{get_i32, get_string};

fn main() {
    let x = get_i32("First number: ");
    let op = get_string("Operator (+, -, *, /): ");
    let y = get_i32("Second number: ");

    let result = match op.as_str() {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("{} {} {} = {}", x, op, y, result);
}
```

### Temperature Converter

```rust
use stdin_helper::{get_f64, get_string};

fn main() {
    let temp = get_f64("Enter temperature: ");
    let unit = get_string("Unit (C/F): ").to_uppercase();

    match unit.as_str() {
        "C" => {
            let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
            println!("{}°C = {:.1}°F", temp, fahrenheit);
        }
        "F" => {
            let celsius = (temp - 32.0) * 5.0 / 9.0;
            println!("{}°F = {:.1}°C", temp, celsius);
        }
        _ => println!("Invalid unit. Use C or F."),
    }
}
```

### Grade Calculator

```rust
use stdin_helper::{get_f32, get_i32};

fn main() {
    let num_grades = get_i32("How many grades? ");
    let mut sum = 0.0;

    for i in 1..=num_grades {
        let grade = get_f32(&format!("Grade {}: ", i));
        sum += grade;
    }

    let average = sum / num_grades as f32;
    println!("Average: {:.2}", average);
}
```

## How It Works

Each function displays your prompt and waits for input. If the input can't be parsed into the requested type, the prompt appears again. This continues until valid input is received.

This approach is perfect for learning, prototyping, and simple CLI tools. For production applications where you need fine-grained error handling, you might want more control over the retry logic.

## Future Plans

**Note:** The features described below are planned for future releases and are **not yet implemented**.

In upcoming versions, stdin-helper will support a builder pattern API for advanced input validation:

```rust
// THIS DOES NOT WORK YET - Coming in a future version!
use stdin_helper::get_input;

let user_rating: u32 = get_input()
    .prompt("Please input a rate: ")
    .range(1..=10, "Number must be between 1 and 10")
    .on_error("That doesn't look like a number")
    .expect("Failed to read input");
```

This will allow you to:

- Define custom validation rules
- Provide specific error messages for different validation failures
- Choose whether to retry automatically or return a Result
- Set retry limits
- Add custom validators with closures

If you'd like to contribute to making this happen, check out the repository!

## Building and Testing

```bash
# Build the library
cargo build

# Run tests (coming soon)
cargo test

# Generate documentation
cargo doc --open
```

## Why Use This?

**For beginners:** Eliminates boilerplate so you can focus on learning Rust concepts rather than input handling mechanics.

**For prototyping:** Quickly test ideas without writing parsing code every time.

**For simple CLIs:** Perfect for straightforward command-line tools where complex error handling isn't needed.

**For teaching:** Great for educational projects and coding exercises where input handling shouldn't be the focus.

## When Not to Use This

This library is designed for simplicity, not for complex input scenarios. Consider alternatives if you need:

- Detailed error messages showing what went wrong
- Non-blocking input handling
- Input with timeout limits
- Advanced parsing with custom error types
- Production-grade validation with specific error recovery

For those cases, using `std::io` directly or other crates might be better choices.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Whether it's:

- Bug reports and feature requests
- Documentation improvements
- Code contributions
- Examples and use cases

Feel free to open an issue or submit a pull request.

## Author

Created by [Richard Dias Alves](https://github.com/richarddalves)

## Acknowledgments

Inspired by the frustration of writing `read_line()` boilerplate repeatedly. Built to make Rust's stdin interaction as simple as it should be.

---

**Repository:** https://github.com/richarddalves/stdin-helper  
**Crate:** https://crates.io/crates/stdin-helper
