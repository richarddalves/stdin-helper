# stdin-helper

![Status](https://img.shields.io/badge/status-stable-green)
![Version](https://img.shields.io/badge/version-2.0.1-blue)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-lightgrey)

A simple, intuitive Rust library for reading typed user input from stdin. Stop fighting with `read_line()` and parsing - just ask for what you need.

## What is this?

`stdin-helper` provides straightforward functions for getting typed input from the user. If you've ever been frustrated writing the same `read_line()` boilerplate over and over, this library is for you.

Every function accepts a `repeat` flag: when set to `true`, it automatically retries on invalid input. Ask for an integer and the user types "hello"? It will ask again until it gets a valid number. When set to `false`, it returns a `Result` immediately, giving you full control over error handling.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
stdin-helper = "2.0.1"
```

Or use cargo:

```bash
cargo add stdin-helper
```

## Basic Usage

All functions return `InputResult<T>`, which is an alias for `Result<T, InputError<...>>`. Use `repeat: true` to loop until valid input is received, or `repeat: false` to get a `Result` back immediately.

```rust
use stdin_helper::{get_string, get_i32, get_f64, get_bool};

fn main() {
    // repeat: true — keeps asking until the user enters something valid
    let name = get_string("What's your name? ", true).unwrap();
    let age = get_i32("How old are you? ", true).unwrap();
    let height = get_f64("Height in meters: ", true).unwrap();
    let verified = get_bool("Are you human? ", true, &["yes", "y", "true"], &["no", "n", "false"]).unwrap();

    println!("{} is {} years old and {:.2}m tall", name, age, height);
}
```

When using `repeat: false`, you can propagate or handle errors idiomatically:

```rust
use stdin_helper::get_i32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let age = get_i32("How old are you? ", false)?;
    println!("You are {age} years old.");
    Ok(())
}
```

## Error Handling

`stdin-helper` distinguishes between two kinds of failures via `InputError<E>`:

- `InputError::Io(io::Error)` — something went wrong reading from stdin (e.g. stdin was closed).
- `InputError::Parse(E)` — the user's input couldn't be converted to the requested type.

This distinction matters because I/O errors are generally unrecoverable, while parse errors are expected and safe to retry. When `repeat: true` is used, parse errors are silently retried; I/O errors are still returned immediately regardless of the flag.

```rust
use stdin_helper::{get_i32, InputError};

match get_i32("Enter a number: ", false) {
    Ok(n) => println!("Got: {n}"),
    Err(InputError::Io(e)) => eprintln!("I/O error: {e}"),
    Err(InputError::Parse(e)) => eprintln!("Invalid input: {e}"),
}
```

## Available Functions

### Text Input

| Function | Returns | Description |
| --- | --- | --- |
| `get_string(prompt, repeat)` | `InputResult<String>` | Gets text input (any text is valid) |
| `get_char(prompt, repeat)` | `InputResult<char>` | Gets a single character |

### Integer Types

| Function | Returns | Range |
| --- | --- | --- |
| `get_i8(prompt, repeat)` | `InputResult<i8>` | -128 to 127 |
| `get_i16(prompt, repeat)` | `InputResult<i16>` | -32,768 to 32,767 |
| `get_i32(prompt, repeat)` | `InputResult<i32>` | -2,147,483,648 to 2,147,483,647 |
| `get_i64(prompt, repeat)` | `InputResult<i64>` | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 |
| `get_i128(prompt, repeat)` | `InputResult<i128>` | -(2^127) to 2^127 - 1 |
| `get_isize(prompt, repeat)` | `InputResult<isize>` | Platform-dependent (32 or 64 bits) |

### Unsigned Integer Types

| Function | Returns | Range |
| --- | --- | --- |
| `get_u8(prompt, repeat)` | `InputResult<u8>` | 0 to 255 |
| `get_u16(prompt, repeat)` | `InputResult<u16>` | 0 to 65,535 |
| `get_u32(prompt, repeat)` | `InputResult<u32>` | 0 to 4,294,967,295 |
| `get_u64(prompt, repeat)` | `InputResult<u64>` | 0 to 18,446,744,073,709,551,615 |
| `get_u128(prompt, repeat)` | `InputResult<u128>` | 0 to 2^128 - 1 |
| `get_usize(prompt, repeat)` | `InputResult<usize>` | Platform-dependent (32 or 64 bits) |

### Floating Point Types

| Function | Returns | Description |
| --- | --- | --- |
| `get_f32(prompt, repeat)` | `InputResult<f32>` | 32-bit floating point |
| `get_f64(prompt, repeat)` | `InputResult<f64>` | 64-bit floating point (recommended) |

### Boolean Input

Boolean input requires you to explicitly define which strings count as `true` and which count as `false`. Comparison is case-insensitive after `.trim()`.

```rust
// Signature
pub fn get_bool(
    msg: &str,
    repeat: bool,
    true_values: &[&str],
    false_values: &[&str],
) -> Result<bool, InputError<BoolParseError>>
```

```rust
let confirmed = get_bool(
    "Confirm? ",
    true,
    &["yes", "y", "sim", "s", "true", "1"],
    &["no", "n", "nao", "false", "0"],
).unwrap();
```

### Generic Input

If you need a type not listed above, use the core generic function directly. Any type that implements `FromStr` works:

```rust
use stdin_helper::get_input;

let value: u64 = get_input("Enter a value: ", true).unwrap();
```

For numeric types specifically, `get_number<T>` is also available with a `Numeric` trait bound that prevents accidental misuse with non-numeric types:

```rust
use stdin_helper::get_number;

let value: f32 = get_number("Enter a value: ", true).unwrap();
```

## Examples

### Simple Calculator

```rust
use stdin_helper::{get_i32, get_string};

fn main() {
    let x = get_i32("First number: ", true).unwrap();
    let op = get_string("Operator (+, -, *, /): ", true).unwrap();
    let y = get_i32("Second number: ", true).unwrap();

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
    let temp = get_f64("Enter temperature: ", true).unwrap();
    let unit = get_string("Unit (C/F): ", true).unwrap().to_uppercase();

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
    let num_grades = get_i32("How many grades? ", true).unwrap();
    let mut sum = 0.0;

    for i in 1..=num_grades {
        let grade = get_f32(&format!("Grade {}: ", i), true).unwrap();
        sum += grade;
    }

    let average = sum / num_grades as f32;
    println!("Average: {:.2}", average);
}
```

### Propagating Errors

```rust
use stdin_helper::get_i32;

fn read_positive() -> Result<i32, Box<dyn std::error::Error>> {
    // repeat: false gives us the Result immediately so we can propagate
    let n = get_i32("Enter a number: ", false)?;
    Ok(n)
}
```

## How It Works

Each function displays your prompt and waits for a line of input. What happens next depends on the `repeat` flag:

- `repeat: true` — if the input can't be parsed into the requested type, the prompt is displayed again. This continues until valid input is received or an I/O error occurs.
- `repeat: false` — tries once and returns a `Result`. A parse failure becomes `Err(InputError::Parse(...))`, which you can handle or propagate with `?`.

I/O errors (`InputError::Io`) always surface immediately regardless of the `repeat` flag, since they signal a problem with stdin itself rather than with the user's input.

## Future Plans

In upcoming versions, `stdin-helper` will support a builder pattern API for more advanced input validation scenarios:

```rust
// THIS DOES NOT WORK YET - Coming in a future version!
use stdin_helper::InputBuilder;

let user_rating: u32 = InputBuilder::new("Please input a rate: ")
    .range(1..=10, "Number must be between 1 and 10")
    .on_error("That doesn't look like a number")
    .get()?;
```

This will allow you to:

- Define custom validation rules
- Provide specific error messages for different validation failures
- Set retry limits
- Add custom validators with closures

If you'd like to contribute to making this happen, check out the repository!

## Building and Testing

```bash
# Build the library
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Why Use This?

**For beginners:** Eliminates boilerplate so you can focus on learning Rust concepts rather than input handling mechanics. Use `repeat: true` and `.unwrap()` to get started fast.

**For prototyping:** Quickly test ideas without writing parsing code every time.

**For simple CLIs:** Perfect for straightforward command-line tools where complex error handling isn't needed.

**For teaching:** Great for educational projects and coding exercises where input handling shouldn't be the focus.

## When Not to Use This

This library is designed for simplicity, not for complex input scenarios. Consider alternatives if you need:

- Non-blocking input handling
- Input with timeout limits
- Production-grade validation with custom retry limits or closures (coming in a future version)

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