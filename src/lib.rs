//! Simplify read_line method

pub mod aliases;

use std::io::{self, Write};
use std::str::FromStr;

pub use aliases::*;

/// ## Core function
/// 
/// Ask user for a input until a valid input is detected
pub fn get_input<T: FromStr>(msg: &str) -> T {
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();

        print!("{msg}");
        io::stdout().flush().expect("Error on flush");

        io::stdin().read_line(&mut input_buffer).expect("Error reading line");

        match input_buffer.trim().parse::<T>() {
            Ok(value) => return value,

            _ => continue
        }
    }
}

/// Ask user for a boolean until a valid boolean is detected
pub fn get_bool(msg: &str) -> bool {
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();

        print!("{msg}");
        io::stdout().flush().expect("Error on flush");

        io::stdin().read_line(&mut input_buffer).expect("Error reading line");

        match input_buffer.trim().to_lowercase().as_str() {
            "y" | "yes" | "s" |"sim" | "true" | "1" => return true,
            "n" | "no" | "not" |"nao" | "false" | "0" => return false,

            _ => continue
        }
    }
}