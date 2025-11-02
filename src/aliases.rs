use crate::*;

// =======================================================
//                   String alias
// =======================================================

/// Ask user for a String until a valid String is detected
pub fn get_string(msg: &str) -> String {
    get_input(msg)
}


// =======================================================
//                   Number aliases
// =======================================================

/// Ask user for a number until a valid number is detected
pub fn get_number<T: FromStr>(msg: &str) -> T {
    get_input(msg)
}

/// Gets a signed 8-bit integer (-128 to 127)
pub fn get_i8(msg: &str) -> i8 {
    get_input(msg)
}

/// Gets an unsigned 8-bit integer (0 to 255)
pub fn get_u8(msg: &str) -> u8 {
    get_input(msg)
}

/// Gets a signed 16-bit integer (-32,768 to 32,767)
pub fn get_i16(msg: &str) -> i16 {
    get_input(msg)
}

/// Gets an unsigned 16-bit integer (0 to 65,535)
pub fn get_u16(msg: &str) -> u16 {
    get_input(msg)
}

/// Gets a signed 32-bit integer (-2,147,483,648 to 2,147,483,647)
pub fn get_i32(msg: &str) -> i32 {
    get_input(msg)
}

/// Gets an unsigned 32-bit integer (0 to 4,294,967,295)
pub fn get_u32(msg: &str) -> u32 {
    get_input(msg)
}

/// Gets a signed 64-bit integer
pub fn get_i64(msg: &str) -> i64 {
    get_input(msg)
}

/// Gets an unsigned 64-bit integer
pub fn get_u64(msg: &str) -> u64 {
    get_input(msg)
}

/// Gets a signed 128-bit integer
pub fn get_i128(msg: &str) -> i128 {
    get_input(msg)
}

/// Gets an unsigned 128-bit integer
pub fn get_u128(msg: &str) -> u128 {
    get_input(msg)
}

/// Gets a 32-bit floating point number
pub fn get_f32(msg: &str) -> f32 {
    get_input(msg)
}

/// Gets a 64-bit floating point number
pub fn get_f64(msg: &str) -> f64 {
    get_input(msg)
}

/// Gets a pointer-sized signed integer
pub fn get_isize(msg: &str) -> isize {
    get_input(msg)
}

/// Gets a pointer-sized unsigned integer
pub fn get_usize(msg: &str) -> usize {
    get_input(msg)
}