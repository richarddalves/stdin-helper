use std::fmt::Display;
use std::io;
use std::error::Error;

#[derive(Debug)]
pub struct BoolParseError(pub &'static str);

impl Display for BoolParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BoolParseError {}

#[derive(Debug)]
pub enum InputError<E> {
    Io(io::Error),
    Parse(E)
}


impl<E: Error> Error for InputError<E>  {}

impl<E: Display> Display for InputError<E>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Io(e) => write!(f, "I/O Error: {e}"),
            InputError::Parse(e) => write!(f, "Parse Error: {e}")
        }
    }
}

impl<E> From<io::Error> for InputError<E>  {
    fn from(value: io::Error) -> Self {
        InputError::Io(value)
    }
}