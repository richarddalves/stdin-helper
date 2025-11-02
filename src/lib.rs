//! Simplify read_line method

use std::{io::{self, Write}, str::FromStr};

/// Ask user for a number until a valid number is detected
pub fn get_number<T: FromStr>(msg: &str) -> T {
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