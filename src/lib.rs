use std::error::Error;
use std::io::{self, Write};
use std::str::FromStr;

mod numeric;

mod erro;
pub use erro::{InputError, BoolParseError};
pub mod aliases;
pub use aliases::*;

pub type InputResult<T> = Result<T, InputError< <T as FromStr>::Err> >;


pub fn get_input<T>(msg: &str, repeat_until_valid: bool) -> Result<T, InputError<T::Err>>
where 
    T: FromStr,
    T::Err: Error
{
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();

        print!("{msg}");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input_buffer)?;
        
        match input_buffer.trim().parse::<T>() {
            Ok(input) => return Ok(input),

            Err(error_msg) => if repeat_until_valid {continue} else { return Err(InputError::Parse(error_msg))}
        }
    }
}