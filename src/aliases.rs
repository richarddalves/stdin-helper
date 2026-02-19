use std::{error::Error, str::FromStr};

use super::numeric::Numeric;

use super::{get_input, InputError, InputResult, BoolParseError};

// =======================================================
//                     Text Aliases
// =======================================================

pub fn get_string(msg: &str, repeat: bool) -> InputResult<String> {
    get_input(msg, repeat)
}

pub fn get_char(msg: &str, repeat: bool) -> InputResult<char> {
    get_input(msg, repeat)
}

// =======================================================
//                     Number Aliases
// =======================================================

pub fn get_number<T>(msg: &str, repeat: bool) -> InputResult<T>
where
    T: FromStr + Numeric,
    T::Err: Error
{
    get_input(msg, repeat)
}

pub fn get_isize(msg: &str, repeat: bool) -> InputResult<isize> {
    get_input(msg, repeat)
}

pub fn get_i128(msg: &str, repeat: bool) -> InputResult<i128> {
    get_input(msg, repeat)
}
pub fn get_i64(msg: &str, repeat: bool) -> InputResult<i64> {
    get_input(msg, repeat)
}
pub fn get_i32(msg: &str, repeat: bool) -> InputResult<i32> {
    get_input(msg, repeat)
}
pub fn get_i16(msg: &str, repeat: bool) -> InputResult<i16> {
    get_input(msg, repeat)
}
pub fn get_i8(msg: &str, repeat: bool) -> InputResult<i8> {
    get_input(msg, repeat)
}

pub fn get_usize(msg: &str, repeat: bool) -> InputResult<usize> {
    get_input(msg, repeat)
}

pub fn get_u128(msg: &str, repeat: bool) -> InputResult<u128> {
    get_input(msg, repeat)
}
pub fn get_u64(msg: &str, repeat: bool) -> InputResult<u64> {
    get_input(msg, repeat)
}
pub fn get_u32(msg: &str, repeat: bool) -> InputResult<u32> {
    get_input(msg, repeat)
}
pub fn get_u16(msg: &str, repeat: bool) -> InputResult<u16> {
    get_input(msg, repeat)
}
pub fn get_u8(msg: &str, repeat: bool) -> InputResult<u8> {
    get_input(msg, repeat)
}

pub fn get_f64(msg: &str, repeat: bool) -> InputResult<f64> {
    get_input(msg, repeat)
}
pub fn get_f32(msg: &str, repeat: bool) -> InputResult<f32> {
    get_input(msg, repeat)
}

// =======================================================
//                     Boolean Alias
// =======================================================

pub fn get_bool(msg: &str, repeat: bool, true_values: &[&str], false_values: &[&str]) -> Result<bool, InputError<BoolParseError>> {
    loop {
        let input: String = match get_input(msg, repeat) {
            Ok(valor) => valor,

            Err(InputError::Io(io_err)) => return Err(InputError::Io(io_err)),

            Err(InputError::Parse(_)) => return Err(InputError::Parse(BoolParseError("A strange error occurred. This error should be impossible")))
        };

        let input = input.trim().to_lowercase();
        
        if true_values.contains(&input.as_str()) {
            return Ok(true)
        } else if false_values.contains(&input.as_str()){
            return Ok(false)
        } else {
            if repeat {
                continue;
            } else {
                return Err(InputError::Parse(BoolParseError("Invalid Value")))
            }
        }
    }
}